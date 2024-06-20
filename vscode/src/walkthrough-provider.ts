import * as vscode from 'vscode';
import { exec } from 'child_process';

export function registerWalkthroughPanel(context: vscode.ExtensionContext) {
  // Command to open the walkthrough
  context.subscriptions.push(vscode.commands.registerCommand('osmium.walkthrough', () => {
    vscode.commands.executeCommand('workbench.action.openWalkthrough', 'OsmiumToolchains.osmium-solidity-extension#osmium.getStarted');
  }));

  // Command to check if forge is installed
  context.subscriptions.push(vscode.commands.registerCommand('osmium.checkIfForgeInstalled', () => {
    exec('forge --version', (err, stdout, stderr) => {
      if (err) {
        vscode.window.showErrorMessage('Foundry/Forge is not installed. Please install it from https://book.getfoundry.sh/getting-started/installation');
        vscode.commands.executeCommand('setContext', 'osmium.forgeInstalled', false);
      } else {
        vscode.window.showInformationMessage(`Foundry/Forge is installed: ${stdout}`);
        vscode.commands.executeCommand('setContext', 'osmium.forgeInstalled', true);
      }
    });
  }));

  // Command to check if slither is installed
  context.subscriptions.push(vscode.commands.registerCommand('osmium.checkIfSlitherInstalled', () => {
    exec('slither --version', (err, stdout, stderr) => {
      if (err) {
        vscode.window.showErrorMessage('Slither is not installed. Please install it from https://github.com/crytic/slither?tab=readme-ov-file#how-to-install');
        vscode.commands.executeCommand('setContext', 'osmium.slitherInstalled', false);
      } else {
        vscode.window.showInformationMessage(`Slither is installed: ${stdout}`);
        vscode.commands.executeCommand('setContext', 'osmium.slitherInstalled', true);
      }
    });
  }));

  // Command to check if solc is installed
  context.subscriptions.push(vscode.commands.registerCommand('osmium.checkIfSolcInstalled', () => {
    exec('solc --version', (err, stdout, stderr) => {
      if (err) {
        vscode.window.showErrorMessage('Solc is not installed. Please install it from https://docs.soliditylang.org/en/latest/installing-solidity.html');
        vscode.commands.executeCommand('setContext', 'osmium.solcInstalled', false);
      } else {
        vscode.window.showInformationMessage(`Solc is installed: ${stdout}`);
        vscode.commands.executeCommand('setContext', 'osmium.solcInstalled', true);
      }
    });
  }));
}
