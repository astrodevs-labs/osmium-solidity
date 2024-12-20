import * as path from 'path';
import * as vscode from 'vscode';
import { Deploy } from './actions/Deploy';
import { DeployContractRepository } from './actions/DeployContractRepository';
import { EnvironmentRepository } from './actions/EnvironmentRepository';
import { Interact } from './actions/Interact';
import { InteractContractRepository } from './actions/InteractContractRepository';
import { ScriptRepository } from './actions/ScriptRepository';
import { WalletRepository } from './actions/WalletRepository';
import { publicClient } from './config';
import { MessageType } from './enums';
import { Message } from './types';
import { getNonce, getTomlValue } from './utils';
import { EnvPanelProvider } from './env-panel-provider';

export class SidebarProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = 'osmium.sidebar';
  private _osmiumWatcher?: vscode.FileSystemWatcher;
  private _outWatcher?: vscode.FileSystemWatcher;
  private _view?: vscode.WebviewView;
  private _outputChannel: vscode.OutputChannel;

  private _deployContractRepository?: DeployContractRepository;
  private _scriptRepository?: ScriptRepository;
  private _interact?: Interact;
  private _deploy?: Deploy;

  constructor(
    private readonly _extensionUri: vscode.Uri,
    private readonly _interactContractRepository: InteractContractRepository,
    private readonly _walletRepository: WalletRepository,
    private readonly _environmentRepository: EnvironmentRepository,
    private readonly _envPanelProvider: EnvPanelProvider,
  ) {
    this._outputChannel = vscode.window.createOutputChannel('Osmium Solidity Logs');
  }

  async _osmiumWatcherCallback(uri: vscode.Uri) {
    if (!this._view) {
      return;
    }
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
          outputChannel: this._outputChannel,
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
          outputChannel: this._outputChannel,
        });
        await this._view.webview.postMessage({
          type: MessageType.READ_RESPONSE,
          response: readResponse.toString(),
        });
        break;
      case MessageType.DEPLOY_SCRIPT:
        const deployScriptResponse = await this._deploy.deployScript({
          environmentId: message.data.environment,
          scriptId: message.data.script,
          verify: message.data.verify,
          outputChannel: this._outputChannel,
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
          outputChannel: this._outputChannel,
        });
        await this._view.webview.postMessage({
          type: MessageType.DEPLOY_CONTRACT_RESPONSE,
          response: deployContractResponse,
        });
        break;
      case MessageType.OPEN_PANEL:
        await vscode.commands.executeCommand('osmium.show-env-panel');

        if (this._envPanelProvider.panel) {
          await this._envPanelProvider.panel.webview.postMessage({
            type: MessageType.OPEN_PANEL_RESPONSE,
            id: message.data.id,
          });
        }
        break;
      case MessageType.OPEN_DOCUMENTATION:
        vscode.commands.executeCommand('osmium.documentation');
        break;
      case MessageType.OPEN_WALKTHROUGH:
        vscode.commands.executeCommand(
          'workbench.action.openWalkthrough',
          'OsmiumToolchains.osmium-solidity-extension#osmium.getStarted',
        );
        break;

      case MessageType.ESTIMATE_GAS:
        const gas = await publicClient.estimateContractGas({
          address: message.data.address,
          abi: message.data.abi,
          functionName: message.data.function,
          account: message.data.walletAddress,
          args: message.data.params,
        });
        const gasWithBuffer = (gas * 12n) / 10n; // gas + 20%, as it's a bigint we can't use * 1.2
        await this._view.webview.postMessage({
          type: MessageType.ESTIMATE_GAS_RESPONSE,
          response: { gas: gasWithBuffer.toString() },
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
