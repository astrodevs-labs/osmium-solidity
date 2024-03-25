import { DFormContract, DFormScript, VSCode } from '@/types';
import { DeployContracts, DeployEnvironment, DeployScript, InteractWallet } from '@backend/actions/types';
import { useEffect, useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';

export enum MessageTypeScript {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_SCRIPTS = 'GET_SCRIPTS',
  SCRIPTS = 'SCRIPTS',
}

export enum MessageTypeContract {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_DEPLOY_CONTRACTS = 'GET_DEPLOY_CONTRACTS',
  DEPLOY_CONTRACTS = 'DEPLOY_CONTRACTS',
  EDIT_ENVIRONMENT = 'EDIT_ENVIRONMENT',
  GET_ENVIRONMENTS = 'GET_ENVIRONMENTS',
  ENVIRONMENTS = 'ENVIRONMENTS',
}

export const useDeployPageScript = (vscode: VSCode) => {
  const [wallets, setWallets] = useState<InteractWallet[]>([]);
  const [scripts, setScripts] = useState<DeployScript[]>([]);
  const form = useForm<DFormScript>({
    defaultValues: {
      wallet: '',
      script: '',
    },
  });

  const onSubmit: SubmitHandler<DFormScript> = (data) => {
    console.log(data);
  };

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageTypeScript.GET_WALLETS });
    vscode.postMessage({ type: MessageTypeScript.GET_SCRIPTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageTypeScript.WALLETS: {
          form.setValue('wallet', event.data.wallets && event.data.wallets.length ? event.data.wallets[0].address : '');
          setWallets(event.data.wallets);
          break;
        }
        case MessageTypeScript.SCRIPTS: {
          form.setValue('script', event.data.scripts && event.data.scripts.length ? event.data.scripts[0].name : '');
          setScripts(event.data.scripts);
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
    form,
    vscode,
    wallets,
    scripts,
    onSubmit,
  };
};

export const useDeployPageContract = (vscode: VSCode) => {
  const [wallets, setWallets] = useState<InteractWallet[]>([]);
  const [contracts, setContracts] = useState<DeployContracts[]>([]);
  const [environments, setEnvironments] = useState<DeployEnvironment[]>([]);

  const form = useForm<DFormContract>({
    defaultValues: {
      wallet: '',
      contract: '',
      environment: '',
      value: 0,
      valueUnit: 'wei',
      gasLimit: 300000,
    },
  });

  const onSubmit: SubmitHandler<DFormContract> = (data) => {
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

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageTypeContract.WALLETS: {
          form.setValue('wallet', event.data.wallets && event.data.wallets.length ? event.data.wallets[0].address : '');
          setWallets(event.data.wallets);
          break;
        }
        case MessageTypeContract.DEPLOY_CONTRACTS: {
          form.setValue('contract', event.data.contracts && event.data.contracts.length ? event.data.contracts[0].path : '');
          setContracts(event.data.contracts);
          break;
        }
        case MessageTypeContract.ENVIRONMENTS: {
          form.setValue('environment', event.data.environments && event.data.environments.length ? event.data.environments[0].name : '');
          setEnvironments(event.data.environments);
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
    form,
    vscode,
    wallets,
    contracts,
    onSubmit,
    environments
  };
};
