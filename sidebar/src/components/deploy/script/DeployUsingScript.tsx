import { VSCode } from '@/types';
import { Environment, Scripts } from '@backend/actions/types';
import { VSCodeButton, VSCodeDivider, VSCodeDropdown, VSCodeOption } from '@vscode/webview-ui-toolkit/react';
import './DeployUsingScript.css';
import { useDeployUsingScript } from './DeployUsingScript.logic.ts';

export const DeployUsingScript = ({
  scripts,
  vscode,
  environments,
}: {
  scripts: Scripts;
  vscode: VSCode;
  environments: Environment[];
}) => {
  const logic = useDeployUsingScript(vscode);

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
                  <VSCodeOption value={environment.id}>
                    {environment.name} ({environment.rpc})
                  </VSCodeOption>
                ))}
              </VSCodeDropdown>
              <VSCodeButton className="add-wallet-button" onClick={logic.editEnvironment}>
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
            {scripts?.map((script) => (
              <VSCodeOption value={script.id}>
                {script.name} ({script.path})
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
