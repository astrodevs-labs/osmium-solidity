import { VSCode } from '@/types';
import { DeployUsingContract } from '@components/deploy/contract/DeployUsingContract.tsx';
import { DeployUsingScript } from '@components/deploy/script/DeployUsingScript.tsx';
import { FormProvider } from 'react-hook-form';
import { useInteractPage } from '../InteractPage/InteractPage.logic.ts';
import './DeployPage.css';
import { useDeployPageContract, useDeployPageScript } from './DeployPage.logic.ts';

export const DeployPage = (props: { vscode: VSCode }) => {
  const logicScript = useDeployPageScript(props.vscode);
  const logicContract = useDeployPageContract(props.vscode);
  const edit = useInteractPage(props.vscode);

  return (
    <div className="page-container">
      <FormProvider {...logicScript.formScript}>
        <form onSubmit={logicScript.formScript.handleSubmit(logicScript.onSubmit)}>
          <DeployUsingScript
            scripts={logicScript.scripts}
            vscode={props.vscode}
            environments={logicContract.environments}
          />
        </form>
      </FormProvider>
      <FormProvider {...logicContract.formContract}>
        <form onSubmit={logicContract.formContract.handleSubmit(logicContract.onSubmit)}>
          <DeployUsingContract
            wallets={logicContract.wallets}
            deployContracts={logicContract.contracts}
            vscode={props.vscode}
            editContracts={edit.contracts}
          />
        </form>
      </FormProvider>
    </div>
  );
};
