import { InteractContract } from '../../../../vscode/src/actions/types';
import { useFormContext } from 'react-hook-form';
import { IFormInput } from '../../types';
import { useEffect } from 'react';

export const useInteractParams = (contracts: InteractContract[]) => {
  const form = useFormContext<IFormInput>();
  const selectedFunction = form.watch('function');
  const selectedContractAddress = form.watch('contract');
  const selectedContract = contracts?.find((contract) => contract.address === selectedContractAddress);
  const functions = selectedContract?.abi?.map((abi) => {
    if (abi.type === 'function') {
      return abi;
    }
  }) || [];
  const func = functions?.find((func) => func?.name === selectedFunction) || null;
  const inputs = func?.inputs || [];

  useEffect(() => {
    form.resetField('inputs');
  }, [selectedFunction, selectedContractAddress]);

  return {
    inputs,
    form,
  };
};
