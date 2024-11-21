import { VSCode } from '@/types';
import { DeployUsingContract } from '@components/deploy/contract/DeployUsingContract.tsx';
import { DeployUsingScript } from '@components/deploy/script/DeployUsingScript.tsx';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { useEffect } from 'react';
import { FormProvider } from 'react-hook-form';
import './DeployPage.css';
import { useDeployPage } from './DeployPage.logic.ts';
import { publicClient } from './config.ts';

export const DeployPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useDeployPage(props.vscode, props.resourceManager);
  const selectedContractId = logic.contractForm.watch('contract');
  const selectedWalletId = logic.contractForm.watch('wallet');

  useEffect(() => {
    const fetchGasEstimate = async () => {
      const selectedContract = logic.contracts.filter((contract) => contract.id === selectedContractId);
      const selectedWallet = logic.wallets.filter((wallet) => wallet.id === selectedWalletId);

      const abi = selectedContract[0].abi;
      const walletAddress = selectedWallet[0].address;

      for (const item of abi) {
        console.log('item', item);
        if (item.type === 'function') {
          const functionName = item.name;
          try {
            const gas = await publicClient.estimateContractGas({
              address: walletAddress,
              abi: abi,
              functionName: functionName,
              account: selectedWallet[0].address,
              args: item.inputs.map((input) => {
                return { type: input.type, value: input.name };
              }),
            });
            console.log(`Gas estimate for ${functionName}:`, gas);
          } catch (error) {
            console.error(`Error estimating gas for ${functionName}:`, error);
          }
        }
      }
    };

    fetchGasEstimate();
  }, [selectedContractId]);
  return (
    <div className="page-container">
      <FormProvider {...logic.scriptForm}>
        <form onSubmit={logic.scriptForm.handleSubmit(logic.onSubmitScriptForm)}>
          <DeployUsingScript
            scripts={logic.scripts}
            vscode={logic.vscode}
            environments={logic.environments}
            isPending={logic.isPendingScript}
            setIsPending={logic.setIsPendingScript}
          />
        </form>
      </FormProvider>
      <FormProvider {...logic.contractForm}>
        <form onSubmit={logic.contractForm.handleSubmit(logic.onSubmitContractForm)}>
          <DeployUsingContract
            wallets={logic.wallets}
            deployContracts={logic.contracts}
            vscode={logic.vscode}
            environments={logic.environments}
            isPending={logic.isPendingContract}
            setIsPending={logic.setIsPendingContract}
          />
        </form>
      </FormProvider>
    </div>
  );
};
