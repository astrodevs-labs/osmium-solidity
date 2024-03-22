/* eslint-disable @typescript-eslint/no-explicit-any */
import {
  Abi,
  createPublicClient,
  createWalletClient, defineChain,
  getContract,
  http,
  webSocket,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { ContractRepository } from "./ContractRepository";
import { WalletRepository } from "./WalletRepository";

interface ReadContractOptions {
  contract: `0x${string}`;
  method: string;
  params?: any[];
}

interface WriteContractOptions {
  account: `0x${string}`;
  address: `0x${string}`;
  abi: Abi;
  functionName: string;
  params?: any[];
  gasLimit?: bigint;
  value?: bigint;
}

export class Interact {
  private contractRepository: ContractRepository;
  private walletRepository: WalletRepository;

  constructor(
    contractRepository: ContractRepository,
    walletRepository: WalletRepository,
  ) {
    this.contractRepository = contractRepository;
    this.walletRepository = walletRepository;
  }

  public async readContract({
    contract,
    method,
    params,
  }: ReadContractOptions): Promise<any> {
    const contractInfos = this.contractRepository.getContract(contract);
    if (!contractInfos) {
      throw new Error(`contract ${contract} not found`);
    }

    const viemContract = getContract({
      address: contractInfos.address,
      abi: contractInfos.abi,
      client: createPublicClient({
        transport: contractInfos.rpc.startsWith("ws")
          ? webSocket(contractInfos.rpc)
          : http(contractInfos.rpc),
      }),
    });

    return await viemContract.read[method](<any>params);
  }

  public async writeContract({
    account,
    address,
    abi,
    functionName,
    params,
    gasLimit,
    value,
  }: WriteContractOptions): Promise<any> {
    const walletInfos = this.walletRepository.getWallet(account);
    if (!walletInfos) {
      throw new Error(`wallet ${account} not found`);
    }
    const contract = this.contractRepository.getContract(address);
    if (!contract) {
      throw new Error(`contract ${address} not found`);
    }

    const rpc = contract.rpc.startsWith("ws") ? {
      default: {
        webSocket: [contract.rpc],
      },
    } : {
      default: {
        http: [contract.rpc],
      },
    };

    const walletClient = createWalletClient({
      chain: defineChain({
        name: "custom",
        id: contract.chainId,
        nativeCurrency: {
          name: "Ethereum",
          symbol: "ETH",
          decimals: 18,
        },
        rpcUrls: <any>rpc,
      }),
      transport: contract.rpc.startsWith("ws")
        ? webSocket(contract.rpc)
        : http(contract.rpc),
      account: privateKeyToAccount(walletInfos.privateKey),
    });

    const viemContract = getContract({
      address,
      abi,
      client: walletClient,
    });

    await walletClient.writeContract({
      address,
      abi,
      functionName,
      args: params,
      gas: gasLimit,
      value,
    });

    return await viemContract.write[functionName](<any>params);
  }
}
