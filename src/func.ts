import { AsyncWorker } from './async_worker';
import { type UnknownObj, BFRuntimeError } from './util';
import { ResultAsync } from 'neverthrow';
import type { BFOptions } from './wasm/wasm_part';
import Worker from './wasm_wrapper?worker';

const worker = new AsyncWorker<{ code: string; options: BFOptions }, string>(
  new Worker()
);

export const exec = (code: string, options: BFOptions = {}) => {
  worker.postMessage({ code, options });

  return ResultAsync.fromPromise(worker.receive(), (_e) => {
    if (_e == null) {
      return BFRuntimeError('UnidentifiedError');
    }

    const e = _e as UnknownObj;

    if (e.name === 'BFRuntimeError') {
      return e as unknown as BFRuntimeError;
    } else {
      return BFRuntimeError(
        e instanceof Error ? e.message : 'UnidentifiedError',
        e instanceof Error ? e.cause : e
      );
    }
  });
};
