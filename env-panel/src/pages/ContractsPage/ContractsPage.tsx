import './ContractsPage.css';
import { ResourceManager } from '@hooks/useResourceManager.ts';
import { VSCode } from '@/types';
import { EditableDataGrid } from '@components/EditableDataGrid/EditableDataGrid.tsx';
import { useContractsPageLogic } from '@pages/ContractsPage/ContractsPage.logic.ts';

export const ContractsPage = (props: { vscode: VSCode; resourceManager: ResourceManager }) => {
  const logic = useContractsPageLogic(props.vscode);

  return (
    <div className="contracts-container">
      <EditableDataGrid
        headers={['Name', 'Address', 'Abi', 'Chain ID', 'Rpc']}
        data={props.resourceManager.interactContracts}
        deleteCallback={logic.deleteContract}
        editCallback={logic.editContract}
      />
    </div>
  );
};
