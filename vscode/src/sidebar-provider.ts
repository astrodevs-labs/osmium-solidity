import { Address } from "viem";
import * as vscode from "vscode";
import { window } from "vscode";
import { ContractRepository } from "./actions/ContractRepository";
import { Interact } from "./actions/Interact";
import { WalletRepository } from "./actions/WalletRepository";
import {
  DeployContracts,
  DeployScriptArgs,
  DeployScript,
  RpcUrl,
} from "./actions/types";
import {
  deployContract,
  deployScript,
  getContracts,
  getScripts,
} from "./actions/deploy";
import { EnvironmentRepository } from "./actions/EnvironmentRepository";
import { getNonce } from "./utils";

enum MessageType {
  GET_WALLETS = "GET_WALLETS",
  WALLETS = "WALLETS",
  GET_INTERACT_CONTRACTS = "GET_INTERACT_CONTRACTS",
  INTERACT_CONTRACTS = "INTERACT_CONTRACTS",
  GET_DEPLOY_CONTRACTS = "GET_DEPLOY_CONTRACTS",
  DEPLOY_CONTRACTS = "DEPLOY_CONTRACTS",
  WRITE = "WRITE",
  WRITE_RESPONSE = "WRITE_RESPONSE",
  READ = "READ",
  GET_SCRIPTS = "GET_SCRIPTS",
  SCRIPTS = "SCRIPTS",
  GET_ENVIRONMENTS = "GET_ENVIRONMENTS",
  ENVIRONMENTS = "ENVIRONMENTS",
  READ_RESPONSE = "READ_RESPONSE",
  EDIT_WALLETS = "EDIT_WALLETS",
  EDIT_CONTRACTS = "EDIT_CONTRACTS",
  EDIT_ENVIRONMENT = "EDIT_ENVIRONMENT",
  DEPLOY_SCRIPT = "DEPLOY_SCRIPT",
  DEPLOY_SCRIPT_RESPONSE = "DEPLOY_SCRIPT_RESPONSE",
  DEPLOY_CONTRACT = "DEPLOY_CONTRACT",
  DEPLOY_CONTRACT_RESPONSE = "DEPLOY_CONTRACT_RESPONSE",
}

type Message = {
  type: MessageType;
  data: any;
};

enum InputAction {
  ADD = "Add",
  REMOVE = "Remove",
}

