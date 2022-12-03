import React, { lazy, Suspense } from 'react';
import '../styles.scss';
import styles from './app.module.scss';
import { Route, Routes } from 'react-router-dom';

// The lazy import provides an async entrypoint for WASM code
// so for AocProblem and it's children you can just use normal imports
// like: import { Part, run } from '@aoc-web/lib-rs';
// and not have to use import().then(...)
const AocProblem = lazy(() => import('../components/problem-with-data'));

export function App() {
    return (
        <div className={styles.app}>
            <main>
                <Routes>
                    <Route
                        path={'/'}
                        element={
                            <Suspense
                                fallback={<div>Loading WASM module...</div>}
                            >
                                <AocProblem />
                            </Suspense>
                        }
                    />
                    <Route
                        path={'/:year/:day'}
                        element={
                            <Suspense
                                fallback={<div>Loading WASM module...</div>}
                            >
                                <AocProblem />
                            </Suspense>
                        }
                    />
                </Routes>
            </main>
        </div>
    );
}

export default App;
