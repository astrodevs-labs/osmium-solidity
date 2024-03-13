import { execSync, exec } from "child_process";
import * as vscode from "vscode";

type GasReport = {
  average: bigint;
  min?: bigint;
  max?: bigint;
  median?: bigint;
};

type Function = {
  name: string,
  line: number
};

// the file path is the key, the value is a map of contract name and the value is a map of function name and the value is the gas report
type Report = Map<string, Map<string, GasReport>>;

type ReportDecorators = Map<string, vscode.DecorationOptions[]>;

function isForgeInstalled(): boolean {
  try {
    execSync("forge --version");
    return true;
  } catch (error) {
    return false;
  }
}

// compute the decorations to send based on forge test --gas-report
async function gasReportTests(cwd: string): Promise<ReportDecorators> {
  const reports: Map<string, Report> = new Map();
  let decorations: ReportDecorators = new Map();

  // Gas estimation from the tests
  await new Promise<void>((resolve, reject) => exec("forge test --gas-report", { cwd }, async (error: any, _stdout: any, _stderr: any) => {
    if (error) {
      console.log("error", error);
      reject(error);
    }

     if (_stdout === "null") {
        resolve();
      }

    // pqrse the forge test --gas-report output to find contracts and functions
    let contractName = "";
    await Promise.all(_stdout.split("\n").map(async (line: string) => {
      const lineParts = line.split("|");
      if (lineParts.length === 8) {
        const trimmedLineParts = lineParts.map((part) => part.trim());
        if (trimmedLineParts[1] !== "" && trimmedLineParts[2] === "" && trimmedLineParts[3] === "" && trimmedLineParts[4] === "" && trimmedLineParts[5] === "" && trimmedLineParts[6] === "") {
          contractName = trimmedLineParts[1].split(" ")[0];
        }

        if (trimmedLineParts[1] !== "" && trimmedLineParts[2] !== "" && trimmedLineParts[3] !== "" && trimmedLineParts[4] !== "" && trimmedLineParts[5] !== "" && trimmedLineParts[6] !== ""
          && !trimmedLineParts[1].split("").every(char => char === '-') && !trimmedLineParts[2].split("").every(char => char === '-') && !trimmedLineParts[3].split("").every(char => char === '-') && !trimmedLineParts[4].split("").every(char => char === '-') && !trimmedLineParts[5].split("").every(char => char === '-') && !trimmedLineParts[6].split("").every(char => char === '-')
          && trimmedLineParts[1] !== "Function Name") {

          const functionName = trimmedLineParts[1];
          const min = BigInt(trimmedLineParts[2]);
          const average = BigInt(trimmedLineParts[3]);
          const median = BigInt(trimmedLineParts[4]);
          const max = BigInt(trimmedLineParts[5]);

          const splittedContractName = contractName.split(":");
          const totalPath = `${cwd}/${splittedContractName[0]}`;
          if (!reports.has(totalPath)) {
            reports.set(totalPath, new Map());
          }
          if (reports.get(totalPath)?.has(splittedContractName[1])) {
            reports.get(totalPath)?.get(splittedContractName[1])?.set(functionName, { min, average, median, max });
          } else {
            reports.get(totalPath)?.set(splittedContractName[1], new Map());
            reports.get(totalPath)?.get(splittedContractName[1])?.set(functionName, { min, average, median, max });
          }
        }
      }
    }));
    resolve();
  }));

  // Go through the reports and create the decorations
  for (const [path, report] of reports) {
    let decorationsArray: vscode.DecorationOptions[] = [];
    const content = await vscode.workspace.fs.readFile(vscode.Uri.file(path));

    for (const [contract, _] of report) {
      const functionsInsideContract = getFunctionsInsideContract(content.toString(), contract);
      for (const func of functionsInsideContract) {
        const gas = report.get(contract)?.get(func.name)?.average;
        if (!gas) {
          continue;
        }

        let range = new vscode.Range(
          new vscode.Position(func.line - 1, 0),
          new vscode.Position(func.line - 1, content.toString().split("\n")[func.line - 1].length)
        );
        let decoration = { range, renderOptions: { after: { contentText: `    ${gas.toString()} gas` } }  };
        decorationsArray.push(decoration);
      }
    }
    decorations.set(path, decorationsArray);
  }

  return decorations;
}

