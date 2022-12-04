import { FC, useEffect, useState } from 'react';
import { environment } from '../environments/environment';
import { Part, run } from '@aoc-web/lib-rs';
import SyntaxHighlighter from 'react-syntax-highlighter';
import { tomorrowNight as codeStyle } from 'react-syntax-highlighter/dist/esm/styles/hljs';
import '../styles.scss';
import { ProblemInfo } from '../app/models';

function fetchProblemData(year: number, day: number): Promise<Response> {
    const prefix = environment.deployUrl
        ? `/${environment.deployUrl}/assets/problems`
        : '/assets/problems';
    const url = `${prefix}/${year}/${day}.txt`;
    return fetch(url);
}

function fetchSolutionCode(year: number, day: number): Promise<Response> {
    const prefix = environment.deployUrl
        ? `/${environment.deployUrl}/assets/source`
        : '/assets/source';
    const url = `${prefix}/aoc_${year}/day${('0' + day).slice(-2)}.rs`;
    return fetch(url);
}

async function checkForError(response: Response): Promise<string> {
    const text = await response.text();

    if (response.status >= 200 && response.status <= 299 && !!text) {
        return text;
    } else {
        throw Error(response.statusText);
    }
}

export const useProblemData = (
    year: number,
    day: number
): [string | null, boolean] => {
    const [problem, setProblem] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        setLoading(true);
        console.log('Fetching data for', year, day);
        fetchProblemData(year, day)
            .then(checkForError)
            .then((data) => {
                setProblem(data);
                setLoading(false);
            })
            .catch(async () => {
                console.error(`Failed to load problem data: ${year} ${day}`);
            });
    }, [year, day]);

    return [problem, loading];
};

const AocProblem: FC<{ info: ProblemInfo }> = ({ info }) => {
    const { year, day } = info;
    const [problem, loading] = useProblemData(year, day.value);
    const [code, setCode] = useState<string>();
    const [showCode, setShowCode] = useState(false);

    useEffect(() => {
        setShowCode(false);
        setCode(undefined);
    }, [year, day]);

    return (
        <div className='problems-section'>
            <div className='problem-header'>
                <span>
                    Year {year} day {day.value}
                </span>
                <a
                    target='_blank'
                    className='desc-link'
                    href={`https://adventofcode.com/${info.year}/day/${info.day.value}`}
                    rel='noreferrer'
                >
                    [Description]
                </a>
                <button
                    className='accent-button'
                    onClick={() => {
                        setShowCode(!showCode);
                        if (!code) {
                            fetchSolutionCode(year, day.value)
                                .then(checkForError)
                                .then((data) => {
                                    // Yeet out the test code if any
                                    setCode(
                                        data.split('#[cfg(test)]')[0].trimEnd()
                                    );
                                })
                                .catch(async () => {
                                    console.error(
                                        `Failed to load solution data: ${year} ${day}`
                                    );
                                });
                        }
                    }}
                >
                    [Solution]
                </button>
                :
            </div>
            <ProblemPart
                problem={problem}
                info={info}
                part={Part.First}
                loading={loading}
            />
            <ProblemPart
                problem={problem}
                info={info}
                part={Part.Second}
                loading={loading}
            />
            {!!code && showCode && (
                <SyntaxHighlighter
                    customStyle={{
                        fontSize: '12px',
                        background: 'none',
                        padding: 0
                    }}
                    showLineNumbers={true}
                    language='rust'
                    style={codeStyle}
                >
                    {code}
                </SyntaxHighlighter>
            )}
        </div>
    );
};

export const ProblemPart: FC<{
    problem: string | null;
    info: ProblemInfo;
    part: Part;
    loading?: boolean;
}> = ({ problem, loading, part, info }) => {
    const { year, day } = info;
    const [time, setTime] = useState<number>();
    const [solving, setSolving] = useState(false);
    const [result, setResult] = useState<string>();

    const runPromise = function(
        year: number,
        day: number,
        part: Part,
        problem: string
    ): Promise<string> {
        return new Promise((resolve, reject) => {
            try {
                const result = run(year, day, part, problem);
                resolve(result);
            } catch (e) {
                reject(e);
            }
        });
    };

    useEffect(() => {
        if(solving && problem) {
            const start = performance.now();
            runPromise(year, day.value, part, problem)
                .then((result) => {
                    setResult(result);
                    setSolving(false);
                    const end = performance.now();
                    setTime(end - start);
                })
                .catch(console.error);
        }
    }, [solving]);


    useEffect(() => {
        setResult(undefined);
        setTime(undefined);
    }, [year, day]);

    return (
        <div className='problem-part'>
            <div className='problem-actions'>
                <span className='part-title'>
                    [Part {part === Part.First ? '1' : '2'}]
                </span>
                <button
                    className='accent-button'
                    disabled={!problem || loading || solving}
                    onClick={() => {
                        if (problem) {
                            setResult(undefined);
                            setSolving(true);
                        }
                    }}
                >
                    {solving ? '[Solving...]' : '[Solve]'}
                </button>
            </div>
            {!!result && !solving && (
                <div>
                    <span>{`${
                        part === Part.First
                            ? day.firstMessage || ''
                            : day.secondMessage || ''
                    }`}</span>
                    <span className='solution'>{` ${result}`}</span>
                    <span>{`, took ${time?.toFixed(1)} ms`}</span>
                </div>
            )}
        </div>
    );
};

export default AocProblem;
