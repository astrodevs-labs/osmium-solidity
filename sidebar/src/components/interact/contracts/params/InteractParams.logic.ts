import { IInteractForm } from '@/types';
import { InteractContracts } from '@backend/actions/types';
import { useEffect } from 'react';
import { useFormContext } from 'react-hook-form';

export const useInteractParams = (contracts: InteractContracts) => {
  const form = useFormContext<IInteractForm>();
  const selectedFunction = form.watch('function');
  const selectedContractAddress = form.watch('contract');
  const selectedContract = contracts?.find((contract) => contract.id === selectedContractAddress);
  const functions =
    selectedContract?.abi?.map((abi) => {
      if (abi.type === 'function') {
        return abi;
      }
    }) || [];
  const func = functions?.find((func) => func?.name === selectedFunction) || null;
  const inputs = func?.inputs || [];
  const displayParams = inputs && inputs.length > 0;

  useEffect(() => {
    form.resetField('inputs');
  }, [selectedFunction, selectedContractAddress]);

  return {
    inputs,
    form,
    displayParams,
  };
};
