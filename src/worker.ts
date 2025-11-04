import { exec } from '@tktb-tess/brainf_ck-interpreter';
import type { BFExecArguments } from './type';
import type { MessageResult } from '@tktb-tess/async-worker';

globalThis.addEventListener(
  'message',
  (e: MessageEvent<[number, BFExecArguments]>) => {
    const [id, req] = e.data;
    const { code, options } = req;

    try {
      const value = exec(code, options);

      const ans = {
        success: true,
        value,
      } satisfies MessageResult<string, Error>;

      postMessage([id, ans]);
    } catch (e) {
      const ans = {
        success: false,
        error:
          e instanceof Error ? e : Error('UnidentifiedError', { cause: e }),
      } satisfies MessageResult<string, Error>;
      postMessage([id, ans]);
    }
  }
);
