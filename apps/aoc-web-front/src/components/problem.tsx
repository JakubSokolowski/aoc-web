import { FC, useEffect, useState } from 'react';
import { environment } from '../environments/environment';
import { Part, run } from '@aoc-web/lib-rs';

function fetchProblemData(year: number, day: number): Promise<Response> {
  const prefix = environment.deployUrl
    ? `/${environment.deployUrl}/assets/problems`
    : '/assets/problems';
  const url = `${prefix}/${year}/${day}.txt`;
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

const AocProblem: FC<{ year: number; day: number }> = ({ year, day }) => {
  const [problem, loading] = useProblemData(year, day);
  const [part1, setPart1] = useState<string | null>(null);
  const [part2, setPart2] = useState<string | null>(null);

  useEffect(() => {
    if (problem) {
      setPart1(run(year, day, Part.First, problem));
      setPart2(run(year, day, Part.Second, problem));
    }
  }, [problem]);

  return (
    <div>
      <div data-test={'problem'}>
        Year {2021} day {1} solution:
      </div>
      <div>Part 1 : {part1}</div>
      <div>Part 2 : {part2}</div>
    </div>
  );
};

export default AocProblem;
