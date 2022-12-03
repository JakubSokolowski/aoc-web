import { StrictMode } from 'react';
import * as ReactDOM from 'react-dom';

import App from './app/app';
import { HashRouter } from 'react-router-dom';

ReactDOM.render(
    <StrictMode>
        <HashRouter basename='/'>
            <App />
        </HashRouter>
    </StrictMode>,
    document.getElementById('root')
);
