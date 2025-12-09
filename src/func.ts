import { asyncWorkerFactory } from './async_worker';
import { type UnknownObj, BFRuntimeError } from './util';
import { ResultAsync } from 'neverthrow';
import type { BFExecOptions } from './wasm/wasm_part';

const w = new Worker(new URL('worker.ts', import.meta.url), { type: 'module' });

const worker = asyncWorkerFactory<
  { code: string; options: BFExecOptions },
  string
>(w);

export const exec = (code: string, options: BFExecOptions = {}) => {
  worker.postMessage({ code, options });

  const ans = worker.receive().then((a) => {
    if (a == undefined) {
      throw Error('!');
    }
    return a;
  });

  return ResultAsync.fromPromise(ans, (_e) => {
    if (_e == null) {
      return BFRuntimeError('UnidentifiedError');
    }

    const e = _e as UnknownObj;

    if (e?.name === 'BFRuntimeError') {
      return e as unknown as BFRuntimeError;
    } else {
      return BFRuntimeError(
        e instanceof Error ? e.message : 'UnidentifiedError',
        e instanceof Error ? e.cause : e
      );
    }
  });
};
