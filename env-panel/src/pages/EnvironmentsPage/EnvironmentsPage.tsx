import './EnvironmentsPage.css';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { VSCode } from '@/types';
import { EditableDataGrid } from '@components/EditableDataGrid/EditableDataGrid.tsx';
import { useEnvironmentsPageLogic } from '@pages/EnvironmentsPage/EnvironmentsPage.logic.ts';

export const EnvironmentsPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useEnvironmentsPageLogic(props.vscode);

  return (
    <div className="environments-container">
      <EditableDataGrid
        headers={['Name', 'Rpc']}
        data={props.resourceManager.environments}
        deleteCallback={logic.deleteEnvironment}
        editCallback={logic.editEnvironment}
      />
    </div>
  );
};
