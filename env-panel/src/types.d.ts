/* eslint-disable @typescript-eslint/no-explicit-any */
export type VSCode = any;

export type WalletForm = {
  name: string;
  privateKey: string;
};

export type EnvironmentForm = {
  name: string;
  rpc: string;
};

export type ContractForm = {
  name: string;
  address: string;
  rpc: string;
  abi: string;
};
