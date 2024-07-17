/* eslint-disable @typescript-eslint/no-explicit-any */
import { VSCodeButton, VSCodeDataGrid, VSCodeDataGridCell, VSCodeDataGridRow } from '@vscode/webview-ui-toolkit/react';
import { useEditableDataGridLogic } from '@components/EditableDataGrid/EditableDataGrid.logic.ts';
import './EditableDataGrid.css';

interface EditableDataGridProps {
  headers: string[];
  data: any[];
  deleteCallback: (id: string) => void;
  editCallback: (id: string, key: string, value: string) => void;
}

export const EditableDataGrid = (props: EditableDataGridProps) => {
  const logic = useEditableDataGridLogic(props.deleteCallback, props.editCallback);

  return (
    <VSCodeDataGrid id="data-grid">
      <VSCodeDataGridRow>
        {props.headers.map((header, index) => (
          <VSCodeDataGridCell cellType="columnheader" grid-column={index + 1}>
            {header}
          </VSCodeDataGridCell>
        ))}
        <VSCodeDataGridCell cellType="columnheader" grid-column={props.headers.length + 1} />
      </VSCodeDataGridRow>
      {props.data.map((line) => {
        const keys = Object.keys(line).filter((key) => key !== 'id');
        const cells = keys.map((key, index) => (
          <VSCodeDataGridCell className={`${line.id} ${key}`} id="editable-cell" grid-column={index + 1}>
            {line[key]}
          </VSCodeDataGridCell>
        ));
        cells.push(
          <VSCodeDataGridCell className="delete-cell" grid-column={keys.length + 1}>
            <VSCodeButton onClick={logic.deleteRow} id={line.id}>
              Delete
            </VSCodeButton>
          </VSCodeDataGridCell>,
        );

        return <VSCodeDataGridRow>{cells}</VSCodeDataGridRow>;
      })}
    </VSCodeDataGrid>
  );
};
