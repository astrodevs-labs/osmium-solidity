// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import { workspace, ExtensionContext } from 'vscode';
import {
	LanguageClient,
} from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
// import { createSlitherClient } from './slither';
import registerForgeFmtLinter from './fmt-wrapper';
import { TestManager } from './tests/test-manager';
import { createFoundryCompilerClient } from './foundry-compiler';
import { createTestsPositionsClient } from './tests-positions';
import { registerGasEstimation } from './gas-estimation';

let linterClient: LanguageClient;
let slitherClient: LanguageClient;
let foundryCompilerClient: LanguageClient;
let testsPositionsClient: LanguageClient;
let testManager: TestManager;

const configuration = workspace.getConfiguration('Osmium');
console.log('Osmium configuration:', configuration);

const isLinterEnable = configuration.get('linter');
const isSlitherEnable = configuration.get('slither');
const isGasEstimationEnable = configuration.get('gas estimation');
const isInteractEnable = configuration.get('interact');
const isDeployEnable = configuration.get('deploy');
const isDebuggerEnable = configuration.get('debugger');
const isTestsEnable = configuration.get('tests');
const isCompilatorEnable = configuration.get('compilator');
const isreferencesEnable = configuration.get('references');

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export async function activate(context: ExtensionContext) {
	if (isLinterEnable) {
		linterClient = await createLinterClient(context);
		registerForgeFmtLinter(context);
		context.subscriptions.push(linterClient);
	}
	if (isSlitherEnable) {
		// slitherClient = await createSlitherClient(context);
		// context.subscriptions.push(slitherClient);
	}
	if (isGasEstimationEnable) {
		registerGasEstimation();
	}
	if (isInteractEnable) {
	}
	if (isDeployEnable) {
	}
	if (isDebuggerEnable) {
	}
	if (workspace.workspaceFolders?.length && isTestsEnable) {
		testsPositionsClient = await createTestsPositionsClient(context);
		testManager = new TestManager(testsPositionsClient, workspace.workspaceFolders[0].uri.fsPath);
		context.subscriptions.push(testManager.testController, testsPositionsClient);
	}
	if (isCompilatorEnable) {
		foundryCompilerClient = await createFoundryCompilerClient(context);
		context.subscriptions.push(foundryCompilerClient);
	}
	if (isreferencesEnable) {
	}

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
