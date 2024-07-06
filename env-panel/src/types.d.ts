/* eslint-disable @typescript-eslint/no-explicit-any */
export type VSCode = any;

export interface IInteractForm {
  wallet: string;
  contract: string;
  function: string;
  gasLimit: number;
  value: number;
  valueUnit: 'wei' | 'gwei' | 'ether';
  inputs: any[];
}

export interface IDeployScriptForm {
  wallet: string;
  script: string;
  environment: string;
}

export interface IDeployContractForm {
  wallet: string;
  contract: string;
  environment: string;
  gasLimit: number;
  value: number;
  valueUnit: 'wei' | 'gwei' | 'ether';
  inputs: any[];
}
