import { LanguageClient } from "vscode-languageclient/node";
import * as vscode from "vscode";
import { testContract, testFunction, FileResult } from "./foundry-test";

enum ItemType {
  file,
  contractCase,
  testCase,
}

export class TestManager {
  public testController: vscode.TestController;
  private testData = new WeakMap<vscode.TestItem, ItemType>();

  constructor(private client: LanguageClient, private workspace: string) {
    this.testController = vscode.tests.createTestController(
      "solidityTestController",
      "Solidity test controller"
    );

    this.testController.resolveHandler = (test) => {
      console.log("controller resolve");
      return this.resolve(test);
    };
    this.testController.createRunProfile(
      "Run tests",
      vscode.TestRunProfileKind.Run,
      (request, token) => this.runHandler(false, request, token)
    );
    // Uncomment this when debugging is supported
    //this.testController.createRunProfile("Debug tests", vscode.TestRunProfileKind.Run, (request, token) => this.runHandler(true, request, token))

    vscode.workspace.onDidOpenTextDocument((e) => {
      this.parseTestsInDocument(e);
    });

    console.log("Test manager created");
  }

  /**
   *
   * @param _shouldDebug Whether the tests should be run in debug mode
   * @param request The TestRunRequest containing the tests to run
   * @param token A cancellation token
   */
  private async runHandler(
    _shouldDebug: boolean,
    request: vscode.TestRunRequest,
    token: vscode.CancellationToken
  ) {
    console.log("Run handler called");
    const run = this.testController.createTestRun(request);
    const queue: vscode.TestItem[] = [];

    // Loop through all included tests, or all known tests, and add them to our queue
    if (request.include) {
      console.log("request include", request.include);
      request.include.forEach((test) => queue.push(test));
    } else {
      console.log("testAll");
      this.testController.items.forEach((test) => queue.push(test));
    }

    // For every test that was queued, try to run it. Call run.passed() or run.failed().
    // The `TestMessage` can contain extra information, like a failing location or
    // a diff output. But here we'll just give it a textual message.
    while (queue.length > 0 && !token.isCancellationRequested) {
      const test = queue.pop()!;

      // Skip tests the user asked to exclude
      if (request.exclude?.includes(test)) {
        continue;
      }

      const date = Date.now();
      try {
        switch (this.testData.get(test)!) {
          case ItemType.file:
            // If we're running a file and don't know what it contains yet, parse it now
            if (test.children.size === 0) {
              await this.parseTestsInFileContents(test);
            }
            break;
          case ItemType.contractCase:
            //get result form foundry wrapper for contract test
            const contractResult = await testContract(
              this.workspace,
              test.label
            );
            const contractTime = Date.now() - date;
            if (this.analyzeTestResults(contractResult)) {
              run.appendOutput(
                this.extractResultLogs(contractResult).join("\r\n")
              );
              run.passed(test, contractTime);
            } else {
              run.failed(
                test,
                new vscode.TestMessage(
                  `Contract test failed\n\n${this.extractResultLogs(contractResult).join(
                    "\n"
                  )}`
                ),
                contractTime
              );
            }
            break;
          case ItemType.testCase:
            //get result form foundry wrapper for test case
            const functionResult = await testFunction(
              this.workspace,
              test.parent!.label,
              test.label
            );
            const functionTime = Date.now() - date;
            
            if (this.analyzeTestResults(functionResult)) {
              run.appendOutput(this.extractResultLogs(functionResult).join("\r\n"));
              run.passed(test, functionTime);
            } else {
              run.failed(
                test,
                new vscode.TestMessage(`Test failed\n\n${this.extractResultLogs(functionResult).join("\n")}`),
                functionTime
              );
            }
            break;
        }
      } catch (e: any) {
        run.appendOutput(JSON.stringify(e));
        run.failed(test, new vscode.TestMessage("Test failed"));
        if (e === "No forge found") {
          vscode.window.showErrorMessage(
            "No forge found. Please install forge and make sure it's in your PATH"
          );
        }
      }

      // If the test type is a file, we'll queue up all of its children (contracts) to run next.
      // Otherwise, we do nothing as the highest level (contracts) already include their children (test cases).
      if (this.testData.get(test) === ItemType.file) {
        test.children.forEach((test) => queue.push(test));
      }
    }

    // Make sure to end the run after all tests have been executed:
    run.end();
  }

  private analyzeTestResults(result: FileResult) {
    let ret = true;

    for (const suiteResult of Object.values(result)) {
      for (const testResult of Object.values(suiteResult.test_results)) {
        if (testResult.status !== "Success") {
          return false;
        }
      }
    }
    return true;
  }

  private extractResultLogs(result: FileResult) {
    let logs: string[] = [];

    for (const suiteResult of Object.values(result)) {
      for (const testResult of Object.values(suiteResult.test_results)) {
        logs = logs.concat(testResult.decoded_logs);
      }
    }

    return logs;
  }

