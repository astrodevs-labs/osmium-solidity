import * as vscode from 'vscode';
import { getNonce } from './utils';

export class EnvPanelProvider {
  public static readonly viewType = 'osmium.env-panel';
  public panel: vscode.WebviewPanel | undefined = undefined;

  constructor(private readonly _extensionUri: vscode.Uri) {}

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
