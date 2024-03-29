import { VSCode } from '@/types';
import { Environment, Scripts } from '@backend/actions/types';
import { useDeployContract } from '@hooks/useDeployContract.ts';
import { useDeployScript } from '@hooks/useDeployScript.ts';
import { VSCodeButton, VSCodeDivider, VSCodeDropdown, VSCodeOption } from '@vscode/webview-ui-toolkit/react';
import './DeployUsingScript.css';

export const DeployUsingScript = ({
  scripts,
  vscode,
  environments,
}: {
  scripts: Scripts;
  vscode: VSCode;
  environments: Environment[];
}) => {
  const logic = useDeployScript();
  const logicContract = useDeployContract(vscode);

  return (
    <div>
      <div>
        <div>DEPLOY USING SCRIPT</div>
        <div className="dropdown-container">
          <div className="dropdown-container">
            <label htmlFor="dropdown-environment" className="label">
              Environment:
            </label>
            <div className="environment-container">
              <VSCodeDropdown
                id="dropdown-environment"
                className="dropdown-environment"
                {...logic.form?.register('environment', { required: true })}
              >
                {environments.map((environment) => (
                  <VSCodeOption>
                    {environment.name} ({environment.rpc})
                  </VSCodeOption>
                ))}
              </VSCodeDropdown>
              <VSCodeButton className="add-wallet-button" onClick={logicContract.editEnvironment}>
                Edit
              </VSCodeButton>
            </div>
          </div>
        </div>
        <div className="dropdown-container">
          <label htmlFor="dropdown" className="label">
            Select script:
          </label>
          <VSCodeDropdown
            id="dropdown"
            {...logic.form?.register('script', {
              required: true,
            })}
          >
            {scripts?.map((scripts) => (
              <VSCodeOption>
                {scripts.name} ({scripts.path})
              </VSCodeOption>
            ))}
          </VSCodeDropdown>
        </div>
      </div>
      <VSCodeDivider className="divider" />
      <VSCodeButton className="submit-button" type="submit">
        Deploy with script
      </VSCodeButton>
      <VSCodeDivider className="divider" />
    </div>
  );
};
