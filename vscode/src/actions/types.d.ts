import { Abi, Address } from 'viem';

export type RpcUrl = `ws://${string}` | `wss://${string}` | `http://${string}` | `https://${string}`;

export type DeployContracts = {
  name: string;
  path: string;
  abi: Abi;
  address?: string;
};

export type DeployScript = {
  path: string;
  name: string;
};

export type DeployEnvironment = {
  name: string;
  rpc: string;
};

export type DeployScriptArgs = {
  rpcUrl: string;
  script: DeployScript;
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
}

export type InteractContracts = InteractContract[];

export interface Wallet {
  name: string;
  address: Address;
  privateKey: Address;
  rpc: RpcUrl;
}

export type Wallets = Wallet[];

export interface Environment {
  name: string;
  rpc: RpcUrl;
}

export type Environments = Environment[];

export interface Script {
  name: string;
  path: string;
}

export type Scripts = Script[];

export type ContractParam = any;

export type ContractParams = ContractParam[];
