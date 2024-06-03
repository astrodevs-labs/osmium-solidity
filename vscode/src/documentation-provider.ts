import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

export function registerDocumentationPanel(context: vscode.ExtensionContext) {
  vscode.commands.registerCommand('osmium.documentation', () => {
    const panel = vscode.window.createWebviewPanel(
      'documentation',
      'Documentation',
      vscode.ViewColumn.One,
      {
        enableScripts: true,
        localResourceRoots: [vscode.Uri.file(path.join(context.extensionPath, 'dist'))]
      },
    );

    const htmlPath = path.join(context.extensionPath, 'dist', 'documentation.html');
    //const jsPath = path.join(context.extensionPath, 'dist', 'documentation.js');

    const htmlContent = fs.readFileSync(htmlPath, 'utf8');
    //const jsContent = fs.readFileSync(jsPath, 'utf8');

    const htmlUri = panel.webview.asWebviewUri(vscode.Uri.file(htmlPath));
    //const jsUri = panel.webview.asWebviewUri(vscode.Uri.file(jsPath));

    panel.webview.html = htmlContent.replace(/{{root}}/g, htmlUri.toString());
  });
}