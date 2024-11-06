import './WalletsPage.css';
import { VSCode, WalletForm } from '@/types';
import { MessageType } from '@backend/enums.ts';
import { SubmitHandler, useForm } from 'react-hook-form';

export const useWalletsPageLogic = (vscode: VSCode) => {
  const form = useForm<WalletForm>({
    defaultValues: {
      name: '',
      privateKey: '',
    },
  });

  const onSubmit: SubmitHandler<WalletForm> = (data) => {
    if (!data.name.length) form.setError('name', { type: 'manual', message: 'Invalid string' });
    if (!data.privateKey.length) form.setError('privateKey', { type: 'manual', message: 'Invalid string' });

    vscode.postMessage({
      type: MessageType.ADD_WALLET,
      data,
    });
  };

  const deleteWallet = (id: string) => {
    vscode.postMessage({
      type: MessageType.DELETE_WALLET,
      data: {
        id: id,
      },
    });
  };

  const editWallet = (id: string, key: string, value: string) => {
    vscode.postMessage({
      type: MessageType.EDIT_WALLET,
      data: {
        id: id,
        key: key,
        value: value,
      },
    });
  };

  return {
    deleteWallet,
    editWallet,
    form,
    onSubmit,
  };
};
