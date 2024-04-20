import { IInteractForm, VSCode } from '@/types';
import { InteractContract, Wallets } from '@backend/actions/types';
import { useEffect, useState } from 'react';
import { SubmitHandler, useForm } from 'react-hook-form';

export enum MessageType {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_INTERACT_CONTRACTS = 'GET_INTERACT_CONTRACTS',
  INTERACT_CONTRACTS = 'INTERACT_CONTRACTS',
  WRITE = 'WRITE',
  WRITE_RESPONSE = 'WRITE_RESPONSE',
  READ = 'READ',
  READ_RESPONSE = 'READ_RESPONSE',
  EDIT_WALLETS = 'EDIT_WALLETS',
  EDIT_CONTRACTS = 'EDIT_CONTRACTS',
  UNKNOWN = 'UNKNOWN',
}

export enum ResponseType {
  READ,
  WRITE,
}

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

export const useInteractPage = (vscode: VSCode) => {
  const [wallets, setWallets] = useState<Wallets>([]);
  const [contracts, setContracts] = useState<InteractContract[]>([]);
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
  const [response, setResponse] = useState<{ responseType: ResponseType; data: unknown }>();

  const param = form.watch('inputs');

  useEffect(() => {
    console.log(param);
  }, [param]);

  const onSubmit: SubmitHandler<IInteractForm> = (data) => {
    console.log(data);
    if (isNaN(data.gasLimit)) form.setError('gasLimit', { type: 'manual', message: 'Invalid number' });
    if (isNaN(data.value)) form.setError('value', { type: 'manual', message: 'Invalid number' });

    vscode.postMessage({ type: getFunctionAction(data.function, data.contract, contracts), data });
  };

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageType.GET_WALLETS });
    vscode.postMessage({ type: MessageType.GET_INTERACT_CONTRACTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.WALLETS: {
          form.setValue('wallet', event.data.wallets && event.data.wallets.length ? event.data.wallets[0].id : '');
          setWallets(event.data.wallets);
          break;
        }
        case MessageType.INTERACT_CONTRACTS: {
          form.setValue(
            'contract',
            event.data.contracts && event.data.contracts.length ? event.data.contracts[0].id : '',
          );
          setContracts(event.data.contracts);
          break;
        }
        case MessageType.WRITE_RESPONSE:
          setResponse({ responseType: ResponseType.WRITE, data: event.data.response });
          break;
        case MessageType.READ_RESPONSE:
          setResponse({ responseType: ResponseType.READ, data: event.data.response });
          break;
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
    result: response,
  };
};
