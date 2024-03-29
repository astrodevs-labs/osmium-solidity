import { Abi, Address } from 'viem';

export type RpcUrl = `ws://${string}` | `wss://${string}` | `http://${string}` | `https://${string}`;

export type DeployContracts = {
  name: string;
  path: string;
  abi: Abi;
  address?: string;
};

export type DeployScriptArgs = {
  rpcUrl: string;
  script: Script;
  verify: boolean;
};

export type DeployContractArgs = {
  rpcUrl: string;
  contract: DeployContracts;
  verify: boolean;
  cstrArgs: string[];
};

export interface InteractContract {
  name: string;
  address: Address;
  abi: Abi;
  chainId: number;
  rpc: RpcUrl;
  id: string;
}

export type InteractContracts = InteractContract[];

export interface Wallet {
  name: string;
  address: Address;
  privateKey: Address;
  rpc: RpcUrl;
  id: string;
}

export type Wallets = Wallet[];

export interface Environment {
  name: string;
  rpc: RpcUrl;
  id: string;
}

export type Environments = Environment[];

export interface Script {
  name: string;
  path: string;
  id: string;
}

export type Scripts = Script[];

export type ContractParam = any;

export type ContractParams = ContractParam[];
