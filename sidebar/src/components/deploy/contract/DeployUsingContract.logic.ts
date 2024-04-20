import { IDeployContractForm, VSCode } from '@/types';
import { useFormContext } from 'react-hook-form';
import { useEdit } from '@hooks/useEdit.ts';
import { useEffect, useState } from 'react';
import { MessageType } from '@backend/enums.ts';
import { DeployContracts, Environments, Wallets } from '@backend/actions/types';

export const useDeployUsingContract = (
  vscode: VSCode,
  wallets: Wallets,
  contracts: DeployContracts,
  environments: Environments,
) => {
  const { editEnvironment, editWallet } = useEdit(vscode);
  const form = useFormContext<IDeployContractForm>();
  const {
    formState: { errors },
  } = form;
  const [response, setResponse] = useState<{
    exitCode: number;
    output: string;
  } | null>(null);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.DEPLOY_CONTRACT_RESPONSE: {
          setResponse(event.data.response);
          break;
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  useEffect(() => {
    form.setValue('wallet', wallets[0]?.id || '');
  }, [wallets]);

  useEffect(() => {
    form.setValue('contract', contracts[0]?.id || '');
  }, [contracts]);

  useEffect(() => {
    form.setValue('environment', environments[0]?.id || '');
  }, [environments]);

  return { form, errors, response, editEnvironment, editWallet };
};
