import { WalletSchema } from '@/schemas/Wallet.schema';
import './WalletsPage.css';
import { VSCode, WalletForm } from '@/types';
import { MessageType } from '@backend/enums.ts';
import { zodResolver } from '@hookform/resolvers/zod';
import { SubmitHandler, useForm } from 'react-hook-form';

export const useWalletsPageLogic = (vscode: VSCode) => {
  const form = useForm<WalletForm>({
    mode: 'onChange',
    resolver: zodResolver(WalletSchema),
  });

  const onSubmit: SubmitHandler<WalletForm> = (data) => {
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
