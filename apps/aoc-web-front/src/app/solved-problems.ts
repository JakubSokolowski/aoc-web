import { SolvedYearProblems } from './models';

export const solvedProblems: SolvedYearProblems[] = [
    {
        year: 2021,
        days: [
            {
                value: 1,
                firstMessage: 'Num measurements larger then previous:',
                secondMessage:
                    'Num measurements larger then previous in 3-wide sliding window:',
            },
            {
                value: 2,
                firstMessage: 'Total area:',
                secondMessage: 'Total aim area:',
            },
            {
                value: 3,
                firstMessage: 'Power consumption:',
                secondMessage: 'Life support rating:',
            },
            {
                value: 4,
                firstMessage: 'First winner score:',
                secondMessage: 'Last winner score:',
            },
            {
                value: 5,
                firstMessage: 'Number of overlapping points:',
                secondMessage: 'Number of overlapping points with diagonal:',
            },
        ],
    },
    {
        year: 2022,
        days: [
            {
                value: 1,
                firstMessage: 'Total calories top Elf is carrying:',
                secondMessage: 'Total calories top 3 Elves are carrying:',
            },
            {
                value: 2,
                firstMessage: 'Total score for strategy:',
                secondMessage: 'Total score for strategy:',
            },
            {
                value: 3,
                firstMessage: 'Sum of priorities:',
                secondMessage: 'Sum of priorities:',
            },
            {
                value: 4,
                firstMessage: 'Count assignments containing:',
                secondMessage: 'Count assignments overlapping:',
            },
        ],
    },
];
