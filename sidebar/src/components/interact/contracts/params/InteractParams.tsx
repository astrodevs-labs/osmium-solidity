import { InteractContract } from '@backend/actions/types';
import { useInteractParams } from '@hooks/useInteractParams.ts';
import { VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './InteractParams.css';

export const InteractParams = (props: { contracts: InteractContract[] }) => {
  const logic = useInteractParams(props.contracts);

  const displayParams = logic.inputs && logic.inputs.length > 0;

  return (
    <>
      {displayParams &&
        <div className="params-container">
          {logic.inputs?.map((input, index) => {
            return <>
              <VSCodeTextField className="text-field" {...logic.form.register(`inputs.${index}` as const, {
                required: true,
                valueAsNumber: input.type.includes('int'),
              })}>
                {input.name} ({input.type})
              </VSCodeTextField>
            </>;
          })}
        </div>
      }
    </>
  );
};