import init, { exec_inner } from './wasm/wasm_part';
import { parseError } from './util';
import type { BFOptions } from './wasm/wasm_part';
import type { WorkerMsg } from './util';

globalThis.onmessage = async (
  e: MessageEvent<{ code: string; options: BFOptions }>,
) => {
  const { code, options } = e.data;
  try {
    await init();
    const value = exec_inner(code, options);

    const ans = {
      success: true,
      value,
    } as const satisfies WorkerMsg;

    postMessage(ans);
  } catch (e) {
    const error = parseError(e);

    const ans = {
      success: false,
      error,
    } as const satisfies WorkerMsg;

    postMessage(ans);
  }
};
