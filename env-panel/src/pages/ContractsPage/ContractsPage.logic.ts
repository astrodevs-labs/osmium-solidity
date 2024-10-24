import { ContractForm, VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';
import { SubmitHandler, useForm } from 'react-hook-form';

export const useContractsPageLogic = (vscode: VSCode) => {
  const form = useForm<ContractForm>({
    defaultValues: {
      name: '',
      address: '',
      chainId: '',
      rpc: '',
      abi: '',
    },
  });

  const onSubmit: SubmitHandler<ContractForm> = (data) => {
    if (!data.name.length) form.setError('name', { type: 'manual', message: 'Invalid string' });
    if (!data.address.length) form.setError('address', { type: 'manual', message: 'Invalid string' });
    if (isNaN(Number(data.chainId))) form.setError('chainId', { type: 'manual', message: 'Invalid number' });
    if (!data.rpc.length) form.setError('rpc', { type: 'manual', message: 'Invalid string' });
    if (!data.abi.length) form.setError('abi', { type: 'manual', message: 'Invalid string' });

    vscode.postMessage({
      type: MessageType.ADD_CONTRACT,
      data: {
        ...data,
        chainId: Number(data.chainId),
      },
    });
  };

  const deleteContract = (id: string) => {
    vscode.postMessage({
      type: MessageType.DELETE_CONTRACT,
      data: {
        id: id,
      },
    });
  };

  const editContract = (id: string, key: string, value: string) => {
    try {
      vscode.postMessage({
        type: MessageType.EDIT_CONTRACT,
        data: {
          id: id,
          key: key,
          value: key === 'abi' ? JSON.parse(value) : value,
        },
      });
    } catch (error) {
      throw new Error('Impossible to parse ABI');
    }
  };

  return {
    form,
    onSubmit,
    deleteContract,
    editContract,
  };
};
