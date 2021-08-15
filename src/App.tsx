import { useTranslation } from 'react-i18next';
import { SidePanel } from './pages/SidePanel';

export default function App(): JSX.Element {
  const { t } = useTranslation();
  return (
    <div>
      <header className="App-header">Hi {t('LinOnetwo')}</header>
      <SidePanel />
    </div>
  );
}
