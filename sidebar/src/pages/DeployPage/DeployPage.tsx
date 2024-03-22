import { FormProvider } from "react-hook-form";
import { DeployUsingContract } from '../../components/DeployUsingContract/DeployUsingContract.tsx';
import { DeployUsingScript } from '../../components/DeployUsingScript/DeployUsingScript.tsx';
import { VSCode } from '../../types';
import { useInteractPage } from '../InteractPage/InteractPage.logic.ts';
import "./DeployPage.css";
import { useDeployPageContract, useDeployPageScript } from './DeployPage.logic.ts';

export const DeployPage = (props: { vscode: VSCode }) => {
  const logicScript = useDeployPageScript(props.vscode);
  const logicContract = useDeployPageContract(props.vscode);
  const edit = useInteractPage(props.vscode);

  return (
  <div className="page-container">
    <FormProvider {...logicScript.form}>
      <form onSubmit={logicScript.form.handleSubmit(logicScript.onSubmit)}>
        <DeployUsingScript wallets={logicScript.wallets} scripts={logicScript.scripts} vscode={props.vscode} contracts={edit.contracts} />
      </form>
    </FormProvider>
    <FormProvider {...logicContract.form}>
      <form onSubmit={logicContract.form.handleSubmit(logicContract.onSubmit)}>
        <DeployUsingContract wallets={logicContract.wallets} deployContracts={logicContract.contracts} vscode={props.vscode} editContracts={edit.contracts} environments={logicContract.environments}/>
      </form>
    </FormProvider>
  </div>);
};