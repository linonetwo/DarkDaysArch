import React from 'react';
import ReactDOM from 'react-dom';
import i18n from 'i18next';
import { ThemeProvider } from 'styled-components';
import StyledEngineProvider from '@material-ui/core/StyledEngineProvider';
import DateFnsUtils from '@material-ui/lab/AdapterDateFns';
import LocalizationProvider from '@material-ui/lab/LocalizationProvider';
import CssBaseline from '@material-ui/core/CssBaseline';
import { I18nextProvider } from 'react-i18next';
import { Provider } from 'react-redux';
import 'typeface-roboto/index.css';

import { initI18N } from './setup/i18n';

import App from './App';
import reportWebVitals from './reportWebVitals';
import { lightTheme } from './setup/defaultTheme';
import { store } from './store/store';

ReactDOM.render(
  <React.StrictMode>
    <ThemeProvider theme={lightTheme}>
      <StyledEngineProvider injectFirst>
        <LocalizationProvider dateAdapter={DateFnsUtils}>
          <CssBaseline />
          <React.Suspense fallback={<div />}>
            <I18nextProvider i18n={i18n}>
              <Provider store={store}>
                <App />
              </Provider>
            </I18nextProvider>
          </React.Suspense>
        </LocalizationProvider>
      </StyledEngineProvider>
    </ThemeProvider>
  </React.StrictMode>,
  document.querySelector('#root'),
);
void initI18N();

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
