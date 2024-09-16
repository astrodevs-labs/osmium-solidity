// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import { workspace, ExtensionContext, window, commands, Disposable } from 'vscode';
import { LanguageClient } from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
import { createSlitherClient } from './slither';
import registerForgeFmtLinter, { format } from './fmt-wrapper';
import { TestManager } from './tests/test-manager';
import { createFoundryCompilerClient } from './foundry-compiler';
import { createTestsPositionsClient } from './tests-positions';
import { registerGasEstimation } from './gas-estimation';
import { createCodeActionsClient } from './code-actions';
import { SidebarProvider } from './sidebar-provider';

let linterClient: LanguageClient | null;
let slitherClient: LanguageClient | null;
let foundryCompilerClient: LanguageClient | null;
let testsPositionsClient: LanguageClient | null;
let codeActionsClient: LanguageClient | null;
let testManager: TestManager | null;
let saveHandler: Disposable | null;
let formatterHandlers: {
  fileDisposable: Disposable;
  workspaceDisposable: Disposable;
  formatterDisposable: Disposable;
} | null;
let interactDeployHandler: Disposable | null;
let gasEstimationHandler: {
  openDisposable: Disposable;
  SaveDisposable: Disposable;
  visibleTextEditorsDisposable: Disposable;
  activeTextEditorDisposable: Disposable;
  commandDisposable: Disposable;
} | null;

let Extcontext: ExtensionContext;

export async function activate(context: ExtensionContext) {
  Extcontext = context;
  await launchFeatures();

  workspace.onDidChangeConfiguration(launchFeatures);
}

async function launchFeatures() {
  const configuration = workspace.getConfiguration('Osmium');

  const isLinterEnable = configuration.get('linter');
  const isSlitherEnable = configuration.get('slither');
  const isGasEstimationEnable = configuration.get('gas estimation');
  const isSidebarEnable = configuration.get('sidebar');
  const isDebuggerEnable = configuration.get('debugger');
  const isTestsEnable = configuration.get('tests');
  const isCompilerEnable = configuration.get('compiler');
  const isreferencesEnable = configuration.get('references');
  const isAutoFormatEnable = configuration.get('auto format');
  const isFormatterEnable = configuration.get('formatter');
  const sidebarProvider = new SidebarProvider(Extcontext.extensionUri);

  if (isAutoFormatEnable && isFormatterEnable && !saveHandler) {
    saveHandler = workspace.onDidSaveTextDocument(format);
  } else if (!isAutoFormatEnable && saveHandler) {
    saveHandler.dispose();
  }

  if (isFormatterEnable && !formatterHandlers) {
    formatterHandlers = registerForgeFmtLinter(Extcontext);
  } else if (!isFormatterEnable && formatterHandlers) {
    formatterHandlers?.fileDisposable.dispose();
    formatterHandlers?.workspaceDisposable.dispose();
    formatterHandlers?.formatterDisposable.dispose();
    formatterHandlers = null;
  }

  if (isSidebarEnable && !interactDeployHandler) {
    commands.executeCommand('setContext', 'Osmium.showsidebar', true);
    interactDeployHandler = window.registerWebviewViewProvider(SidebarProvider.viewType, sidebarProvider);
    Extcontext.subscriptions.push(interactDeployHandler);
  } else if (!isSidebarEnable && interactDeployHandler) {
    commands.executeCommand('setContext', 'Osmium.showsidebar', false);
    interactDeployHandler.dispose();
    interactDeployHandler = null;
  }

  if (isGasEstimationEnable && !gasEstimationHandler) {
    gasEstimationHandler = registerGasEstimation(Extcontext);
  } else if (!isGasEstimationEnable && gasEstimationHandler) {
    gasEstimationHandler.SaveDisposable.dispose();
    gasEstimationHandler.openDisposable.dispose();
    gasEstimationHandler.visibleTextEditorsDisposable.dispose();
    gasEstimationHandler.activeTextEditorDisposable.dispose();
    gasEstimationHandler.commandDisposable.dispose();
    gasEstimationHandler = null;
  }

  if (isCompilerEnable && !foundryCompilerClient) {
    foundryCompilerClient = createFoundryCompilerClient(Extcontext);
    Extcontext.subscriptions.push(foundryCompilerClient);
  } else if (!isCompilerEnable && foundryCompilerClient) {
    foundryCompilerClient.stop();
    foundryCompilerClient = null;
  }

  if (isLinterEnable && !linterClient) {
    linterClient = await createLinterClient(Extcontext);
    Extcontext.subscriptions.push(linterClient);
  } else if (!isLinterEnable && linterClient) {
    linterClient.stop();
    linterClient = null;
  }

  if (isreferencesEnable && !codeActionsClient) {
    codeActionsClient = await createCodeActionsClient(Extcontext);
    Extcontext.subscriptions.push(codeActionsClient);
  } else if (!isreferencesEnable && codeActionsClient) {
    codeActionsClient.stop();
    codeActionsClient = null;
  }

  if (isSlitherEnable && !slitherClient) {
    slitherClient = await createSlitherClient(Extcontext);
    Extcontext.subscriptions.push(slitherClient);
  } else if (!isSlitherEnable && slitherClient) {
    slitherClient.stop();
    slitherClient = null;
  }

  if (isDebuggerEnable) {
  }

  if (workspace.workspaceFolders?.length && isTestsEnable && !testsPositionsClient) {
    testsPositionsClient = await createTestsPositionsClient(Extcontext);
    testManager = new TestManager(testsPositionsClient, workspace.workspaceFolders[0].uri.fsPath);
    Extcontext.subscriptions.push(testManager.testController, testsPositionsClient);
  } else if (!isTestsEnable && testsPositionsClient) {
    testsPositionsClient.stop();
    testsPositionsClient = null;
  }

  const folders = workspace.workspaceFolders;
  if (folders) {
    const files = await workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
    files.forEach((file) => {
      if (!file.path.includes('forge-std')) {
        workspace.openTextDocument(file);
      }
    });
  }
}

// This method is called when your extension is deactivated
export function deactivate() {}
