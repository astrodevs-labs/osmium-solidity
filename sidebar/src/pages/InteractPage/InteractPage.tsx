import { VSCode } from '@/types';
import { MessageType } from '@backend/enums.ts';
import { InteractContracts } from '@components/interact/contracts/InteractContracts.tsx';
import { InteractParams } from '@components/interact/contracts/params/InteractParams.tsx';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import { useEffect } from 'react';
import { FormProvider } from 'react-hook-form';
import Loader from '../../components/Loader.tsx';
import './InteractPage.css';
import { useInteractPage } from './InteractPage.logic.ts';

const Response = (result: { responseType: MessageType; data: string }) => {
  return (
    <div>
      <VSCodeDivider className="divider" />
      <p>{result.responseType === MessageType.READ ? 'Read response:' : 'Transaction hash:'}</p>
      <p>{result.data}</p>
    </div>
  );
};

export const InteractPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useInteractPage(props.vscode, props.resourceManager);
  const selectedFunctionId = logic.form.watch('function');
  const selectedContractId = logic.form.watch('contract');
  const selectedWalletId = logic.form.watch('wallet');

  useEffect(() => {
    const functions =
      logic.contracts
        ?.find((contract) => contract.id === selectedContractId)
        ?.abi?.map((abi) => {
          if (abi.type === 'function') {
            return abi.name;
          }
        }) || [];
    if (functions.length > 0 && functions[0]) {
      logic.form.setValue('function', functions[0]);
    }
  }, [selectedContractId]);

  useEffect(() => {
    if (!props.vscode || !selectedContractId || !selectedWalletId) {
      return;
    }
    const selectedContract = logic.contracts.filter((contract) => contract.id === selectedContractId);
    const selectedWallet = logic.wallets.filter((wallet) => wallet.id === selectedWalletId);

    if (
      !selectedContract ||
      !selectedWallet ||
      !selectedFunctionId ||
      !selectedContract[0].abi ||
      !selectedWallet[0].address ||
      !selectedContract[0].address
    ) {
      return;
    }

    const functionAbi = selectedContract[0].abi.find(
      (abi) => abi.type === 'function' && abi.name === selectedFunctionId,
    ) as any;

    if (!functionAbi) return;

    const params = functionAbi.inputs.map((input: any, i: number) => {
      console.log('AA input', input);
      return logic.form.getValues(`inputs.${i}`);
    });
    console.log('AA params', params);
    console.log('AA params.length', params.length);
    console.log('AA functionAbi.inputs.length', functionAbi.inputs.length);

    if (params.length !== functionAbi.inputs.length) {
      return;
    }

    for (const param of params) {
      console.log('AA param =', param);
      if (param === null || param === undefined) {
        console.log('AA one param is null|undefined');
        return;
      }
    }

    const data = {
      abi: selectedContract[0].abi,
      walletAddress: selectedWallet[0].address,
      params,
      function: selectedFunctionId,
      address: selectedContract[0].address,
    };
    console.log('AA data = ', data);

    props.vscode.postMessage({
      type: MessageType.ESTIMATE_GAS,
      data,
    });
  }, [props.vscode, selectedContractId, selectedWalletId, selectedFunctionId]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.ESTIMATE_GAS_RESPONSE: {
          console.log('AA response = ', event.data.response.gas);
          logic.form.setValue('gasLimit', event.data.response.gas);
          break;
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  return (
    <div className="page-container">
      <FormProvider {...logic.form}>
        <form onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
          <InteractContracts wallets={logic.wallets} contracts={logic.contracts} vscode={props.vscode} />
          <VSCodeDivider className="divider" />
          <InteractParams contracts={logic.contracts} />
          <VSCodeButton className="submit-button" appearance="primary" type="submit">
            Send transaction
          </VSCodeButton>
          {logic.isPending && !logic.response && <Loader />}
          {logic.response && Response({ ...logic.response })}
        </form>
      </FormProvider>
    </div>
  );
};