// Contracts needs to be formatted like this : ["contract1.sol:Contract1", "contract2.sol:Contract2"]
async function getGasReport(contracts: string[], cwd: string): Promise<Report> {
  const report: Report = new Map();

  // Gas estimation from the contracts inspection
  await Promise.all(contracts.map(async (contract) => {
    return await new Promise<void>((resolve, reject) => {
      exec(`forge inspect "${contract}" gasEstimates`, { cwd }, (error: any, _stdout: any, _stderr: any) => {
      if (error) {
        console.log("error", error);
        reject(error);
      }

      if (_stdout === "null") {
        resolve();
      }

      const json = JSON.parse(_stdout);
      const internalFunctions = Object.keys(json.internal);
      const externalFunctions = Object.keys(json.external);

      // Go through the internal and external functions and create the report
      internalFunctions.forEach((functionName) => {
        const cleanFunctionName = functionName.split("(")[0];
        const res: string = json.internal[functionName];
        if (res !== "infinite") {
          if (report.has(contract)) {
            report.get(contract)?.set(cleanFunctionName, { average: BigInt(res) });
          } else {
            report.set(contract, new Map());
            report.get(contract)?.set(cleanFunctionName, { average: BigInt(res) });
          }
        }
      });
      externalFunctions.forEach((functionName) => {
        const cleanFunctionName = functionName.split("(")[0];
        const res: string = json.external[functionName];
        if (res !== "infinite") {
          if (report.has(contract)) {
            report.get(contract)?.set(cleanFunctionName, { average: BigInt(res) });
          } else {
            report.set(contract, new Map());
            report.get(contract)?.set(cleanFunctionName, { average: BigInt(res) });
          }
        }
      });
      resolve();
    });
  });
  }));
  return report;
}

function getXthWord(line: string, index: number): string {
  return line.split(" ").filter((str) => (!!str.length)).at(index) || "";
}

function getContractsInsideFile(content: string, path: string): string[] {
  const contracts: string[] = [];
  const lines = content.split("\n");

  lines.forEach((line) => {
    if (getXthWord(line, 0) === "contract") {
      const contractName = getXthWord(line, 1);
      contracts.push(`${path}:${contractName}`);
    } else if (getXthWord(line, 0) === "abstract" && getXthWord(line, 1) === "contract") {
      const contractName = getXthWord(line, 2);
      contracts.push(`${path}:${contractName}`);
    }
  });
  return contracts;
}

// Get all the functions and abstract functions inside a contract with their line number
function getFunctionsInsideContract(content: string, contractName: string): Function[] {
  const functions: Function[] = [];
  const lines = content.split("\n");

  let start = false;
  let bracketsCount = 0;
  let currentContractName = "";
  lines.forEach((line, index) => {
    const firstWord = getXthWord(line, 0);
    const secondWord = getXthWord(line, 1);
    if (firstWord === "contract") {
      currentContractName = secondWord;
      if (contractName === currentContractName) {
        start = true;
      }
    }
    if (start) {
      bracketsCount += (line.split("{").length - 1) - (line.split("}").length - 1);
      if (bracketsCount === -1) {
        return functions;
      }
      if (firstWord === "function") {
        const functionName = secondWord.split("(")[0];
        functions.push({
          name: functionName,
          line: index + 1
        });
      }
    }
  });

  return functions;
}

