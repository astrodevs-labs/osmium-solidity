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

    const updateParams = () => {
      const params = functionAbi.inputs.map((_input: any, i: number) => {
        return logic.form.getValues(`inputs.${i}`);
      });

      if (params.length !== functionAbi.inputs.length) {
        return;
      }

      for (const param of params) {
        if (param === null || param === undefined) {
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

      props.vscode.postMessage({
        type: MessageType.ESTIMATE_GAS,
        data,
      });
    };

    updateParams();

    const subscription = logic.form.watch((_value, { name }) => {
      if (name && name.startsWith('inputs')) {
        updateParams();
      }
    });

    return () => subscription.unsubscribe();
  }, [props.vscode, selectedContractId, selectedWalletId, selectedFunctionId]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.ESTIMATE_GAS_RESPONSE: {
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