  /**
   * Sends a request to the language server to get the positions of all tests in a file
   * @param content The content of the file to parse
   * @returns A structure containing the positions of all tests in the file (see /toolchains/solidity/core/tests-positions-server/src/get-tests-positions.rs)
   */
  private async getTestsPositions(content: string): Promise<any> {
    return this.client.sendRequest("osmium/getTestsPositions", {
      file_content: content, // eslint-disable-line @typescript-eslint/naming-convention
    });
  }

  /**
   * Check if a TestItem for a file already exists in the testController, and if not, create it
   * @param uri URI of the file to get or create a TestItem for
   * @returns The TestItem for the file
   */
  private getOrCreateTestFileItem(uri: vscode.Uri) {
    const existing = this.testController.items.get(uri.toString());
    if (existing) {
      return existing;
    }

    const file = this.testController.createTestItem(
      uri.toString(),
      uri.path.split("/").pop()!,
      uri
    );
    this.testData.set(file, ItemType.file);
    file.canResolveChildren = true;
    this.testController.items.add(file);
    return file;
  }

  /**
   * Resolve a TestItem. If it's a file, parse it for tests. If it's a contract, parse it for tests and add them as children
   * @param test The TestItem to resolve
   */
  private async resolve(test?: vscode.TestItem) {
    if (!test) {
      await this.discoverAllFilesInWorkspace();
    } else {
      await this.parseTestsInFileContents(test);
    }
  }

  /**
   * Discover all files in the workspace and add them to the testController
   * Also create a FileSystemWatcher for each file to watch for changes
   */
  private async discoverAllFilesInWorkspace() {
    if (!vscode.workspace.workspaceFolders) {
      return []; // handle the case of no open folders
    }

    return Promise.all(
      vscode.workspace.workspaceFolders.map(async (workspaceFolder) => {
        const pattern = new vscode.RelativePattern(
          workspaceFolder,
          "**/*.t.sol"
        );
        const watcher = vscode.workspace.createFileSystemWatcher(pattern);

        // When files are created, make sure there's a corresponding "file" node in the tree
        watcher.onDidCreate((uri) => this.getOrCreateTestFileItem(uri));
        // When files change, re-parse them. Note that you could optimize this so
        // that you only re-parse children that have been resolved in the past.
        watcher.onDidChange((uri) =>
          this.parseTestsInFileContents(this.getOrCreateTestFileItem(uri))
        );
        // And, finally, delete TestItems for removed files. This is simple, since
        // we use the URI as the TestItem's ID.
        watcher.onDidDelete((uri) =>
          this.testController.items.delete(uri.toString())
        );

        for (const file of await vscode.workspace.findFiles(pattern)) {
          this.getOrCreateTestFileItem(file);
        }

        return watcher;
      })
    );
  }

  /**
   * Check if the document is a test file and parse it if it is
   * @param e TextDocument that was opened
   */
  private parseTestsInDocument(e: vscode.TextDocument) {
    if (e.uri.scheme === "file" && e.uri.path.endsWith(".t.sol")) {
      this.parseTestsInFileContents(
        this.getOrCreateTestFileItem(e.uri),
        e.getText()
      );
    }
  }

  /**
   * Read the contents of a file and parse it for tests by calling the tests-positions language server method. It will then fill the children of the TestItem with the tests found.
   * @param file A TestItem representing the file to parse
   * @param contents The contents of the file. If not provided, the file will be read from disk
   */
  private async parseTestsInFileContents(
    file: vscode.TestItem,
    contents?: string
  ) {
    // If a document is open, VS Code already knows its contents. If this is being
    // called from the resolveHandler when a document isn't open, we'll need to
    // read them from disk ourselves.
    if (contents === undefined) {
      const rawContent = await vscode.workspace.fs.readFile(file.uri!);
      contents = new TextDecoder().decode(rawContent);
    }

    if (contents !== undefined) {
      // CALL getTestPositions and fill children
      await this.getTestsPositions(contents)
        .then((testPositions) => {
          testPositions.contracts.forEach((contract: any) => {
            const contractName = contract.name.replace(" ", "");
            const contractItem = this.testController.createTestItem(
              contractName,
              contract.name,
              file.uri
            );
            contractItem.range = convertRange(contract.range);
            this.testData.set(contractItem, ItemType.contractCase);
            file.children.add(contractItem);

            contract.tests.forEach((test: any) => {
              const functionItem = this.testController.createTestItem(
                `${contractName}_${test.name}`,
                test.name,
                file.uri
              );
              functionItem.range = convertRange(test.range);
              this.testData.set(functionItem, ItemType.testCase);
              contractItem.children.add(functionItem);
            });
          });
        })
        .catch((error) => {
          console.log("Error getting tests positions", error);
          vscode.window.showErrorMessage("Error while getting tests positions");
        });
    }
  }
}

/**
 * Convert a LSP range to a VSCode range (offsets are 0-based in VScode and 1-based in LSP)
 * @param lspRange LSP range
 * @returns A VSCode range with the same start and end positions
 */
function convertRange(lspRange: any): vscode.Range {
  const range = new vscode.Range(
    new vscode.Position(lspRange.start.line - 1, lspRange.start.character),
    new vscode.Position(lspRange.end.line - 1, lspRange.end.character)
  );
  return range;
}
