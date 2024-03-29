import { ContractRepository } from './ContractRepository';
import { WalletRepository } from './WalletRepository';
import { ScriptRepository } from './ScriptRepository';
import { Address } from 'viem';
import { Script } from './types';

export interface DeployScriptOptions {
  account: Address;
  script: Script;
  verify: boolean;
}

export class Deploy {
  private _contractRepository: ContractRepository;
  private _walletRepository: WalletRepository;
  private _scriptRepository: ScriptRepository;

  constructor(
    contractRepository: ContractRepository,
    walletRepository: WalletRepository,
    scriptRepository: ScriptRepository,
  ) {
    this._contractRepository = contractRepository;
    this._walletRepository = walletRepository;
    this._scriptRepository = scriptRepository;
  }

  public async deployScript({ account, script, verify }: DeployScriptOptions): Promise<void> {
    const walletInfos = this._walletRepository.getWallet(account);
    if (!walletInfos) {
      throw new Error(`wallet ${account} not found`);
    }

    const command = '';
  }
}
