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
            {
                value: 6,
                firstMessage: 'Population size after 80 days:',
                secondMessage: 'Population size after 256 days:',
            },
            {
                value: 7,
                firstMessage: 'Total fuel spend:',
                secondMessage: 'Total fuel spend:',
            },
            {
                value: 8,
                firstMessage: 'Num times 1,4,7,8 appear:',
                secondMessage: 'Output sum:',
            },
            {
                value: 9,
                firstMessage: 'Heightmap risk:',
                secondMessage: 'Largest basin product:',
            },
            {
                value: 10,
                firstMessage: 'Syntax error score:',
                secondMessage: 'Line completion score:',
            },
            {
                value: 11,
                firstMessage: 'Flash count:',
                secondMessage: 'First simultaneous flash at step:',
            },
            {
                value: 12,
                firstMessage: 'Paths count:',
                secondMessage: 'Paths count:',
            },
            {
                value: 13,
                firstMessage: 'Dots after first fold:',
                secondMessage: {
                    display: 'pre',
                    text: 'Code: \n',
                },
            },
            {
                value: 14,
                firstMessage: 'Difference after 10 steps:',
                secondMessage: 'Difference after 40 steps:',
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
