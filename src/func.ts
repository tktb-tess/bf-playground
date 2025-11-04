import AsyncWorker from '@tktb-tess/async-worker';
import type { BFExecArguments } from './type';
import { ResultAsync } from 'neverthrow';

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

  return ResultAsync.fromPromise(ans, (e) => {
    if (e instanceof Error) {
      return e;
    } else {
      return Error(`${e}`, { cause: e });
    }
  });
};
