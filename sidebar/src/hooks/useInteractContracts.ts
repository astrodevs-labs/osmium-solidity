import { IFormInput, VSCode } from '@/types';
import { InteractContract } from '@backend/actions/types';
import { MessageType } from '@pages/InteractPage/InteractPage.logic.ts';
import { useFormContext } from 'react-hook-form';

export const useInteractContracts = (contracts: InteractContract[], vscode: VSCode) => {
  const {
    register,
    watch,
    formState: { errors },
  } = useFormContext<IFormInput>();
  const selectedContract = watch('contract');

  const functions =
    contracts
      ?.find((contract) => contract.id === selectedContract)
      ?.abi?.map((abi) => {
        if (abi.type === 'function') {
          return abi.name;
        }
      }) || [];

  const editWallet = () => {
    vscode.postMessage({ type: MessageType.EDIT_WALLETS });
  };

  const editContract = () => {
    vscode.postMessage({ type: MessageType.EDIT_CONTRACTS });
  };

  return { register, selectedContract, functions, errors, editWallet, editContract };
};
