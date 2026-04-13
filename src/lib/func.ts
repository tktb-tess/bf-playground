import { getWorker } from './get_worker';
import { BFRuntimeError, parseError, type Result } from './util';
import type { BFOptions } from './wasm/wasm_part';

export const exec = async (
  code: string,
  options: BFOptions = {},
): Promise<Result<string, BFRuntimeError>> => {
  const worker = getWorker();
  try {
    const value = await worker.postMessage({ code, options });
    return {
      success: true,
      value,
    };
  } catch (e) {
    return {
      success: false,
      error: parseError(e),
    };
  }
};
