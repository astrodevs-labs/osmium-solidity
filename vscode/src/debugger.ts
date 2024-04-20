import * as path from 'path';
import {
    debug,
    window,
    DebugAdapterDescriptorFactory,
    DebugSession,
    DebugAdapterExecutable,
    DebugAdapterDescriptor,
    ExtensionContext,
    OutputChannel,
    DebugConfigurationProvider,
    ProviderResult,
    DebugConfiguration,
    CancellationToken,
    WorkspaceFolder,
  } from 'vscode';
import * as os from 'os';

let outputChannel: OutputChannel;

export function registerDebugger(context: ExtensionContext) {
  outputChannel = window.createOutputChannel('Solidity Debugger');

  context.subscriptions.push(
    debug.registerDebugAdapterDescriptorFactory('solidity', new SolidityDebugAdapterDescriptorFactory(context)),
    debug.registerDebugConfigurationProvider('solidity', new SolidityDebugConfigurationProvider()),
    debug.onDidTerminateDebugSession(() => {
      outputChannel.appendLine(`Debug session ended.`);
    }),
  );
}

export class SolidityDebugAdapterDescriptorFactory implements DebugAdapterDescriptorFactory {
  context: ExtensionContext;

  constructor(context: ExtensionContext) {
    this.context = context;
  }

  async createDebugAdapterDescriptor(
    _session: DebugSession,
    _executable: DebugAdapterExecutable,
  ): Promise<ProviderResult<DebugAdapterDescriptor>> {
    const serverBinary = this.context.asAbsolutePath(
      path.join('dist',
      os.platform().startsWith("win") ? 'foundry-dap-server.exe' : 'foundry-dap-server')
    );

    if (!serverBinary) {
      throw new Error('Could not find Solidity debugger server');
    }

    return new DebugAdapterExecutable(serverBinary, []);
  }
}

class SolidityDebugConfigurationProvider implements DebugConfigurationProvider {
  resolveDebugConfiguration(
    _folder: WorkspaceFolder | undefined,
    config: DebugConfiguration,
    _token?: CancellationToken,
  ): ProviderResult<DebugConfiguration> {
    if (
      (!config.projectFolder || config.projectFolder === ``) &&
      window.activeTextEditor?.document.languageId != 'solidity' &&
      !window.activeTextEditor?.document.uri.fsPath.endsWith('.s.sol') &&
      !window.activeTextEditor?.document.uri.fsPath.endsWith('.t.sol')
    ) {
      throw new Error('No script/test selected');
    }

      const currentFilePath = window.activeTextEditor!.document.uri.fsPath;
      // TODO find the foundry.toml from the project folder
      const projectFolder = config.projectFolder;

    let functionName = '';
    window.showInputBox({
      prompt: currentFilePath.endsWith('.t.sol') ? 'Please enter the test name' : 'Please enter the script name',
    }).then((input) => {
      if (input) {
        functionName = input;
      } else {
        throw new Error('No input provided');
      }
    });

    const resolvedConfig = {
      type: config.type || 'solidity',
      name: config.name || 'Foundry Debug',
      request: 'launch',
      program: currentFilePath,
      functionName,
      projectFolder,
    };

    return resolvedConfig;
  }
}