import { exec } from 'child_process';
import fs from 'fs';
import path from 'path';
import * as vscode from 'vscode';
import { getTomlValue } from '../utils';
import { DeployContractRepository } from './DeployContractRepository';
import { EnvironmentRepository } from './EnvironmentRepository';
import { ScriptRepository } from './ScriptRepository';
import { WalletRepository } from './WalletRepository';
import { ContractParams } from './types';

export interface DeployScriptOptions {
  environmentId: string;
  scriptId: string;
  verify: boolean;
  outputChannel : vscode.OutputChannel;
}

export interface DeployContractOptions {
  environmentId: string;
  contractId: string;
  walletId: string;
  verify: boolean;
  gasLimit: number;
  value: number;
  params: ContractParams[];
  outputChannel : vscode.OutputChannel;
}

export class Deploy {
  private _contractRepository: DeployContractRepository;
  private _walletRepository: WalletRepository;
  private _scriptRepository: ScriptRepository;
  private _environmentRepository: EnvironmentRepository;
  private readonly _scriptFolderPath: string;
  private readonly _projectPath: string;

  constructor(
    contractRepository: DeployContractRepository,
    walletRepository: WalletRepository,
    scriptRepository: ScriptRepository,
    environmentRepository: EnvironmentRepository,
    workspacePath: string,
  ) {
    this._contractRepository = contractRepository;
    this._walletRepository = walletRepository;
    this._scriptRepository = scriptRepository;
    this._environmentRepository = environmentRepository;

    this._projectPath = workspacePath;
    const foundryConfigPath = path.join(this._projectPath, 'foundry.toml');

    if (fs.existsSync(foundryConfigPath)) {
      const script = getTomlValue(foundryConfigPath, 'script');

      this._scriptFolderPath = script ? script : 'script';
    } else {
      this._scriptFolderPath = 'script';
    }
  }

  public async deployScript({ environmentId, scriptId, verify, outputChannel }: DeployScriptOptions): Promise<any> {
    const environmentInfos = this._environmentRepository.getEnvironment(environmentId);
    const scriptInfos = this._scriptRepository.getScript(scriptId);

    if (!environmentInfos) {
      throw new Error(`environment id ${environmentId} not found`);
    }

    if (!scriptInfos) {
      throw new Error(`script id ${scriptId} not found`);
    }

    const command = `forge script --broadcast ${path.join(this._scriptFolderPath, scriptInfos.path)}:${scriptInfos.name} --rpc-url ${environmentInfos.rpc} ${verify ? '--verify' : ''}`;

    return new Promise((resolve, reject) => {
      const childProcess = exec(command, { cwd: this._projectPath }, (error, stdout, stderr) => {
      if (error) {
        outputChannel.appendLine(`Error: ${error.message}`);
        resolve({
        exitCode: error.code,
        output: error.message,
        });
      } else {
        const printableData = stdout.replace(/[^\x20-\x7E]|(\[2m|\[0m|\[32m)/g, '');
        outputChannel.append(printableData);
        resolve({
        exitCode: 0,
        output: stdout,
        });
      }
      outputChannel.show();
      });

      childProcess.stdout?.on('data', (data) => {
        const printableData = data.replace(/[^\x20-\x7E]|(\[2m|\[0m|\[32m)/g, '');
        outputChannel.append(printableData);
      });

      childProcess.stderr?.on('data', (data) => {
        const printableData = data.replace(/[^\x20-\x7E]|(\[2m|\[0m|\[32m)/g, '');
        outputChannel.append(printableData);
      });
    });
  }

  public async deployContract({
    contractId,
    environmentId,
    walletId,
    gasLimit,
    value,
    params,
    verify,
    outputChannel
  }: DeployContractOptions): Promise<any> {
    const environmentInfos = this._environmentRepository.getEnvironment(environmentId);
    const contractInfos = this._contractRepository.getContract(contractId);
    const walletInfos = this._walletRepository.getWallet(walletId);

    if (!environmentInfos) {
      throw new Error(`environment id ${environmentId} not found`);
    }
    if (!contractInfos) {
      throw new Error(`contract id ${contractId} not found`);
    }
    if (!walletInfos) {
      throw new Error(`wallet id ${walletId} not found`);
    }

    const command = [
      'forge',
      'create',
      `"${contractInfos.path}:${contractInfos.name}"`,
      '--private-key',
      walletInfos.privateKey,
      '--rpc-url',
      environmentInfos.rpc,
      '--value',
      value.toString(),
    ];

    if (gasLimit) {
      command.push('--gas-limit', gasLimit.toString());
    }

    if (verify) {
      command.push('--verify');
    }

    if (params.length) {
      command.push(`--constructor-args ${params.join(' ')}`);
    }

    return new Promise((resolve, reject) => {
      const childProcess = exec(command.join(' '), { cwd: this._projectPath }, (error, stdout, stderr) => {
      if (error) {
        outputChannel.appendLine(`Error: ${error.message}`);
        resolve({
        exitCode: error.code,
        output: error.message,
        });
      } else {
        const printableData = stdout.replace(/[^\x20-\x7E]|(\[2m|\[0m|\[32m)/g, '');
        outputChannel.appendLine(printableData);
        resolve({
        exitCode: 0,
        output: stdout,
        });
      }
      outputChannel.show();
      });
      childProcess.stdout?.on('data', (data) => {
        const printableData = data.replace(/[^\x20-\x7E]|(\[2m|\[0m|\[32m)/g, '');
        outputChannel.append(printableData);
      });
      childProcess.stderr?.on('data', (data) => {
        const printableData = data.replace(/[^\x20-\x7E]|(\[2m|\[0m|\[32m)/g, '');
        outputChannel.append(printableData);
      });
    });
  }
}
