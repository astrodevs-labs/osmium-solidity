import { ContractSchema } from '@/schemas/Contract.schema';
import { ContractForm, VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';
import { zodResolver } from '@hookform/resolvers/zod';
import { SubmitHandler, useForm } from 'react-hook-form';

export const useContractsPageLogic = (vscode: VSCode) => {
  const form = useForm<ContractForm>({
    mode: 'onChange',
    resolver: zodResolver(ContractSchema),
  });

  const onSubmit: SubmitHandler<ContractForm> = (data) => {
    vscode.postMessage({
      type: MessageType.ADD_CONTRACT,
      data,
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
