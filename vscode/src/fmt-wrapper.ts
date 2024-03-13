import { exec } from "child_process";
import * as vscode from "vscode";

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
  files: string[];
};

type ForgeFmtResult = {
  exitCode: number;
  output: string;
};

function isFmtInstalled(): boolean {
  try {
    exec("forge fmt --version", (error, _stdout, _stderr) => {
      if (error) {
        throw error;
      }
    });
    return true;
  } catch (error) {
    return false;
  }
}

function forgeFmt(
  args: ForgeFmtArgs,
  debug?: boolean
): Promise<ForgeFmtResult> {
  const { options, files } = args;
  const { root, check, raw } = options;

  const commandArgs = ["fmt"];

  if (root) {
    commandArgs.push("--root", `"${root}"`);
  }

  if (check) {
    commandArgs.push("--check");
  }

  if (raw) {
    commandArgs.push("--raw");
  }

  commandArgs.push(
    ...files.map((file) => (file.includes(" ") ? `"${file}"` : file))
  );

  const command = `forge ${commandArgs.join(" ")}`;

  if (debug) {
    console.debug("command =>", command);
  }

  return new Promise((resolve, reject) => {
    exec(command, (error, stdout, _stderr) => {
      if (error && !check) {
        reject(error);
      } else {
        resolve({
          exitCode: 0,
          output: stdout,
        });
      }
    });
  });
}

function registerForgeFmtLinter(context: vscode.ExtensionContext) {
  const lintSolFile = vscode.commands.registerCommand(
    "osmium.format-sol-file",
    function () {
      if (!isFmtInstalled()) {
        vscode.window.showErrorMessage(
          "Forge fmt is not installed. Please install it and try again."
        );
        return;
      }

      // Get the active text editor
      const editor = vscode.window.activeTextEditor;

      if (editor) {
        const document = editor.document;

        if (
          document.languageId !== "solidity" &&
          editor.document.fileName.split(".").pop() !== "sol"
        ) {
          vscode.window.showErrorMessage(
            "Forge fmt is only available for solidity files."
          );
          return;
        }

        const options: ForgeFmtOptions = {
          root: vscode.workspace.workspaceFolders?.[0].uri.fsPath,
          check: false,
          raw: false,
        };

        const args: ForgeFmtArgs = {
          options,
          files: [document.fileName],
        };

        forgeFmt(args)
          .then((result) => {
            if (result.exitCode === 0) {
              vscode.window.showInformationMessage(
                "Forge fmt ran successfully."
              );
            } else {
              vscode.window.showErrorMessage(
                "Forge fmt failed. Please check the output for details."
              );

              console.log(result.output);
            }
          })
          .catch((error) => {
            vscode.window.showErrorMessage(
              "Forge fmt failed. Please check the output for details."
            );
            console.error(error);
          });
      } else {
        vscode.window.showErrorMessage(
          "Forge fmt is only available for solidity files."
        );
      }
    }
  );

  const lintSolWorkspace = vscode.commands.registerCommand(
    "osmium.format-sol-workspace",
    function () {
      if (!isFmtInstalled()) {
        vscode.window.showErrorMessage(
          "Forge fmt is not installed. Please install it and try again."
        );
        return;
      }

      if (!vscode.workspace.workspaceFolders?.[0]) {
        vscode.window.showErrorMessage(
          "Unable to find workspace root. Please open a folder and try again."
        );
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
          if (result.exitCode === 0) {
            vscode.window.showInformationMessage("Forge fmt ran successfully.");
          } else {
            vscode.window.showErrorMessage(
              "Forge fmt failed. Please check the output for details."
            );

            console.log(result.output);
          }
        })
        .catch((error) => {
          vscode.window.showErrorMessage(
            "Forge fmt failed. Please check the output for details."
          );
          console.error(error);
        });
    }
  );

  const formatter = vscode.languages.registerDocumentFormattingEditProvider(
    "solidity",
    {
      provideDocumentFormattingEdits: (document) => {
        if (!isFmtInstalled()) {
          vscode.window.showErrorMessage(
            "Forge fmt is not installed. Please install it and try again."
          );
          return;
        }

        const options: ForgeFmtOptions = {
          root: vscode.workspace.workspaceFolders?.[0].uri.fsPath,
          check: false,
          raw: false,
        };

        const args: ForgeFmtArgs = {
          options,
          files: [document.fileName],
        };

        return forgeFmt(args).then((result) => {
          if (result.exitCode === 0) {
            vscode.window.showInformationMessage("Forge fmt ran successfully.");
          } else {
            vscode.window.showErrorMessage(
              "Forge fmt failed. Please check the output for details."
            );

            console.log(result.output);
          }

          return [];
        });
      },
    }
  );

  context.subscriptions.push(lintSolFile);
  context.subscriptions.push(lintSolWorkspace);

  context.subscriptions.push(formatter);
}

export default registerForgeFmtLinter;
