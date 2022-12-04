export interface DayInfo {
    value: number;
    first?: string;
    second?: string;
    firstMessage?: ResultMessage;
    secondMessage?: ResultMessage;
}

export type SimpleMessage = string;
export type ComplexMessage = {
    display: 'span' | 'pre';
    text?: string;
};

export type ResultMessage = SimpleMessage | ComplexMessage;

export interface SolvedYearProblems {
    year: number;
    days: DayInfo[];
}

export interface ProblemInfo {
    year: number;
    day: DayInfo;
}
