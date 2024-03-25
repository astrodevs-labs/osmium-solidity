import { exec } from "child_process";
import * as path from "path";
import * as toml from "toml";
import { workspace } from "vscode";
import { DeployContracts, DeployScript } from "./types";

async function getScriptFolder(): Promise<string> {
  try {
    const foundryConfigContent = await workspace.fs.readFile(
      workspace.workspaceFolders![0].uri.with({
        path: path.join(
          workspace.workspaceFolders![0].uri.path,
          "foundry.toml",
        ),
      }),
    );
    const parsedFoundryConfig: any = toml.parse(
      foundryConfigContent.toString(),
    );
    return parsedFoundryConfig.script ?? "script";
  } catch (error) {
    console.error("Error reading foundry.toml file:", error);
    return "not found";
  }
}

async function getContractFolder(): Promise<string> {
  try {
    const foundryConfigContent = await workspace.fs.readFile(
      workspace.workspaceFolders![0].uri.with({
        path: path.join(
          workspace.workspaceFolders![0].uri.path,
          "foundry.toml",
        ),
      }),
    );
    const parsedFoundryConfig: any = toml.parse(
      foundryConfigContent.toString(),
    );
    return parsedFoundryConfig.contract ?? "src";
  } catch (error) {
    console.error("Error reading foundry.toml file:", error);
    return "not found";
  }
}

async function getOutFolder(): Promise<string> {
  try {
    const foundryConfigContent = await workspace.fs.readFile(
      workspace.workspaceFolders![0].uri.with({
        path: path.join(
          workspace.workspaceFolders![0].uri.path,
          "foundry.toml",
        ),
      }),
    );
    const parsedFoundryConfig: any = toml.parse(
      foundryConfigContent.toString(),
    );
    return parsedFoundryConfig.out ?? "out";
  } catch (error) {
    console.error("Error reading foundry.toml file:", error);
    return "not found";
  }
}

async function getAbiFile(
  workspacePath: string,
  scriptFile: string,
  outFolder: string,
): Promise<string> {
  const abiFilePath = path.join(
    workspacePath,
    outFolder,
    scriptFile + ".sol",
    `${scriptFile}.json`,
  );
  try {
    const abiFileBuffer = await workspace.fs.readFile(
      workspace.workspaceFolders![0].uri.with({ path: abiFilePath }),
    );
    const abiFileContent = Buffer.from(abiFileBuffer).toString("utf-8");
    return abiFileContent;
  } catch (error) {
    return "{}";
  }
}

export async function getContracts(): Promise<DeployContracts[]> {
  try {
    const contracts: DeployContracts[] = [];
    const contractFolder = await getContractFolder();
    const contractFiles = await workspace.findFiles(
      `**/${contractFolder}/*.sol`,
    );
    const filteredContractFiles = contractFiles.filter((file) => {
      const parts = file.path.split("/");
      let srcIndex = parts.indexOf(contractFolder);
      let forgeStdIndex = parts.indexOf("forge-std");
      if (forgeStdIndex > -1 && srcIndex > forgeStdIndex) {
        return false;
      }
      return true;
    });

    const outFolder = await getOutFolder();
    const workspacePath = contractFiles[0].path
      .split("/")
      .slice(0, -2)
      .join("/");
    for (const contractFile of filteredContractFiles) {
      const contractContentBuffer = await workspace.fs.readFile(contractFile);
      const contractContent = Buffer.from(contractContentBuffer).toString(
        "utf-8",
      );
      const contractNameRegex = /contract\s+(\w+)/g;
      let contractNameMatch = contractNameRegex.exec(contractContent) || [];
      while (contractNameMatch.length !== 0) {
        const contractName = path.basename(contractFile.path, ".sol");
        const abi = await getAbiFile(workspacePath, contractName, outFolder);
        const contract = {
          name: contractNameMatch[1],
          path: path.basename(contractFile.path),
          abi: JSON.parse(abi).abi,
        };
        contracts.push(contract);
        contractNameMatch = contractNameRegex.exec(contractContent) || [];
      }
    }
    return contracts;
  } catch (e) {
    console.error("Error getting contracts", e);
    return [];
  }
}

export async function getScripts(): Promise<DeployScript[]> {
  try {
    const scripts: DeployScript[] = [];
    const scriptFolder = await getScriptFolder();
    const scriptFiles = await workspace.findFiles(`**/${scriptFolder}/*.s.sol`);
    const outFolder = await getOutFolder();
    const workspacePath = scriptFiles[0].path.split("/").slice(0, -2).join("/");
    for (const scriptFile of scriptFiles) {
      const scriptContentBuffer = await workspace.fs.readFile(scriptFile);
      const scriptContent = Buffer.from(scriptContentBuffer).toString("utf-8");
      const contractNameRegex = /contract\s+(\w+)\s+is\s+Script/g;
      let scriptNameMatch = contractNameRegex.exec(scriptContent) || [];
      while (scriptNameMatch.length !== 0) {
        const fileName = path.basename(scriptFile.path, ".s.sol");
        const abi = await getAbiFile(workspacePath, fileName, outFolder);
        const script = {
          name: scriptNameMatch[1],
          path: path.basename(scriptFile.path),
          abi: JSON.parse(abi).abi,
        };
        scripts.push(script);
        scriptNameMatch = contractNameRegex.exec(scriptContent) || [];
      }
    }
    return scripts;
  } catch (e) {
    console.error("Error getting scripts", e);
    return [];
  }
}

export async function deployContract(
  rpcUrl: string,
  contract: DeployContracts,
  verify: boolean,
  cstrArgs: string[],
): Promise<void> {
  const verifyStr = verify ? "--verify" : "";
  exec(
    `forge create ${contract.path}:${contract.name} --rpc-url ${rpcUrl} ${verifyStr} --contructor-args ${cstrArgs.join(" ")}`,
    (error, _stdout, _stderr) => {
      if (error) {
        throw error;
      }
    },
  );
}

export async function deployScript(
  rpcUrl: string,
  script: DeployScript,
  verify: boolean,
): Promise<void> {
  const verifyStr = verify ? "--verify" : "";
  exec(
    `forge script ${script.path}:${script.name} --rpc-url ${rpcUrl} --broadcast ${verifyStr}`,
    (error, _stdout, _stderr) => {
      if (error) {
        throw error;
      }
    },
  );
}
