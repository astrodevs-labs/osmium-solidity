import { IInteractForm, VSCode } from '@/types';
import { InteractContract } from '@backend/actions/types';
import { useFormContext } from 'react-hook-form';
import { useEdit } from '@hooks/useEdit.ts';

export const useInteractContracts = (contracts: InteractContract[], vscode: VSCode) => {
  const { editWallet, editContract } = useEdit(vscode);
  const {
    register,
    watch,
    formState: { errors },
  } = useFormContext<IInteractForm>();
  const selectedContract = watch('contract');

  const functions =
    contracts
      ?.find((contract) => contract.id === selectedContract)
      ?.abi?.map((abi) => {
        if (abi.type === 'function') {
          return abi.name;
        }
      }) || [];

  return { register, functions, errors, editWallet, editContract };
};
