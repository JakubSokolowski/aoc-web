import { render } from '@testing-library/react';

import App from './app';
import { BrowserRouter } from 'react-router-dom';

// Workaround for "Jest encountered an unexpected token" when testing
// component that has some child that imports some wasm module
// This could probably be solved with some jest config option like moduleNameMapper
// but mocking whole component works also. GameOfLife import is lazy so the factory
// needs to return a function or class, instead of usual component:
// { GameOfLife: () => Game of Life was here' } <- will not work
jest.mock('../components/problem', () => () => 'Problem?');

describe('App', () => {
    it('should render successfully', () => {
        const { baseElement } = render(<BrowserRouter>
            <App />
        </BrowserRouter>);

        expect(baseElement).toBeTruthy();
    });
});
