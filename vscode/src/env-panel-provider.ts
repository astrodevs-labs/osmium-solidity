import * as vscode from 'vscode';
import { getNonce } from './utils';
import { InteractContractRepository } from './actions/InteractContractRepository';
import { WalletRepository } from './actions/WalletRepository';
import { EnvironmentRepository } from './actions/EnvironmentRepository';
import { Message } from './types';
import { MessageType } from './enums';
import path from 'path';

export class EnvPanelProvider {
  public static readonly viewType = 'osmium.env-panel';
  public panel: vscode.WebviewPanel | undefined = undefined;
  private _osmiumWatcher?: vscode.FileSystemWatcher;

  constructor(
    private readonly _extensionUri: vscode.Uri,
    private readonly _interactContractRepository: InteractContractRepository,
    private readonly _walletRepository: WalletRepository,
    private readonly _environmentRepository: EnvironmentRepository,
  ) {
    this._osmiumWatcher = vscode.workspace.createFileSystemWatcher('**/.osmium/*.json');
    this._osmiumWatcher.onDidChange((uri) => this._osmiumWatcherCallback(uri));
  }

  async _osmiumWatcherCallback(uri: vscode.Uri) {
    if (!this.panel) return;
    const basename = path.basename(uri.fsPath, '.json');
    if (basename === 'contracts') {
      this._interactContractRepository?.load();
      await this.panel.webview.postMessage({
        type: MessageType.INTERACT_CONTRACTS,
        contracts: this._interactContractRepository?.getContracts(),
      });
    }
    if (basename === 'wallets') {
      this._walletRepository?.load();
      await this.panel.webview.postMessage({
        type: MessageType.WALLETS,
        wallets: this._walletRepository?.getWallets(),
      });
    }
    if (basename === 'environments') {
      this._environmentRepository?.load();
      await this.panel.webview.postMessage({
        type: MessageType.ENVIRONMENTS,
        environments: this._environmentRepository?.getEnvironments(),
      });
    }
  }

  async _onMessageCallback(message: Message) {
    if (!this.panel) {
      return;
    }
    switch (message.type) {
      case MessageType.GET_WALLETS:
        await this.panel.webview.postMessage({
          type: MessageType.WALLETS,
          wallets: this._walletRepository.getWallets(),
        });
        break;
      case MessageType.GET_INTERACT_CONTRACTS:
        await this.panel.webview.postMessage({
          type: MessageType.INTERACT_CONTRACTS,
          contracts: this._interactContractRepository.getContracts(),
        });
        break;
      case MessageType.GET_ENVIRONMENTS:
        await this.panel.webview.postMessage({
          type: MessageType.ENVIRONMENTS,
          environments: this._environmentRepository.getEnvironments(),
        });
        break;
      case MessageType.DELETE_WALLET:
        this._walletRepository.deleteWallet(message.data.id);
        break;
      case MessageType.EDIT_WALLET:
        this._walletRepository.updateWallet(message.data.id, message.data.key, message.data.value);
        break;
      case MessageType.ADD_WALLET:
        this._walletRepository.createWallet(message.data.name, message.data.privateKey);
        break;
      case MessageType.DELETE_ENVIRONMENT:
        this._environmentRepository.deleteEnvironment(message.data.id);
        break;
      case MessageType.EDIT_ENVIRONMENT:
        this._environmentRepository.updateEnvironment(message.data.id, message.data.key, message.data.value);
        break;
      case MessageType.ADD_ENVIRONMENT:
        this._environmentRepository.createEnvironment(message.data.name, message.data.rpc);
        break;
      case MessageType.DELETE_CONTRACT:
        this._interactContractRepository.deleteContract(message.data.id);
        break;
      case MessageType.EDIT_CONTRACT:
        this._interactContractRepository.updateContract(message.data.id, message.data.key, message.data.value);
        break;
      case MessageType.ADD_CONTRACT:
        this._interactContractRepository.createContract(
          message.data.address,
          JSON.parse(message.data.abi),
          message.data.chainId,
          message.data.name,
          message.data.rpc,
        );
        break;
    }
  }

  public async resolveWebview(context: vscode.ExtensionContext) {
    if (this.panel) {
      this.panel.reveal();
    } else {
      this.panel = vscode.window.createWebviewPanel(
        EnvPanelProvider.viewType,
        'Environment Panel',
        vscode.ViewColumn.One,
        {
          enableScripts: true,
          localResourceRoots: [this._extensionUri],
        },
      );
      this.panel.webview.html = this._getHtmlForWebview(this.panel.webview);
      this.panel.webview.onDidReceiveMessage((e) => {
        this._onMessageCallback(e);
      });

      this.panel.onDidDispose(
        () => {
          this.panel = undefined;
        },
        null,
        context.subscriptions,
      );
    }
  }

  private _getHtmlForWebview(webview: vscode.Webview) {
    const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'dist', 'index-env-panel.js'));
    const styleUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'dist', 'index-env-panel.css'));
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
