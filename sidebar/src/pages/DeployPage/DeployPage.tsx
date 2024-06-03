import { VSCode } from '@/types';
import { DeployUsingContract } from '@components/deploy/contract/DeployUsingContract.tsx';
import { DeployUsingScript } from '@components/deploy/script/DeployUsingScript.tsx';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { FormProvider } from 'react-hook-form';
import './DeployPage.css';
import { useDeployPage } from './DeployPage.logic.ts';

export const DeployPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useDeployPage(props.vscode, props.resourceManager);

  return (
    <div className="page-container">
      <FormProvider {...logic.scriptForm}>
        <form onSubmit={logic.scriptForm.handleSubmit(logic.onSubmitScriptForm)}>
          <DeployUsingScript
            scripts={logic.scripts}
            vscode={logic.vscode}
            environments={logic.environments}
            isPending={logic.isPending}
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
            isPending={logic.isPending}
          />
        </form>
      </FormProvider>
    </div>
  );
};
