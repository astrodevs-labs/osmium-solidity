import { ContractRepository } from './ContractRepository';
import { WalletRepository } from './WalletRepository';
import { ScriptRepository } from './ScriptRepository';
import { Address } from 'viem';
import { Script } from './types';
import { EnvironmentRepository } from './EnvironmentRepository';
import { exec } from 'child_process';

export interface DeployScriptOptions {
  environmentId: string;
  scriptId: string;
  verify: boolean;
}

export class Deploy {
  private _contractRepository: ContractRepository;
  private _walletRepository: WalletRepository;
  private _scriptRepository: ScriptRepository;
  private _environmentRepository: EnvironmentRepository;

  constructor(
    contractRepository: ContractRepository,
    walletRepository: WalletRepository,
    scriptRepository: ScriptRepository,
    environmentRepository: EnvironmentRepository,
  ) {
    this._contractRepository = contractRepository;
    this._walletRepository = walletRepository;
    this._scriptRepository = scriptRepository;
    this._environmentRepository = environmentRepository;
  }

  public async deployScript({ environmentId, scriptId, verify }: DeployScriptOptions): Promise<void> {
    const environmentInfos = this._environmentRepository.getEnvironment(environmentId);
    const scriptInfos = this._scriptRepository.getScript(scriptId);

    if (!environmentInfos) {
      throw new Error(`environment id ${environmentId} not found`);
    }

    if (!scriptInfos) {
      throw new Error(`script id ${scriptId} not found`);
    }

    const command = `forge script ${scriptInfos.path}:${scriptInfos.name} --rpc-url ${environmentInfos.rpc} ${verify ?? '--verify'}`;

    exec(command, (error, _stdout, _stderr) => {
      if (error) {
        throw error;
      }
    });
  }
}
