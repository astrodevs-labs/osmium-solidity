import { DeployContracts } from '@backend/actions/types';
import { useDeployContractsParams } from './DeployContractParams.logic.ts';
import { VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './DeployContractParams.css';

export const DeployContractParams = (props: { contracts: DeployContracts }) => {
  const logic = useDeployContractsParams(props.contracts);

  return (
    <>
      {logic.displayParams && (
        <div className="params-container">
          {logic.inputs?.map((input, index) => {
            return (
              <>
                <VSCodeTextField
                  className="text-field"
                  {...logic.form.register(`inputs.${index}` as const, {
                    required: true,
                    valueAsNumber: input.type.includes('int'),
                  })}
                >
                  {input.name} ({input.type})
                </VSCodeTextField>
              </>
            );
          })}
        </div>
      )}
    </>
  );
};
