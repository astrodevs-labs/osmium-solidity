import { InteractContract } from '@backend/actions/types';
import { VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './InteractParams.css';
import { useInteractParams } from './InteractParams.logic.ts';

export const InteractParams = (props: { contracts: InteractContract[] }) => {
  const logic = useInteractParams(props.contracts);

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