export class SidebarProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = "osmium.sidebar";

  private _view?: vscode.WebviewView;

  private _contractRepository?: ContractRepository;
  private _walletRepository?: WalletRepository;
  private _environmentRepository?: EnvironmentRepository;
  private _interact?: Interact;
  private _scripts?: DeployScript[];
  private _contracts?: DeployContracts[];

  private _watcher?: vscode.FileSystemWatcher;

  constructor(private readonly _extensionUri: vscode.Uri) {}

  public async resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken,
  ) {
    this._view = webviewView;

    if (vscode.workspace.workspaceFolders?.length) {
      const fsPath = vscode.workspace.workspaceFolders?.[0].uri.fsPath;
      this._contractRepository = new ContractRepository(fsPath);
      this._walletRepository = new WalletRepository(fsPath);
      this._environmentRepository = new EnvironmentRepository(fsPath);

      this._interact = new Interact(
        this._contractRepository,
        this._walletRepository,
      );

      this._scripts = await getScripts();
      this._contracts = await getContracts();

      const pattern = new vscode.RelativePattern(fsPath, ".osmium/*.json");
      this._watcher = vscode.workspace.createFileSystemWatcher(pattern);

      this._watcher.onDidChange(async (uri) => {
        if (!this._view) {
          return;
        }
        if (uri.fsPath.endsWith("contracts.json")) {
          this._contractRepository?.load();
          await this._view.webview.postMessage({
            type: MessageType.INTERACT_CONTRACTS,
            contracts: this._contractRepository?.getContracts(),
          });
        }
        if (uri.fsPath.endsWith("wallets.json")) {
          this._walletRepository?.load();
          await this._view.webview.postMessage({
            type: MessageType.WALLETS,
            wallets: this._walletRepository?.getWallets(),
          });
        }
        if (uri.fsPath.endsWith("environments.json")) {
          this._environmentRepository?.load();
          await this._view.webview.postMessage({
            type: MessageType.ENVIRONMENTS,
            environments: this._environmentRepository?.getEnvironments(),
          });
        }
      });
    }

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this._extensionUri],
    };

    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

    webviewView.webview.onDidReceiveMessage(async (message: Message) => {
      if (
        !this._view ||
        !this._contractRepository ||
        !this._walletRepository ||
        !this._environmentRepository ||
        !this._interact ||
        !this._scripts ||
        !this._contracts
      ) {
        return;
      }
      switch (message.type) {
        case MessageType.GET_WALLETS:
          await this._view.webview.postMessage({
            type: MessageType.WALLETS,
            wallets: this._walletRepository.getWallets(),
          });
          break;
        case MessageType.GET_INTERACT_CONTRACTS:
          await this._view.webview.postMessage({
            type: MessageType.INTERACT_CONTRACTS,
            contracts: this._contractRepository.getContracts(),
          });
          break;
        case MessageType.GET_SCRIPTS:
          await this._view.webview.postMessage({
            type: MessageType.SCRIPTS,
            scripts: this._scripts,
          });
          break;
        case MessageType.GET_DEPLOY_CONTRACTS:
          await this._view.webview.postMessage({
            type: MessageType.DEPLOY_CONTRACTS,
            contracts: this._contracts,
          });
          break;
        case MessageType.GET_ENVIRONMENTS:
          await this._view.webview.postMessage({
            type: MessageType.ENVIRONMENTS,
            environments: this._environmentRepository.getEnvironments(),
          });
          break;
        case MessageType.WRITE:
          let value = BigInt(message.data.value);

          if (message.data.valueUnit === "ether") {
            value = value * BigInt(10) ** BigInt(18);
          } else if (message.data.valueUnit === "gwei") {
            value = value * BigInt(10) ** BigInt(9);
          }

          const writeResponse = await this._interact.writeContract({
            account: message.data.wallet,
            address: message.data.contract,
            abi: this._contractRepository.getContract(message.data.contract)!
              .abi,
            functionName: message.data.function,
            params: message.data.inputs,
            gasLimit:
              message.data.gasLimit > 0 ? message.data.gasLimit : undefined,
            value: value > 0 ? value : undefined,
          });
          await this._view.webview.postMessage({
            type: MessageType.WRITE_RESPONSE,
            response: writeResponse,
          });
          break;
        case MessageType.READ:
          const readResponse = await this._interact.readContract({
            contract: message.data.contract,
            method: message.data.function,
            params: message.data.inputs,
          });
          await this._view.webview.postMessage({
            type: MessageType.READ_RESPONSE,
            response: readResponse,
          });
          break;
        case MessageType.EDIT_WALLETS:
          const walletAction = await window.showQuickPick(
            [InputAction.ADD, InputAction.REMOVE],
            {
              title: "Edit Wallets",
              ignoreFocusOut: true,
            },
          );

          if (walletAction === InputAction.ADD) {
            const walletName = await window.showInputBox({
              prompt: "Enter name",
              ignoreFocusOut: true,
            });
            const walletAddress = await window.showInputBox({
              prompt: "Enter address",
              ignoreFocusOut: true,
            });
            const walletPk = await window.showInputBox({
              prompt: "Enter private key",
              ignoreFocusOut: true,
            });
            const walletRpc = await window.showInputBox({
              prompt: "Enter rpc",
              ignoreFocusOut: true,
            });

            if (!walletName || !walletAddress || !walletPk || !walletRpc)
              return;
            if (!walletAddress.startsWith("0x") || !walletPk.startsWith("0x"))
              return;
            if (!walletRpc.startsWith("http") && !walletRpc.startsWith("ws"))
              return;

            this._walletRepository.createWallet({
              name: walletName,
              address: <Address>walletAddress,
              privateKey: <Address>walletPk,
              rpc: <RpcUrl>walletRpc,
            });
          }

          if (walletAction === InputAction.REMOVE) {
            const walletAddress = await window.showInputBox({
              prompt: "Enter address",
              ignoreFocusOut: true,
            });
            if (!walletAddress) return;
            this._walletRepository.deleteWallet(<Address>walletAddress);
          }
          break;
        case MessageType.EDIT_CONTRACTS:
          const contractAction = await window.showQuickPick(
            [InputAction.ADD, InputAction.REMOVE],
            {
              title: "Edit Wallets",
              ignoreFocusOut: true,
            },
          );

          if (contractAction === InputAction.ADD) {
            const contractName = await window.showInputBox({
              prompt: "Enter name",
              ignoreFocusOut: true,
            });
            const contractAddress = await window.showInputBox({
              prompt: "Enter address",
              ignoreFocusOut: true,
            });
            const contractAbi = await window.showInputBox({
              prompt: "Enter abi",
              ignoreFocusOut: true,
            });
            const contractRpc = await window.showInputBox({
              prompt: "Enter rpc",
              ignoreFocusOut: true,
            });
            const contractChainId = await window.showInputBox({
              prompt: "Enter chain id",
              ignoreFocusOut: true,
            });

            if (
              !contractName ||
              !contractAddress ||
              !contractAbi ||
              !contractRpc ||
              !contractChainId
            )
              return;
            if (!contractAddress.startsWith("0x")) return;
            if (
              !contractRpc.startsWith("http") &&
              !contractRpc.startsWith("ws")
            )
              return;

            this._contractRepository.createContract({
              address: <Address>contractAddress,
              abi: JSON.parse(contractAbi),
              chainId: parseInt(contractChainId),
              name: contractName,
              rpc: <RpcUrl>contractRpc,
            });
          }

          if (contractAction === InputAction.REMOVE) {
            const contractAddress = await window.showInputBox({
              prompt: "Enter address",
            });
            if (!contractAddress) return;
            this._contractRepository.deleteContract(<Address>contractAddress);
          }
          break;
        // start
        case MessageType.EDIT_ENVIRONMENT:
          const environmentAction = await window.showQuickPick(
            [InputAction.ADD, InputAction.REMOVE],
            {
              title: "Edit environment",
              ignoreFocusOut: true,
            },
          );

          if (environmentAction === InputAction.ADD) {
            const environmentName = await window.showInputBox({
              prompt: "Enter name",
              ignoreFocusOut: true,
            });
            const environmentRpc = await window.showInputBox({
              prompt: "Enter rpc",
              ignoreFocusOut: true,
            });

            if (!environmentName || !environmentRpc) return;
            if (
              !environmentRpc.startsWith("http") &&
              !environmentRpc.startsWith("ws")
            )
              return;

            this._environmentRepository.createEnvironment({
              name: environmentName,
              rpc: <RpcUrl>environmentRpc,
            });
          }

          if (environmentAction === InputAction.REMOVE) {
            const environmentName = await window.showInputBox({
              prompt: "Enter name",
              ignoreFocusOut: true,
            });
            if (!environmentName) return;
            this._environmentRepository.deleteEnvironment(environmentName);
          }
          break;

        case MessageType.DEPLOY_SCRIPT:
          const deployScriptArgs: DeployScriptArgs = message.data;
          const deployScriptResponse = await deployScript(
            deployScriptArgs.rpcUrl,
            deployScriptArgs.script,
            deployScriptArgs.verify,
          );
          await this._view.webview.postMessage({
            type: MessageType.DEPLOY_SCRIPT_RESPONSE,
            response: deployScriptResponse,
          });
          break;

        case MessageType.DEPLOY_CONTRACT:
          const deployContractArgs = message.data;
          const deployContractResponse = deployContract(
            deployContractArgs.rpcUrl,
            deployContractArgs.contract,
            deployContractArgs.verify,
            deployContractArgs.cstrArgs,
          );
          await this._view.webview.postMessage({
            type: MessageType.DEPLOY_CONTRACT_RESPONSE,
            response: deployContractResponse,
          });
          break;
      }
    });
  }

  private _getHtmlForWebview(webview: vscode.Webview) {
    const scriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, "dist", "index.js"),
    );
    const styleUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, "dist", "index.css"),
    );
    const nonce = getNonce();

    return `<!doctype html>
        <html lang="en">
          <head>
            <meta charset="UTF-8" />
            <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource}; script-src 'nonce-${nonce}';">
			<meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Panel</title>
            <script type="module" nonce="${nonce}" crossorigin src="${scriptUri}"></script>
            <link rel="stylesheet" crossorigin href="${styleUri}">
          </head>
          <body>
            <div id="root"></div>
          </body>
        </html>`;
  }
}
