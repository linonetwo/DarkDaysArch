import { useTranslation } from 'react-i18next';

export default function App(): JSX.Element {
  const { t } = useTranslation();
  return (
    <div className="App">
      <header className="App-header">Hi {t('LinOnetwo')}</header>
    </div>
  );
}
