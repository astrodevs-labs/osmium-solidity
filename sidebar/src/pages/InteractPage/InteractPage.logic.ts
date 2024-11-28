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
  const selectedFunctionId = form.watch('function');
  const selectedContractId = form.watch('contract');
  const selectedWalletId = form.watch('wallet');

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
  }, [resourceManager.wallets]);

  useEffect(() => {
    form.setValue(
      'contract',
      resourceManager.interactContracts && resourceManager.interactContracts.length
        ? resourceManager.interactContracts[0].id
        : '',
    );
  }, [resourceManager.interactContracts]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.WRITE_RESPONSE:
          setResponse({ responseType: MessageType.WRITE, data: event.data.response });
          break;
        case MessageType.READ_RESPONSE:
          setResponse({ responseType: MessageType.READ, data: event.data.response });
          break;
        case MessageType.ESTIMATE_GAS_RESPONSE: {
          form.setValue('gasLimit', event.data.response.gas);
          break;
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  useEffect(() => {
    const functions =
      resourceManager.interactContracts
        ?.find((contract) => contract.id === selectedContractId)
        ?.abi?.map((abi) => {
          if (abi.type === 'function') {
            return abi.name;
          }
        }) || [];
    if (functions.length > 0 && functions[0]) {
      form.setValue('function', functions[0]);
    }
  }, [selectedContractId]);

  useEffect(() => {
    if (!vscode || !selectedContractId || !selectedWalletId) {
      return;
    }
    const selectedContract = resourceManager.interactContracts.filter((contract) => contract.id === selectedContractId);
    const selectedWallet = resourceManager.wallets.filter((wallet) => wallet.id === selectedWalletId);

    if (
      !selectedContract ||
      !selectedWallet ||
      !selectedFunctionId ||
      !selectedContract[0].abi ||
      !selectedWallet[0].address ||
      !selectedContract[0].address
    ) {
      return;
    }

    const functionAbi = selectedContract[0].abi.find(
      (abi) => abi.type === 'function' && abi.name === selectedFunctionId,
    ) as any;

    if (!functionAbi) return;

    const updateParams = () => {
      const params = functionAbi.inputs.map((_input: any, i: number) => {
        return form.getValues(`inputs.${i}`);
      });

      if (params.length !== functionAbi.inputs.length) {
        return;
      }

      for (const param of params) {
        if (param === null || param === undefined) {
          return;
        }
      }

      const data = {
        abi: selectedContract[0].abi,
        walletAddress: selectedWallet[0].address,
        params,
        function: selectedFunctionId,
        address: selectedContract[0].address,
      };

      vscode.postMessage({
        type: MessageType.ESTIMATE_GAS,
        data,
      });
    };

    updateParams();

    const subscription = form.watch((_value, { name }) => {
      if (name && name.startsWith('inputs')) {
        updateParams();
      }
    });

    return () => subscription.unsubscribe();
  }, [vscode, selectedContractId, selectedWalletId, selectedFunctionId]);

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
