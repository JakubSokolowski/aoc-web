const getProblemButton = (year: number, day: number) => cy.getByDataTest(`problem-${year}-${day}`);
const hasProperSolution = (part: 'first' | 'second', solution: string) => {
    cy.getByDataTest(`solve-${part}`).click();
    cy.getByDataTest(`solution-${part}`, {timeout: 30000}).contains(solution);
}
interface YearSolutions {
    value: number;
    days: DaySolutions[]
}

interface DaySolutions {
    value: number,
    first: string,
    second: string
}

describe('aoc-web-front', () => {
    beforeEach(() => cy.visit('/'));

    it('should display container with game of life', () => {
        cy.getByDataTest('description').contains(
            'Rust AOC solutions, run in browser with WASM:'
        );
    });

    it.only('should show proper solutions', () => {
        const solutions: YearSolutions[] = [
            {
                value: 2021,
                days: [
                    {
                        value: 1,
                        first: '1248',
                        second: '1298'
                    },
                    {
                        value: 2,
                        first: '1804520',
                        second: '1971095320'
                    },
                    {
                        value: 3,
                        first: '3959450',
                        second: '7440311'
                    },
                    {
                        value: 4,
                        first: '58412',
                        second: '10030'
                    },
                    {
                        value: 5,
                        first: '6572',
                        second: '21466'
                    },
                    {
                        value: 6,
                        first: '372300',
                        second: '1675781200288'
                    },
                    {
                        value: 7,
                        first: '342641',
                        second: '93006301'
                    },
                    {
                        value: 8,
                        first: '412',
                        second: '978171'
                    },
                    {
                        value: 9,
                        first: '502',
                        second: '1330560'
                    },
                    {
                        value: 10,
                        first: '399153',
                        second: '2995077699'
                    },
                    {
                        value: 11,
                        first: '1686',
                        second: '360'
                    },
                    {
                        value: 12,
                        first: '5212',
                        second: '134862'
                    },
                    {
                        value: 14,
                        first: '2068',
                        second: '2158894777814'
                    },
                    {
                        value: 15,
                        first: '595',
                        second: '2914'
                    },
                    {
                        value: 16,
                        first: '925',
                        second: '342997120375'
                    },
                    {
                        value: 17,
                        first: '5671',
                        second: '4556'
                    },
                    {
                        value: 18,
                        first: '4132',
                        second: '4685'
                    },
                ]
            },
            {
                value: 2022,
                days: [
                    {
                        value: 1,
                        first: '75501',
                        second: '215594'
                    },
                    {
                        value: 2,
                        first: '13682',
                        second: '12881'
                    },
                    {
                        value: 3,
                        first: '8401',
                        second: '2641'
                    },
                    {
                        value: 4,
                        first: '532',
                        second: '854'
                    },
                    {
                        value: 5,
                        first: 'GFTNRBZPF',
                        second: 'VRQWPDSGP'
                    },
                    {
                        value: 6,
                        first: '1042',
                        second: '2980'
                    },
                    {
                        value: 7,
                        first: '1427048',
                        second: '2940614'
                    },
                    {
                        value: 8,
                        first: '1700',
                        second: '470596'
                    }
                ]
            }
        ];

        solutions.forEach((year) => {
            year.days.forEach((day) => {
                getProblemButton(year.value, day.value).click();
                hasProperSolution("first", day.first);
                hasProperSolution("second", day.second);
            });
        });
    });
});
