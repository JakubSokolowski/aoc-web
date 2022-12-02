export interface DayInfo {
    value: number;
    firstMessage?: string;
    secondMessage?: string;
}

export interface SolvedYearProblems {
    year: number;
    days: DayInfo[];
}

export interface ProblemInfo {
    year: number;
    day: DayInfo;
}
