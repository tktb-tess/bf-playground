export type UnknownObj = {
  [key: string | symbol]: unknown;
};

type BFRuntimeError = {
  readonly name: 'BFRuntimeError';
  readonly message: string;
  readonly stack?: string;
  readonly cause?: unknown;
};

const BFRuntimeError = (message: string, cause?: unknown): BFRuntimeError => {
  return {
    name: 'BFRuntimeError',
    message,
    stack: new Error().stack,
    cause,
  };
};

export const fromError = (error: Error): BFRuntimeError => {
  const { message, stack, cause } = error;
  return {
    name: 'BFRuntimeError',
    message,
    stack,
    cause,
  };
};

export { BFRuntimeError };

export type WorkerResult<T, E> =
  | {
      success: true;
      value: T;
    }
  | {
      success: false;
      error: E;
    };
