import { IInteractForm, VSCode } from '@/types';
import { InteractContract } from '@backend/actions/types';
import { useFormContext } from 'react-hook-form';

export const useInteractContracts = (contracts: InteractContract[], vscode: VSCode) => {
  const {
    register,
    watch,
    formState: { errors },
  } = useFormContext<IInteractForm>();
  const selectedContract = watch('contract');

  const openPanel = () => {
    vscode.postMessage({ type: 'OPEN_PANEL' });
  };

  const functions =
    contracts
      ?.find((contract) => contract.id === selectedContract)
      ?.abi?.map((abi) => {
        if (abi.type === 'function') {
          return abi.name;
        }
      }) || [];

  return { register, functions, errors, openPanel };
};
