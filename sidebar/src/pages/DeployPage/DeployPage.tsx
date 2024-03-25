import { VSCode } from '@/types';
import { DeployUsingContract } from '@components/deploy/contract/DeployUsingContract.tsx';
import { DeployUsingScript } from '@components/deploy/script/DeployUsingScript.tsx';
import { VSCodeDivider } from "@vscode/webview-ui-toolkit/react";
import { useInteractPage } from '../InteractPage/InteractPage.logic.ts';
import "./DeployPage.css";

export const DeployPage = (props: { vscode: VSCode }) => {
  const edit = useInteractPage(props.vscode);

  return (
  <div className="page-container">
    <DeployUsingScript vscode={props.vscode} contracts={edit.contracts} />
    <VSCodeDivider className='divider'/>
    <DeployUsingContract vscode={props.vscode} editContracts={edit.contracts} />
  </div>);
};