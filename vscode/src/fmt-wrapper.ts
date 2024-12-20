import { exec } from 'child_process';
import * as vscode from 'vscode';
import { Disposable } from 'vscode';

type ForgeFmtOptions = {
  root?: string; // Root is used to get fmt config from forge.toml
} & (
  | {
      check: true;
      raw?: boolean;
    }
  | {
      check?: false;
      raw: false;
    }
);

type ForgeFmtArgs = {
  options: ForgeFmtOptions;
  files?: string[];
  content?: string;
};

type ForgeFmtResult = {
  exitCode: number;
  output: string;
};

function isFmtInstalled(): boolean {
  try {
    exec('forge --version', (error, _stdout, _stderr) => {
      if (error) {
        throw error;
      }
    });
    return true;
  } catch (error) {
    return false;
  }
}

function forgeFmt(args: ForgeFmtArgs, debug?: boolean): Promise<ForgeFmtResult> {
  const { options, files, content } = args;
  const { root, check, raw } = options;

  const commandArgs = ['fmt'];

  if (root) {
    commandArgs.push('--root', `"${root}"`);
  }

  if (check) {
    commandArgs.push('--check');
  }

  if (raw) {
    commandArgs.push('--raw');
  }

  if (files) {
    commandArgs.push(...files.map((file) => (file.includes(' ') ? `"${file}"` : file)));
  } else if (content) {
    commandArgs.push('-');
  }

  const command = `forge ${commandArgs.join(' ')}`;

  if (debug) {
    console.debug('command =>', command);
  }

  return new Promise((resolve, reject) => {
    const child = exec(command, (error, stdout, _stderr) => {
      if (error && !check) {
        reject(error);
      } else {
        resolve({
          exitCode: 0,
          output: stdout,
        });
      }
    });

    if (content && child.stdin) {
      child.stdin.write(content);
      child.stdin.end();
    }
  });
}

function format() {
  if (!isFmtInstalled()) {
    vscode.window.showErrorMessage('Forge fmt is not installed. Please install it and try again.');
    return;
  }

  // Get the active text editor
  const editor = vscode.window.activeTextEditor;

  if (editor) {
    const document = editor.document;

    if (document.languageId !== 'solidity' || editor.document.fileName.split('.').pop() !== 'sol') {
      vscode.window.showErrorMessage('Forge fmt is only available for solidity files.');
      return;
    }

    const options: ForgeFmtOptions = {
      root: vscode.workspace.workspaceFolders?.[0].uri.fsPath,
      check: false,
      raw: false,
    };

    const args: ForgeFmtArgs = {
      options,
      content: document.getText(),
    };

    forgeFmt(args)
      .then((result) => {
        if (result.exitCode === 0) {
          vscode.window.showInformationMessage('Forge fmt ran successfully.');
        } else {
          vscode.window.showErrorMessage('Forge fmt failed. Please check the output for details.');
        }
      })
      .catch((error) => {
        vscode.window.showErrorMessage('Forge fmt failed. Please check the output for details.');
        console.error(error);
      });
  }
}

function registerForgeFmtLinter(context: vscode.ExtensionContext): {
  fileDisposable: Disposable;
  workspaceDisposable: Disposable;
  formatterDisposable: Disposable;
} {
  const lintSolFile = vscode.commands.registerCommand('osmium.format-sol-file', format);

  const lintSolWorkspace = vscode.commands.registerCommand('osmium.format-sol-workspace', function () {
    if (!isFmtInstalled()) {
      vscode.window.showErrorMessage('Forge fmt is not installed. Please install it and try again.');
      return;
    }

    if (!vscode.workspace.workspaceFolders?.[0]) {
      vscode.window.showErrorMessage('Unable to find workspace root. Please open a folder and try again.');
      return;
    }

    const options: ForgeFmtOptions = {
      root: vscode.workspace.workspaceFolders?.[0].uri.fsPath,
      check: false,
      raw: false,
    };

    const args: ForgeFmtArgs = {
      options,
      files: [vscode.workspace.workspaceFolders?.[0].uri.fsPath],
    };

    forgeFmt(args)
      .then((result) => {
        if (result.exitCode !== 0) {
          vscode.window.showErrorMessage('Forge fmt failed. Please check the output for details.');

          console.log(result.output);
        }
      })
      .catch((error) => {
        vscode.window.showErrorMessage('Forge fmt failed. Please check the output for details.');
        console.error(error);
      });
  });

  const formatter = vscode.languages.registerDocumentFormattingEditProvider('solidity', {
    provideDocumentFormattingEdits: async (document) => {
      if (!isFmtInstalled()) {
        vscode.window.showErrorMessage('Forge fmt is not installed. Please install it and try again.');
        return [];
      }

      const options: ForgeFmtOptions = {
        root: vscode.workspace.workspaceFolders?.[0].uri.fsPath,
        check: true,
        raw: true,
      };

      const args: ForgeFmtArgs = {
        options,
        content: document.getText(),
      };

      try {
        const res = await forgeFmt(args);
        const fullRange = new vscode.Range(document.positionAt(0), document.positionAt(document.getText().length));

        return [vscode.TextEdit.replace(fullRange, res.output)];
      } catch (error) {
        vscode.window.showErrorMessage('Forge fmt failed. Please check the output for details.');
        console.error(error);
      }

      return [];
    },
  });

  context.subscriptions.push(lintSolFile);
  context.subscriptions.push(lintSolWorkspace);
  context.subscriptions.push(formatter);

  return { fileDisposable: lintSolFile, workspaceDisposable: lintSolWorkspace, formatterDisposable: formatter };
}

export default registerForgeFmtLinter;
export { format };
