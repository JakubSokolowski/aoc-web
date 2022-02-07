import styles from './app.module.scss';
import React, { lazy, Suspense } from 'react';

const AocProblem = lazy(() => import('../components/problem'));

export function App() {
  return (
    <div className={styles.app}>
      <main>
        <Suspense fallback={<div>Loading...</div>}>
          <AocProblem />
        </Suspense>
      </main>
    </div>
  );
}

export default App;
