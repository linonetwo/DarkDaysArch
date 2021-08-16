import { useDispatch, useSelector } from 'react-redux';
import { Dispatch, RootState } from 'src/store/store';
import styled from 'styled-components';

const OpenedFilesContainer = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
`;
const OpenedFileItem = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
`;

export function OpenedFiles(): JSX.Element {
  const dispatch = useDispatch<Dispatch>();
  const openedFiles = useSelector((state: RootState) => state.files.openedFiles);

  return (
    <OpenedFilesContainer>
      {openedFiles.map((file) => (
        <OpenedFileItem key={file.path} onClick={() => dispatch.files.selectFile(file.path)}>
          {file.path}
        </OpenedFileItem>
      ))}
    </OpenedFilesContainer>
  );
}
