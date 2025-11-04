import AsyncWorker from '@tktb-tess/async-worker';
import type { BFExecArguments } from './type';
import { ResultAsync } from 'neverthrow';
import { BFRuntimeError } from '@tktb-tess/brainf_ck-interpreter';

const w = new Worker(new URL('worker.ts', import.meta.url), { type: 'module' });

const worker = new AsyncWorker<BFExecArguments, string>(w);

export const exec = (
  code: BFExecArguments['code'],
  options: BFExecArguments['options'] = {}
) => {
  worker.postMessage({ code, options });

  const ans = worker.receive().then((a) => {
    if (a === undefined) {
      throw Error('!');
    }
    return a;
  });

  return ResultAsync.fromPromise(ans, (_e) => {
    if (_e == null) {
      return new BFRuntimeError('UnidentifiedError');
    }

    const e = _e as {
      name?: string;
    };

    if (e?.name === 'BFRuntimeError') {
      return e as BFRuntimeError;
    } else {
      return new BFRuntimeError(
        e instanceof Error ? e.message : 'UnidentifiedError',
        e instanceof Error ? e.cause : e
      );
    }
  });
};
