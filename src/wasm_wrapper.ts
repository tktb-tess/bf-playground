import init, { exec_inner, type BFOptions } from './wasm/wasm_part';
import { BFRuntimeError, fromError, type WorkerResult } from './util';

globalThis.onmessage = async (
  e: MessageEvent<{ code: string; options: BFOptions }>
) => {
  const { code, options } = e.data;

  try {
    await init();
    const value = exec_inner(code, options);

    const ans = {
      success: true,
      value,
    } as const satisfies WorkerResult<string, BFRuntimeError>;

    postMessage(ans);
  } catch (e) {
    let error: BFRuntimeError;

    if (e instanceof Error) {
      error = fromError(e);
    } else {
      error = BFRuntimeError('Unidentified error');
    }

    const ans = {
      success: false,
      error,
    } as const satisfies WorkerResult<string, BFRuntimeError>;

    postMessage(ans);
  }
};
