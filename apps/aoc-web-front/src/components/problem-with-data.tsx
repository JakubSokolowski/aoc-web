import { useNavigate, useParams } from 'react-router-dom';
import React from 'react';
import { ProblemInfo } from '../app/models';
import AocProblem from './problem';
import { solvedProblems } from '../app/solved-problems';
import { YearMenu } from '../app/YearMenu';

function latestSolved(): ProblemInfo {
    const latestYear = solvedProblems[solvedProblems.length - 1];
    const { year, days } = latestYear;
    const latestDay = days[days.length - 1];

    return {
        year,
        day: latestDay,
    };
}

function findSolved(year: number, day: number): ProblemInfo | undefined {
    const targetYear = solvedProblems.find((p) => p.year === year);
    if (!targetYear) return;
    const targetDay = targetYear.days.find((d) => d.value === day);
    if (!targetDay) return;

    return {
        year: targetYear.year,
        day: targetDay,
    };
}

const ProblemWithData = () => {
    const navigate = useNavigate();
    const { year, day } = useParams();
    const [info, setInfo] = React.useState<ProblemInfo>();

    React.useEffect(() => {
        if (year && day) {
            const yearNum = Number.parseInt(year);
            const dayNum = Number.parseInt(day);
            if (yearNum && dayNum) {
                const problem = findSolved(yearNum, dayNum);
                if (problem) {
                    setInfo(problem);
                } else {
                    setInfo(latestSolved());
                }
            } else {
                setInfo(latestSolved());
            }
        } else {
            setInfo(latestSolved());
        }
    }, [year, day]);

    return (
        <div>
            {!!info && (
                <div>
                    <span data-test="description" className="description">
                        Rust AOC solutions, run in browser with WASM:
                    </span>
                    <div className="problem-menu">
                        {solvedProblems.map((p) => {
                            return (
                                <YearMenu
                                    selectedProblem={info}
                                    key={p.year}
                                    data={p}
                                    onProblemClick={(problemInfo) => {
                                        setInfo(problemInfo);
                                        navigate(
                                            `/${problemInfo.year}/${problemInfo.day.value}`,
                                            { replace: true }
                                        );
                                    }}
                                />
                            );
                        })}
                    </div>
                    <AocProblem info={info} />
                </div>
            )}
        </div>
    );
};

export default ProblemWithData;
