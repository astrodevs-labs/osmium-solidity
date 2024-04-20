import { InteractContractRepository } from './InteractContractRepository';
import { WalletRepository } from './WalletRepository';
import { ScriptRepository } from './ScriptRepository';
import { Address } from 'viem';
import { ContractParams, Script } from './types';
import { EnvironmentRepository } from './EnvironmentRepository';
import { exec } from 'child_process';
import { DeployContractRepository } from './DeployContractRepository';
import fs from 'fs';
import { getTomlValue } from '../utils';
import path from 'path';

export interface DeployScriptOptions {
  environmentId: string;
  scriptId: string;
  verify: boolean;
}

export interface DeployContractOptions {
  environmentId: string;
  contractId: string;
  walletId: string;
  verify: boolean;
  gasLimit: number;
  value: number;
  params: ContractParams[];
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

  public async deployScript({ environmentId, scriptId, verify }: DeployScriptOptions): Promise<any> {
    const environmentInfos = this._environmentRepository.getEnvironment(environmentId);
    const scriptInfos = this._scriptRepository.getScript(scriptId);

    if (!environmentInfos) {
      throw new Error(`environment id ${environmentId} not found`);
    }

    if (!scriptInfos) {
      throw new Error(`script id ${scriptId} not found`);
    }

    const command = `forge script ${path.join(this._scriptFolderPath, scriptInfos.path)}:${scriptInfos.name} --rpc-url ${environmentInfos.rpc} ${verify ?? '--verify'}`;

    return new Promise((resolve, reject) => {
      exec(command, { cwd: this._projectPath }, (error, stdout, _stderr) => {
        if (error) {
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

  public async deployContract({
    contractId,
    environmentId,
    walletId,
    gasLimit,
    value,
    params,
    verify,
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
      `${contractInfos.path}:${contractInfos.name}`,
      '--private-key',
      walletInfos.privateKey,
      '--rpc-url',
      environmentInfos.rpc,
      '--value',
      value.toString(),
      '--contructor-args',
      ...params,
    ];

    if (gasLimit) {
      command.push('--gas-limit', gasLimit.toString());
    }

    if (verify) {
      command.push('--verify');
    }

    return new Promise((resolve, reject) => {
      exec(command.join(' '), (error, stdout, _stderr) => {
        if (error) {
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
}
