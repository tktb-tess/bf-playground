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

export const parseError = (_e: unknown) => {
  if (_e instanceof Error) {
    return fromError(_e);
  }
  return BFRuntimeError('unidentified error', `${_e}`);
};

export { BFRuntimeError };

interface WOk<T> {
  success: true;
  value: T;
}

interface WErr<E = unknown> {
  success: false;
  error: E;
}

export type WorkerResult<T, E> = WOk<T> | WErr<E>;

export type WorkerMsg = WorkerResult<string, BFRuntimeError>;
