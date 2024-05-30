import * as vscode from 'vscode';

// export class DocumentationPanelProvider implements vscode.WebviewViewProvider {
//   public static readonly viewType = 'osmium.documentation';

//   private _view?: vscode.WebviewView;

//   constructor(private readonly _extensionUri: vscode.Uri) {}

//   public resolveWebviewView(
//     webviewView: vscode.WebviewView,
//     // eslint-disable-next-line @typescript-eslint/no-unused-vars
//     context: vscode.WebviewViewResolveContext,
//     // eslint-disable-next-line @typescript-eslint/no-unused-vars
//     _token: vscode.CancellationToken,
//   ) {
//     this._view = webviewView;

//     webviewView.webview.options = {
//       // Allow scripts in the webview
//       enableScripts: true,

//       localResourceRoots: [this._extensionUri],
//     };

//     webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

//     webviewView.webview.onDidReceiveMessage((data) => {
//       console.log(data);
//       this._answer();
//     });
//   }

//   public _answer() {
//     if (this._view) {
//       this._view.show?.(true); // `show` is not implemented in 1.49 but is for 1.50 insiders
//       this._view.webview.postMessage('answer');
//     }
//   }

//   public getNonce(): string {
//     let text: string = '';
//     const possible: string = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
//     for (let i = 0; i < 32; i++) {
//       text += possible.charAt(Math.floor(Math.random() * possible.length));
//     }
//     return text;
//   }

//   private _getHtmlForWebview(webview: vscode.Webview) {
//     // Get the local path to main script run in the webview, then convert it to a uri we can use in the webview.
//     const scriptUri = webview.asWebviewUri(vscode.Uri.joinPath(this._extensionUri, 'react', 'dist', 'index.js'));

//     // Use a nonce to only allow a specific script to be run.
//     const nonce = this.getNonce();

//     return `<!doctype html>
//         <html lang="en">
//           <head>
//             <meta charset="UTF-8" />
//             <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource}; script-src 'nonce-${nonce}';">
// 			<meta name="viewport" content="width=device-width, initial-scale=1.0">
//             <title>Panel</title>
//             <script type="module" nonce="${nonce}" crossorigin src="${scriptUri}"></script>
//           </head>
//           <body>
//             <div id="root"></div>
//           </body>
//         </html>`;
//   }
// }

export function registerDocumentationPanel(context: vscode.ExtensionContext) {
  // context.subscriptions.push(
  //   vscode.window.registerWebviewViewProvider(DocumentationPanelProvider.viewType, new DocumentationPanelProvider(context.extensionUri)),
  // );

  vscode.commands.registerCommand('osmium.documentation', () => {
    vscode.window.createWebviewPanel(
      'documentation',
      'Documentation',
      vscode.ViewColumn.One,
      {
        enableScripts: true,
      },
    );
  });
}