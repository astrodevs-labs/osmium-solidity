import { VSCode } from '@/types';
import { InteractContracts } from '@components/interact/contracts/InteractContracts.tsx';
import { InteractParams } from '@components/interact/contracts/params/InteractParams.tsx';
import { VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import { FormProvider } from 'react-hook-form';
import './InteractPage.css';
import { ResponseType, useInteractPage } from './InteractPage.logic.ts';

export const InteractPage = (props: { vscode: VSCode }) => {
  const logic = useInteractPage(props.vscode);

  interface ResultProps {
    responseType: ResponseType;
    data: any;
  }

  const Result = (result: ResultProps) => {
    return (
      <div>
        <VSCodeDivider className="divider" />
        <p>{result.responseType === ResponseType.READ ? 'Read response:' : 'Transaction hash:'}</p>
        <p>{result.data + ''}</p>
      </div>
    );
  }

  return (
    <div className="page-container">
      <FormProvider {...logic.form} >
        <form onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
          <InteractContracts wallets={logic.wallets} contracts={logic.contracts} vscode={props.vscode} />
          <VSCodeDivider className="divider" />
          <InteractParams contracts={logic.contracts} />
          <VSCodeButton className="submit-button" type="submit">Send transaction</VSCodeButton>
          {logic.result && Result({...logic.result})
          }
        </form>
      </FormProvider>
    </div>
  );
};