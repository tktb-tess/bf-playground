export type BFExecArguments = {
  code: string;
  options: {
    input?: string;
    initBuffLength?: number;
  };
};