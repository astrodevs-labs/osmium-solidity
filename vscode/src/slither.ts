import * as path from "path";
import * as os from "os";
import { workspace, ExtensionContext, Uri } from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";
import { TextDecoder } from "util";

export async function createSlitherClient(
  context: ExtensionContext,
): Promise<LanguageClient> {
  // The server is implemented in node
  const serverBinary = context.asAbsolutePath(
    path.join(
      "dist",
      os.platform().startsWith("win") ? "slither-server.exe" : "slither-server",
    ),
  );

  // If the extension is launched in debug mode then the debug server options are used
  // Otherwise the run options are used
  const serverOptions: ServerOptions = {
    run: { command: serverBinary, transport: TransportKind.stdio },
    debug: {
      command: serverBinary,
      transport: TransportKind.stdio,
    },
  };

  // Options to control the language client
  const clientOptions: LanguageClientOptions = {
    // Register the server for plain text documents
    documentSelector: [{ scheme: "file", language: "solidity" }],
    synchronize: {
      // Notify the server about file changes to '.clientrc files contained in the workspace
      fileEvents: workspace.createFileSystemWatcher("**/.solidhunter.json"),
    },
  };

  // Create the language client and start the client.
  const client = new LanguageClient(
    "osmium-slither",
    "Osmium Slither Language Server",
    serverOptions,
    clientOptions,
  );

  client.onRequest("osmium/getContent", async (params: { uri: string }) => {
    const contentUint8 = await workspace.fs.readFile(Uri.parse(params.uri));
    const content = new TextDecoder().decode(contentUint8);
    return {
      content,
    };
  });

  // Start the client. This will also launch the server
  await client.start();

  return client;
}