// compute the decorations to send based on forge inspection
async function gasReport(content: string, path: string): Promise<vscode.DecorationOptions[]> {
  const workspace = vscode.workspace.workspaceFolders?.[0];
  const workspacePath = workspace?.uri.path;
  if (!workspacePath) {
    return [];
  }
  const contracts = getContractsInsideFile(content, path);
  const report = await getGasReport(contracts, workspacePath);

  const functionsPerContract: Map<string, Function[]> = new Map();
  contracts.map((contract) => {
    const functions = getFunctionsInsideContract(content, contract.split(":")[1]);
    functionsPerContract.set(contract, functions);
  });

  let decorationsArray: vscode.DecorationOptions[] = [];
  for (const [contract, functions] of functionsPerContract) {
    for (const func of functions) {
      const gas = report.get(contract)?.get(func.name)?.average;
      if (!gas) {
        continue;
      }

      let range = new vscode.Range(
        new vscode.Position(func.line - 1, 0),
        new vscode.Position(func.line - 1, content.split("\n")[func.line - 1].length)
      );

      let decoration = { range, renderOptions: { after: { contentText: `    ${gas.toString()} gas` } }  };
      decorationsArray.push(decoration);
    }
  }
  return decorationsArray;
}

// Send the decorations to the editor
async function showReport(editor: vscode.TextEditor, reports: ReportDecorators, reportsSaved: ReportDecorators, decorationType: vscode.TextEditorDecorationType) {
  let report = reports.get(editor.document.uri.path);
    const reportSaved = reportsSaved.get(editor.document.uri.path);

    if (!report && !reportSaved) {
    } else if (report && !reportSaved) {
      editor.setDecorations(decorationType, report);
    } else if (!report && reportSaved) {
      editor.setDecorations(decorationType, reportSaved);
    } else if (report && reportSaved) {
      report = report.filter((reportElement) => {
        return !reportSaved.some((reportSavedElement) => {
          return reportElement.range.isEqual(reportSavedElement.range);
        });
      });
      const combinedReport = [...(report || []), ...(reportSaved || [])];
      editor.setDecorations(decorationType, combinedReport);
    }
}

export function registerGasEstimation() {
  const forgeInstalled =  isForgeInstalled();

  const decorationType = vscode.window.createTextEditorDecorationType({
    after: {
      color: 'rgba(255, 255, 255, 0.5)'
    }
  });

  const reports: ReportDecorators  = new Map();
  let reportsSaved: ReportDecorators  = new Map();

  // Generate the report when the file is opened or saved
  vscode.workspace.onDidOpenTextDocument(async (document) => {
    // gas estimate only the main contracts
    const workspacePath = vscode.workspace.workspaceFolders?.[0].uri.path;
    if (!workspacePath) {
      return;
    }
    const cleanPath = document.uri.path.replace(workspacePath, "");
    if (cleanPath.includes("lib") || cleanPath.includes("test") || cleanPath.includes("script") || cleanPath.includes(".git") || !forgeInstalled) {
      return;
    }
    const report = await gasReport(document.getText(), document.uri.path);
    reports.set(document.uri.path, report);

    vscode.window.visibleTextEditors.forEach((editor) => {
      showReport(editor, reports, reportsSaved, decorationType);
    });
  });
  vscode.workspace.onDidSaveTextDocument(async (document) => {
    // gas estimate only the main contracts
    const workspacePath = vscode.workspace.workspaceFolders?.[0].uri.path;
    if (!workspacePath) {
      return;
    }
    const cleanPath = document.uri.path.replace(workspacePath, "");
    if (cleanPath.includes("lib") || cleanPath.includes("test") || cleanPath.includes("script") || cleanPath.includes(".git") || !forgeInstalled) {
      return;
    }
    const report = await gasReport(document.getText(), document.uri.path);
    reports.set(document.uri.path, report);

    vscode.window.visibleTextEditors.forEach((editor) => {
      showReport(editor, reports, reportsSaved, decorationType);
    });
  });

  // Show reports when the editor is changed
  vscode.window.onDidChangeVisibleTextEditors(async (editors) => {
    editors.forEach((editor) => {
      showReport(editor, reports, reportsSaved, decorationType);
    });
  });
  vscode.window.onDidChangeActiveTextEditor(async (editor) => {
    if (editor) {
      showReport(editor, reports, reportsSaved, decorationType);
    }
  });

  vscode.commands.registerCommand(
    "osmium.gas-estimation",
    async function () {
      if (vscode.workspace.workspaceFolders?.[0].uri.fsPath) {
        const report = await gasReportTests(vscode.workspace.workspaceFolders?.[0].uri.fsPath);
        reportsSaved = report;
      }
    }
  );
}
