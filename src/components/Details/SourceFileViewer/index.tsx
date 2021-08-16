import Editor from '@monaco-editor/react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';

import { Dispatch, RootState } from 'src/store/store';
import styled from 'styled-components';

const NoActiveFileInfo = styled.div``;

export function SourceFileViewer(): JSX.Element {
  const { t } = useTranslation();
  const activeOpenedFile = useSelector((state: RootState) => state.files.openedFiles.find((file) => file.path === state.files.activeOpenedFilePath));
  if (activeOpenedFile === undefined) {
    return <NoActiveFileInfo>{t('Details.NoActiveFile')}</NoActiveFileInfo>;
  }
  if (activeOpenedFile.content === undefined) {
    return <NoActiveFileInfo>{t('Details.ActiveFileIsEmpty')}</NoActiveFileInfo>;
  }
  return <Editor height="90vh" defaultLanguage="json" defaultValue={activeOpenedFile.content} />;
}
