import { createPublicClient, createWalletClient, defineChain, getContract, http, webSocket } from 'viem';
import { privateKeyToAccount } from 'viem/accounts';
import * as vscode from 'vscode';
import { InteractContractRepository } from './InteractContractRepository';
import { WalletRepository } from './WalletRepository';
import { ContractParams } from './types';

export interface ReadContractOptions {
  contractId: string;
  method: string;
  params?: ContractParams;
  outputChannel: vscode.OutputChannel;
}

export interface WriteContractOptions {
  walletId: string;
  contractId: string;
  functionName: string;
  params?: ContractParams;
  gasLimit?: bigint;
  value?: bigint;
  outputChannel: vscode.OutputChannel;
}

export class Interact {
  private _contractRepository: InteractContractRepository;
  private _walletRepository: WalletRepository;

  constructor(contractRepository: InteractContractRepository, walletRepository: WalletRepository) {
    this._contractRepository = contractRepository;
    this._walletRepository = walletRepository;
  }

  public async readContract({ contractId, method, params, outputChannel }: ReadContractOptions): Promise<any> {
    const contractInfos = this._contractRepository.getContract(contractId);

    if (!contractInfos) {
      throw new Error(`contract id ${contractId} not found`);
    }

    const viemContract = getContract({
      address: contractInfos.address,
      abi: contractInfos.abi,
      client: createPublicClient({
        transport: contractInfos.rpc.startsWith('ws') ? webSocket(contractInfos.rpc) : http(contractInfos.rpc),
      }),
    });

    const res = await viemContract.read[method](<any>params);
    outputChannel.append('Output :' + res + '\n');
    return res;
  }

  public async writeContract({
    walletId,
    contractId,
    functionName,
    params,
    gasLimit,
    value,
    outputChannel,
  }: WriteContractOptions): Promise<any> {
    const walletInfos = this._walletRepository.getWallet(walletId);
    const contractInfos = this._contractRepository.getContract(contractId);

    if (!walletInfos) {
      throw new Error(`wallet id ${walletId} not found`);
    }
    if (!contractInfos) {
      throw new Error(`contract id ${contractId} not found`);
    }

    const rpc = contractInfos.rpc.startsWith('ws')
      ? {
          default: {
            webSocket: [contractInfos.rpc],
          },
        }
      : {
          default: {
            http: [contractInfos.rpc],
          },
        };

    const walletClient = createWalletClient({
      chain: defineChain({
        name: 'custom',
        id: contractInfos.chainId,
        nativeCurrency: {
          name: 'Ethereum',
          symbol: 'ETH',
          decimals: 18,
        },
        rpcUrls: <any>rpc,
      }),
      transport: contractInfos.rpc.startsWith('ws') ? webSocket(contractInfos.rpc) : http(contractInfos.rpc),
      account: privateKeyToAccount(walletInfos.privateKey),
    });

    const viemContract = getContract({
      address: contractInfos.address,
      abi: contractInfos.abi,
      client: walletClient,
    });

    const res = await viemContract.write[functionName](<any>params);
    outputChannel.append('Transaction hash :' + res + '\n');
    return res;
  }
}
