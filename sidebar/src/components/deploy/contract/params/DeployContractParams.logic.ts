import { IDeployContractForm } from '@/types';
import { DeployContracts } from '@backend/actions/types';
import { useEffect, useMemo } from 'react';
import { useFormContext } from 'react-hook-form';

export const useDeployContractsParams = (contracts: DeployContracts) => {
  const form = useFormContext<IDeployContractForm>();
  const selectedContractFile = form.watch('contract');
  const inputs = useMemo(() => {
    const res = [];

    if (selectedContractFile) {
      const selectedContract = contracts.find((contract) => contract.path === selectedContractFile);
      if (selectedContract) {
        const constructorAbi = selectedContract.abi?.find((abi) => abi.type === 'constructor');
        if (constructorAbi && constructorAbi.type === 'constructor') {
          res.push(...constructorAbi.inputs);
        }
      }
    }
    return res;
  }, [selectedContractFile, contracts]);

  const displayParams = inputs && inputs.length > 0;

  useEffect(() => {
    return () => {
      inputs.forEach((_, idx) => {
        form.resetField(`inputs.${idx}`);
      });
    };
  }, [inputs]);

  return {
    inputs,
    form,
    displayParams,
  };
};
