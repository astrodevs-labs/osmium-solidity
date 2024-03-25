import { InteractContract } from '@backend/actions/types';
import { useInteractParams } from '@hooks/useInteractParams.ts';
import { VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './InteractParams.css';

export const InteractParams = ({ contracts }: { contracts: InteractContract[] }) => {
  const { inputs, form } = useInteractParams(contracts);

  const displayParams = inputs && inputs.length > 0;

  return (
    <>
      {displayParams && (
        <div className="params-container">
          {inputs?.map((input, index) => {
            return (
              <>
                <VSCodeTextField
                  className="text-field"
                  {...form.register(`inputs.${index}` as const, {
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
