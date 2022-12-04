import React, { FC } from 'react';
import { ProblemInfo, SolvedYearProblems } from './models';

export const YearMenu: FC<{
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
                                data-test={`problem-${year}-${d.value}`}
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
