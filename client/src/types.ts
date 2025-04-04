export type ResultsText = string | {
  [packageName: string]: {
    current: string;
    wanted: string;
    latest: string;
  };
}
