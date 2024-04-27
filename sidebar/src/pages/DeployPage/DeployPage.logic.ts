import { IDeployContractForm, IDeployScriptForm, VSCode } from '@/types';
import { SubmitHandler, useForm } from 'react-hook-form';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { MessageType } from '@backend/enums.ts';

export const useDeployPage = (vscode: VSCode, resourceManager: ResourceManager) => {
  const scriptForm = useForm<IDeployScriptForm>({
    defaultValues: {
      environment: '',
      script: '',
    },
  });

  const contractForm = useForm<IDeployContractForm>({
    defaultValues: {
      wallet: '',
      contract: '',
      environment: '',
      value: 0,
      valueUnit: 'wei',
      gasLimit: 300000,
      inputs: [],
    },
  });

  const onSubmitScriptForm: SubmitHandler<IDeployScriptForm> = (data) => {
    vscode.postMessage({
      type: MessageType.DEPLOY_SCRIPT,
      data,
    });
  };

  const onSubmitContractForm: SubmitHandler<IDeployContractForm> = (data) => {
    if (isNaN(data.gasLimit)) contractForm.setError('gasLimit', { type: 'manual', message: 'Invalid number' });
    if (isNaN(data.value)) contractForm.setError('value', { type: 'manual', message: 'Invalid number' });

    vscode.postMessage({
      type: MessageType.DEPLOY_CONTRACT,
      data,
    });
  };

  return {
    scriptForm,
    contractForm,
    vscode,
    wallets: resourceManager.wallets,
    scripts: resourceManager.scripts,
    environments: resourceManager.environments,
    contracts: resourceManager.deployContracts,
    onSubmitContractForm,
    onSubmitScriptForm,
  };
};
