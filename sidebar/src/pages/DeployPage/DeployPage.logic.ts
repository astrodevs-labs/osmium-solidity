import { IDeployContractForm, IDeployScriptForm, VSCode } from '@/types';
import { DeployContracts, Environments, Scripts, Wallets } from '@backend/actions/types';
import { useEffect, useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';

export enum MessageTypeScript {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_SCRIPTS = 'GET_SCRIPTS',
  SCRIPTS = 'SCRIPTS',
  DEPLOY_SCRIPT = 'DEPLOY_SCRIPT',
  DEPLOY_SCRIPT_RESPONSE = 'DEPLOY_SCRIPT_RESPONSE',
}

export enum MessageTypeContract {
  GET_WALLETS = 'GET_WALLETS',
  GET_DEPLOY_CONTRACTS = 'GET_DEPLOY_CONTRACTS',
  DEPLOY_CONTRACTS = 'DEPLOY_CONTRACTS',
  EDIT_ENVIRONMENT = 'EDIT_ENVIRONMENT',
  GET_ENVIRONMENTS = 'GET_ENVIRONMENTS',
  ENVIRONMENTS = 'ENVIRONMENTS',
}

export const useResourceManager = () => {
  const [wallets, setWallets] = useState<Wallets>([]);
  const [scripts, setScripts] = useState<Scripts>([]);
  const [contracts, setContracts] = useState<DeployContracts>([]);
  const [environments, setEnvironments] = useState<Environments>([]);
  const [deployScriptResponse, setDeployScriptResponse] = useState<string>('');

  const formScript = useForm<IDeployScriptForm>({
    defaultValues: {
      wallet: '',
      script: '',
    },
  });

  const formContract = useForm<IDeployContractForm>({
    defaultValues: {
      wallet: '',
      contract: '',
      environment: '',
      value: 0,
      valueUnit: 'wei',
      gasLimit: 300000,
    },
  });

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageTypeScript.WALLETS: {
          formScript.setValue(
            'wallet',
            event.data.wallets && event.data.wallets.length ? event.data.wallets[0].id : '',
          );
          setWallets(event.data.wallets);
          break;
        }
        case MessageTypeScript.SCRIPTS: {
          formScript.setValue(
            'script',
            event.data.scripts && event.data.scripts.length ? event.data.scripts[0].id : '',
          );
          setScripts(event.data.scripts);
          break;
        }
        case MessageTypeContract.DEPLOY_CONTRACTS: {
          formContract.setValue(
            'contract',
            event.data.contracts && event.data.contracts.length ? event.data.contracts[0].id : '',
          );
          setContracts(event.data.contracts);
          break;
        }
        case MessageTypeContract.ENVIRONMENTS: {
          formContract.setValue(
            'environment',
            event.data.environments && event.data.environments.length ? event.data.environments[0].id : '',
          );
          setEnvironments(event.data.environments);
          break;
        }
        case MessageTypeScript.DEPLOY_SCRIPT_RESPONSE: {
          setDeployScriptResponse(event.data.response);
          break;
        }
        default: {
          throw Error('Unknown command: ' + event.type);
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  return {
    formScript,
    formContract,
    wallets,
    scripts,
    contracts,
    environments,
    deployScriptResponse,
  };
};

// -----------------------------------------------------------------------

export const useDeployPageScript = (vscode: VSCode) => {
  const { formScript, wallets, scripts } = useResourceManager();

  const onSubmit: SubmitHandler<IDeployScriptForm> = (data) => {
    vscode.postMessage({
      type: MessageTypeScript.DEPLOY_SCRIPT,
      data,
    });
  };

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageTypeScript.GET_WALLETS });
    vscode.postMessage({ type: MessageTypeScript.GET_SCRIPTS });
  }, [vscode]);

  return {
    formScript,
    vscode,
    wallets,
    scripts,
    onSubmit,
  };
};

// -----------------------------------------------------------------------

export const useDeployPageContract = (vscode: VSCode) => {
  const onSubmit: SubmitHandler<IDeployContractForm> = (data) => {
    console.log(data);
  };

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageTypeContract.GET_WALLETS });
    vscode.postMessage({ type: MessageTypeContract.GET_DEPLOY_CONTRACTS });
    vscode.postMessage({ type: MessageTypeContract.GET_ENVIRONMENTS });
  }, [vscode]);

  const { formContract, wallets, contracts, environments } = useResourceManager();

  return {
    formContract,
    vscode,
    wallets,
    contracts,
    onSubmit,
    environments,
  };
};
