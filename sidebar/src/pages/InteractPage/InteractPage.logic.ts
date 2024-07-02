import { IInteractForm, VSCode } from '@/types';
import { InteractContract } from '@backend/actions/types';
import { MessageType } from '@backend/enums.ts';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { useEffect, useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';

const getFunctionAction = (func: string, contract: string, contracts: InteractContract[]): MessageType => {
  const selectedContract = contracts?.find((c) => c.id === contract);
  const functions =
    selectedContract?.abi?.map((abi) => {
      if (abi.type === 'function') {
        return abi;
      }
    }) || [];
  const selectedFunction = functions?.find((f) => f?.name === func) || null;

  if (!selectedFunction) return MessageType.UNKNOWN;
  if (selectedFunction.stateMutability === 'view') return MessageType.READ;
  return MessageType.WRITE;
};

export const useInteractPage = (vscode: VSCode, resourceManager: ResourceManager) => {
  const [isPending, setIsPending] = useState(false);
  const form = useForm<IInteractForm>({
    defaultValues: {
      wallet: '',
      contract: '',
      function: '',
      value: 0,
      valueUnit: 'wei',
      gasLimit: 300000,
    },
  });
  const [response, setResponse] = useState<{ responseType: MessageType; data: string }>();

  const onSubmit: SubmitHandler<IInteractForm> = (data) => {
    if (isNaN(data.gasLimit)) form.setError('gasLimit', { type: 'manual', message: 'Invalid number' });
    if (isNaN(data.value)) form.setError('value', { type: 'manual', message: 'Invalid number' });
    
    setIsPending(true);
    vscode.postMessage({
      type: getFunctionAction(data.function, data.contract, resourceManager.interactContracts),
      data,
    });
  };

  useEffect(() => {
    form.setValue(
      'wallet',
      resourceManager.wallets && resourceManager.wallets.length ? resourceManager.wallets[0].id : '',
    );
  }, [resourceManager.wallets, form]);

  useEffect(() => {
    form.setValue(
      'contract',
      resourceManager.interactContracts && resourceManager.interactContracts.length
        ? resourceManager.interactContracts[0].id
        : '',
    );
  }, [resourceManager.interactContracts, form]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.WRITE_RESPONSE:
          setResponse({ responseType: MessageType.WRITE, data: event.data.response });
          break;
        case MessageType.READ_RESPONSE:
          setResponse({ responseType: MessageType.READ, data: event.data.response });
          break;
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  return {
    form,
    vscode,
    wallets: resourceManager.wallets,
    contracts: resourceManager.interactContracts,
    onSubmit,
    response,
    isPending,
  };
};
