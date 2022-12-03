import React, { FC, lazy, Suspense } from 'react';
import { ProblemInfo, SolvedYearProblems } from './models';
import '../styles.scss';
import styles from './app.module.scss';

const AocProblem = lazy(() => import('../components/problem'));

const solvedProblems: SolvedYearProblems[] = [
    {
        year: 2021,
        days: [
            {
                value: 1,
                firstMessage: 'Num measurements larger then previous:',
                secondMessage:
                    'Num measurements larger then previous in 3-wide sliding window:'
            },
            {
                value: 2,
                firstMessage: 'Total area:',
                secondMessage:
                    'Total aim area:'
            }
        ]
    },
    {
        year: 2022,
        days: [
            {
                value: 1,
                firstMessage: 'Total calories top Elf is carrying:',
                secondMessage: 'Total calories top 3 Elves are carrying:'
            },
            {
                value: 2,
                firstMessage: 'Total score for strategy:',
                secondMessage: 'Total score for strategy:'
            },
            {
                value: 3,
                firstMessage: 'Sum of priorities:',
                secondMessage: 'Sum of priorities:'
            }
        ]
    }
];

const YearMenu: FC<{
    selectedProblem: ProblemInfo;
    data: SolvedYearProblems;
    onProblemClick: (info: ProblemInfo) => void;
}> = ({ data, onProblemClick, selectedProblem }) => {
    const { year, days } = data;

    return (
        <div>
            <div className='accent'>{year}</div>
            <div
                style={{
                    display: 'flex',
                    flexDirection: 'row',
                    flexWrap: 'wrap'
                }}
            >
                {days.map((d) => {
                    const selected =
                        year === selectedProblem.year &&
                        d.value === selectedProblem.day.value;
                    return (
                        <div key={d.value}>
                            <button
                                className={`problem-button ${
                                    selected && 'problem-selected'
                                }`}
                                onClick={() => onProblemClick({ year, day: d })}
                            >
                                [{d.value}]
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
                <span data-test='description' className='description'>
                    Rust AOC solutions, run in browser with WASM:
                </span>
                <div className='problem-menu'>
                    {solvedProblems.map((p) => {
                        return (
                            <YearMenu
                                selectedProblem={info}
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
                    <Suspense fallback={<div>Loading WASM module...</div>}>
                        <AocProblem info={info} />
                    </Suspense>
                </main>
            </div>
        </div>
    );
}

export default App;
