import * as path from 'path';
import { workspace, ExtensionContext } from "vscode";
import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';
import * as os from 'os';

export function createFoundryCompilerClient(context: ExtensionContext): LanguageClient {
    // The server is implemented in node
	const serverBinary = context.asAbsolutePath(
		path.join('dist',
		os.platform().startsWith("win") ? 'foundry-compiler-server.exe' : 'foundry-compiler-server')
	);

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used[]
	const serverOptions: ServerOptions = {
		run: { command: serverBinary, transport: TransportKind.stdio },
		debug: {
			command: serverBinary,
			transport: TransportKind.stdio,
		}
	};

	// Options to control the language client
	const clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'solidity' }],
		synchronize: {
			// Notify the server about file changes to '.clientrc files contained in the workspace
			fileEvents: workspace.createFileSystemWatcher('**/.solidhunter.json')
		}
	};

	// Create the language client and start the client.
	const client = new LanguageClient(
		'osmium-solidity-foundry-compiler',
		'Osmium Solidity Foundry Compiler Language Server',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start();

    return client;
}