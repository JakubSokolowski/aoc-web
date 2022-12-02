import styles from './app.module.scss';
import React, { FC, lazy, Suspense } from 'react';

const AocProblem = lazy(() => import('../components/problem'));

interface SolvedYearProblems {
    year: number;
    days: number[];
}

interface ProblemInfo {
    year: number;
    day: number;
}

const solvedProblems: SolvedYearProblems[] = [
    {
        year: 2021,
        days: [1]
    },
    {
        year: 2022,
        days: [1]
    }
];

const YearMenu: FC<{
    data: SolvedYearProblems;
    onProblemClick: (info: ProblemInfo) => void;
}> = ({ data, onProblemClick }) => {
    const { year, days } = data;

    return (
        <div>
            <div style={{ fontWeight: 'bold', fontSize: '30px' }}>{year}</div>
            <div
                style={{
                    display: 'flex',
                    flexDirection: 'row',
                    flexWrap: 'wrap'
                }}
            >
                {days.map((d) => {
                    return (
                        <div
                            key={d}
                            style={{
                                paddingRight: '10px',
                                paddingBottom: '10px'
                            }}
                        >
                            <button
                                onClick={() => onProblemClick({ year, day: d })}
                            >
                                {d}
                            </button>
                        </div>
                    );
                })}
            </div>
        </div>
    );
};

export function App() {
    const [info, setInfo] = React.useState<ProblemInfo>({
        year: solvedProblems[0].year,
        day: solvedProblems[0].days[0]
    });

    return (
        <div className={styles.app}>
            <div>
                {solvedProblems.map((p) => {
                    return (
                        <YearMenu
                            key={p.year}
                            data={p}
                            onProblemClick={(problemInfo) => {
                                setInfo(problemInfo);
                            }}
                        />
                    );
                })}
            </div>
            <main>
                <Suspense fallback={<div>Loading...</div>}>
                    <AocProblem {...info} />
                </Suspense>
            </main>
        </div>
    );
}

export default App;
