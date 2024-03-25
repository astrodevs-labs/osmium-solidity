import { VSCode } from '@/types';
import { InteractContracts } from '@components/interact/contracts/InteractContracts.tsx';
import { InteractParams } from '@components/interact/contracts/params/InteractParams.tsx';
import { VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import { FormProvider } from 'react-hook-form';
import './InteractPage.css';
import { ResponseType, useInteractPage } from './InteractPage.logic.ts';

export const InteractPage = (vscode: { vscode: VSCode }) => {
  const logic = useInteractPage(vscode);

  const Result = ({ responseType, data }: { responseType: ResponseType; data: unknown }) => {
    return (
      <>
        <VSCodeDivider className="divider" />
        <p>{responseType === ResponseType.READ ? 'Read response:' : 'Transaction hash:'}</p>
        <p>{data + ''}</p>
      </>
    );
  };

  return (
    <div className="page-container">
      <FormProvider {...logic.form}>
        <form onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
          <InteractContracts wallets={logic.wallets} contracts={logic.contracts} vscode={vscode} />
          <VSCodeDivider className="divider" />
          <InteractParams contracts={logic.contracts} />
          <VSCodeButton className="submit-button" type="submit">
            Send transaction
          </VSCodeButton>
          {logic.result && <Result {...logic.result} />}
        </form>
      </FormProvider>
    </div>
  );
};
