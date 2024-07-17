import { Address } from 'viem';
import * as vscode from 'vscode';
import { window } from 'vscode';
import { InteractContractRepository } from './actions/InteractContractRepository';
import { Interact } from './actions/Interact';
import { WalletRepository } from './actions/WalletRepository';
import { RpcUrl } from './actions/types';
import { EnvironmentRepository } from './actions/EnvironmentRepository';
import { getNonce, getTomlValue } from './utils';
import { ScriptRepository } from './actions/ScriptRepository';
import { DeployContractRepository } from './actions/DeployContractRepository';
import { Deploy } from './actions/Deploy';
import { InputAction, MessageType } from './enums';
import { Message } from './types';
import * as path from 'path';

export class SidebarProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = 'osmium.sidebar';
  private _osmiumWatcher?: vscode.FileSystemWatcher;
  private _outWatcher?: vscode.FileSystemWatcher;
  private _view?: vscode.WebviewView;

  private _deployContractRepository?: DeployContractRepository;
  private _scriptRepository?: ScriptRepository;
  private _interact?: Interact;
  private _deploy?: Deploy;

  constructor(
    private readonly _extensionUri: vscode.Uri,
    private readonly _interactContractRepository: InteractContractRepository,
    private readonly _walletRepository: WalletRepository,
    private readonly _environmentRepository: EnvironmentRepository,
  ) {}

  async _osmiumWatcherCallback(uri: vscode.Uri) {
    if (!this._view) {return;}
    const basename = path.basename(uri.fsPath, '.json');
    if (basename === 'contracts') {
      this._interactContractRepository?.load();
      await this._view.webview.postMessage({
        type: MessageType.INTERACT_CONTRACTS,
        contracts: this._interactContractRepository?.getContracts(),
      });
    }
    if (basename === 'wallets') {
      this._walletRepository?.load();
      await this._view.webview.postMessage({
        type: MessageType.WALLETS,
        wallets: this._walletRepository?.getWallets(),
      });
    }
    if (basename === 'environments') {
      this._environmentRepository?.load();
      await this._view.webview.postMessage({
        type: MessageType.ENVIRONMENTS,
        environments: this._environmentRepository?.getEnvironments(),
      });
    }
  }

  async _outWatcherCallback() {
    if (!this._view) {
      return;
    }
    this._deployContractRepository?.load();
    await this._view.webview.postMessage({
      type: MessageType.DEPLOY_CONTRACTS,
      contracts: this._deployContractRepository?.getContracts(),
    });
  }

  async _showInputsBox(inputsBox: any) {
    const tmp = inputsBox;

    for (const input of Object.keys(inputsBox)) {
      const value = await window.showInputBox({
        prompt: inputsBox[input],
        ignoreFocusOut: true,
      });
      if (!value) {
        return undefined;
      }
      tmp[input] = value;
    }

    return tmp;
  }

  _init() {
    if (vscode.workspace.workspaceFolders?.length) {
      const fsPath = vscode.workspace.workspaceFolders?.[0].uri.fsPath;
      this._deployContractRepository = new DeployContractRepository(fsPath);
      this._scriptRepository = new ScriptRepository(fsPath);

      this._interact = new Interact(this._interactContractRepository, this._walletRepository);
      this._deploy = new Deploy(
        this._deployContractRepository,
        this._walletRepository,
        this._scriptRepository,
        this._environmentRepository,
        fsPath,
      );
      this._osmiumWatcher = vscode.workspace.createFileSystemWatcher('**/.osmium/*.json');
      this._osmiumWatcher.onDidChange((uri) => this._osmiumWatcherCallback(uri));
      this._outWatcher = vscode.workspace.createFileSystemWatcher(
        `**/${getTomlValue('foundry.toml', 'out') ?? 'out'}/*.json`,
      );
      this._outWatcher.onDidChange(() => this._outWatcherCallback());
    }
  }

  async _onMessageCallback(message: Message) {
    if (
      !this._view ||
      !this._interactContractRepository ||
      !this._deployContractRepository ||
      !this._walletRepository ||
      !this._environmentRepository ||
      !this._scriptRepository ||
      !this._interact ||
      !this._deploy
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
          contracts: this._interactContractRepository.getContracts(),
        });
        break;
      case MessageType.GET_SCRIPTS:
        await this._view.webview.postMessage({
          type: MessageType.SCRIPTS,
          scripts: this._scriptRepository.getScripts(),
        });
        break;
      case MessageType.GET_DEPLOY_CONTRACTS:
        await this._view.webview.postMessage({
          type: MessageType.DEPLOY_CONTRACTS,
          contracts: this._deployContractRepository.getContracts(),
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

        if (message.data.valueUnit === 'ether') {
          value = value * BigInt(10) ** BigInt(18);
        } else if (message.data.valueUnit === 'gwei') {
          value = value * BigInt(10) ** BigInt(9);
        }

        const writeResponse = await this._interact.writeContract({
          walletId: message.data.wallet,
          contractId: message.data.contract,
          functionName: message.data.function,
          params: message.data.inputs,
          gasLimit: message.data.gasLimit > 0 ? message.data.gasLimit : undefined,
          value: value > 0 ? value : undefined,
        });
        await this._view.webview.postMessage({
          type: MessageType.WRITE_RESPONSE,
          response: writeResponse,
        });
        break;
      case MessageType.READ:
        const readResponse = await this._interact.readContract({
          contractId: message.data.contract,
          method: message.data.function,
          params: message.data.inputs,
        });
        await this._view.webview.postMessage({
          type: MessageType.READ_RESPONSE,
          response: readResponse.toString(),
        });
        break;
      case MessageType.OPEN_PANEL:
        await vscode.commands.executeCommand('osmium.openPanel');
        break;
      case MessageType.EDIT_CONTRACTS:
        const contractAction = await window.showQuickPick([InputAction.ADD, InputAction.REMOVE], {
          title: 'Edit Wallets',
          ignoreFocusOut: true,
        });

        if (contractAction === InputAction.ADD) {
          const inputs = await this._showInputsBox({
            contractName: 'Enter name',
            contractAddress: 'Enter address',
            contractAbi: 'Enter abi',
            contractRpc: 'Enter rpc',
            contractChainId: 'Enter chain id',
          });
          if (!inputs || !inputs.contractAddress.startsWith('0x')) return;
          if (!inputs.contractRpc.startsWith('http') && !inputs.contractRpc.startsWith('ws')) return;
          this._interactContractRepository.createContract(
            <Address>inputs['contractAddress'],
            JSON.parse(inputs['contractAbi']),
            parseInt(inputs['contractChainId']),
            inputs['contractName'],
            <RpcUrl>inputs['contractRpc'],
          );
        }
        if (contractAction === InputAction.REMOVE) {
          const contractName = await window.showQuickPick(
            this._interactContractRepository.getContracts().map((c) => c.name),
            {
              title: 'Remove contract',
              ignoreFocusOut: true,
            },
          );
          if (!contractName) return;
          this._interactContractRepository.deleteContract(contractName);
        }
        break;
      case MessageType.EDIT_ENVIRONMENT:
        const environmentAction = await window.showQuickPick([InputAction.ADD, InputAction.REMOVE], {
          title: 'Edit environment',
          ignoreFocusOut: true,
        });
        if (environmentAction === InputAction.ADD) {
          const inputs = await this._showInputsBox({
            environmentName: 'Enter name',
            environmentRpc: 'Enter rpc',
          });
          if (!inputs) return;
          if (!inputs.environmentRpc.startsWith('http') && !inputs.environmentRpc.startsWith('ws')) return;

          this._environmentRepository.createEnvironment(inputs.environmentName, <RpcUrl>inputs.environmentRpc);
        }
        if (environmentAction === InputAction.REMOVE) {
          const environmentName = await window.showQuickPick(
            this._environmentRepository.getEnvironments().map((e) => e.name),
            {
              title: 'Remove environment',
              ignoreFocusOut: true,
            },
          );
          if (!environmentName) return;
          this._environmentRepository.deleteEnvironment(environmentName);
        }
        break;
      case MessageType.DEPLOY_SCRIPT:
        const deployScriptResponse = await this._deploy.deployScript({
          environmentId: message.data.environment,
          scriptId: message.data.script,
          verify: message.data.verify,
        });
        await this._view.webview.postMessage({
          type: MessageType.DEPLOY_SCRIPT_RESPONSE,
          response: deployScriptResponse,
        });
        break;
      case MessageType.DEPLOY_CONTRACT:
        const deployContractResponse = await this._deploy.deployContract({
          contractId: message.data.contract,
          environmentId: message.data.environment,
          walletId: message.data.wallet,
          value: message.data.value,
          gasLimit: message.data.gasLimit,
          params: message.data.inputs,
          verify: message.data.verify,
        });
        await this._view.webview.postMessage({
          type: MessageType.DEPLOY_CONTRACT_RESPONSE,
          response: deployContractResponse,
        });
        break;
    }
  }

  public async resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken,
  ) {
    this._view = webviewView;
    this._init();

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this._extensionUri],
    };
    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);
    webviewView.webview.onDidReceiveMessage((e) => {
      this._onMessageCallback(e);
    });
  }

  private _getHtmlForWebview(webview: vscode.Webview) {
    const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'dist', 'index-sidebar.js'));
    const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'dist', 'index-sidebar.css'));
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
