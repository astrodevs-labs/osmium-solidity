import './EnvironmentsPage.css';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { VSCode } from '@/types';
import { EditableDataGrid } from '@components/EditableDataGrid/EditableDataGrid.tsx';
import { useEnvironmentsPageLogic } from '@pages/EnvironmentsPage/EnvironmentsPage.logic.ts';
import { VSCodeButton, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import { FormProvider } from 'react-hook-form';

export const EnvironmentsPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useEnvironmentsPageLogic(props.vscode);

  return (
    <FormProvider {...logic.form}>
      <form className="environments-container" onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
        <div style={{ display: 'flex', flexDirection: 'row', gap: '1rem', marginBottom: '1rem' }}>
          <div style={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
            <VSCodeTextField
              {...logic.form.register(`name`, {
                required: true,
              })}
            >
              Name
            </VSCodeTextField>
            {logic.form.formState.errors.name && (
              <span style={{ color: 'var(--vscode-errorForeground)' }}>Name is required</span>
            )}
          </div>
          <div style={{ width: '100%', display: 'flex', flexDirection: 'column' }}>
            <VSCodeTextField
              {...logic.form.register(`rpc`, {
                required: true,
              })}
            >
              Rpc
            </VSCodeTextField>
            {logic.form.formState.errors.rpc && (
              <span style={{ color: 'var(--vscode-errorForeground)' }}>Rpc is required</span>
            )}
          </div>
        </div>
        <VSCodeButton style={{ width: '100%', marginBottom: '1rem' }} type="submit">
          Add
        </VSCodeButton>
        <EditableDataGrid
          headers={['Name', 'Rpc']}
          data={props.resourceManager.environments}
          deleteCallback={logic.deleteEnvironment}
          editCallback={logic.editEnvironment}
          gridId="environments-grid"
        />
      </form>
    </FormProvider>
  );
};
