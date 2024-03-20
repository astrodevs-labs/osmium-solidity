import * as path from 'path';
import * as os from 'os';
import { workspace, ExtensionContext, Uri } from "vscode";
import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind,
	SocketTransport,
	StreamInfo
} from 'vscode-languageclient/node';
import { TextDecoder } from 'util';
import * as net from 'net';

export async function createReferencesClient(context: ExtensionContext): Promise<LanguageClient> {
	/*
	let connectionInfo = {
		port: 9001,
		host: "127.0.0.1"
    };
	let serverOptions = () => {
        // Connect to language server via socket
        let socket = net.connect(connectionInfo);
        let result: StreamInfo = {
            writer: socket,
            reader: socket
        };
		return Promise.resolve(result);
	};
	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const socketOptions: SocketTransport = {
		port: 9001,
		kind: TransportKind.socket,
	};
	*/
	
	// The server is implemented in node
	const serverBinary = context.asAbsolutePath(
		path.join(
			'dist', 
			os.platform().startsWith("win") ? 'references-server.exe' : 'references-server'
		)
	);

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
		'osmium-solidity-references',
		'Osmium Solidity References Language Server',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	await client.start();

    return client;
}