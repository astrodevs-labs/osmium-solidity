// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import { workspace, ExtensionContext } from 'vscode';
import {
	LanguageClient,
} from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
import { createSlitherClient } from './slither';
import registerForgeFmtLinter from './fmt-wrapper';
import { TestManager } from './tests/test-manager';
import { createFoundryCompilerClient } from './foundry-compiler';
import { createTestsPositionsClient } from './tests-positions';
import { registerGasEstimation } from './gas-estimation';
import { createCodeActionsClient } from './code-actions';

let linterClient: LanguageClient;
let slitherClient: LanguageClient;
let foundryCompilerClient: LanguageClient;
let testsPositionsClient: LanguageClient;
let codeActionsClient: LanguageClient;
let testManager: TestManager;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export async function activate(context: ExtensionContext) {
	linterClient = await createLinterClient(context);
	slitherClient = await createSlitherClient(context);
	foundryCompilerClient = await createFoundryCompilerClient(context);
	testsPositionsClient = await createTestsPositionsClient(context);
	codeActionsClient = await createCodeActionsClient(context);
	if (workspace.workspaceFolders?.length) {
		testManager = new TestManager(testsPositionsClient, workspace.workspaceFolders[0].uri.fsPath);
	}

	registerForgeFmtLinter(context);
	registerGasEstimation();
	
	context.subscriptions.push(
		linterClient,
		slitherClient,
		foundryCompilerClient,
		testsPositionsClient,
		testManager.testController,
		codeActionsClient
	);

	const folders = workspace.workspaceFolders;
	if (folders) {
		const files = await workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
		files.forEach(file => {
			if (!file.path.includes('forge-std')) {
				workspace.openTextDocument(file);
			}
		});
	}
}

// This method is called when your extension is deactivated
export function deactivate() { }
