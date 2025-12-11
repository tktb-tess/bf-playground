import init, { exec, type BFExecOptions } from './wasm/wasm_part';
import { BFRuntimeError, fromError, type Result } from './util';

globalThis.addEventListener(
  'message',
  async (
    e: MessageEvent<[number, { code: string; options: BFExecOptions }]>
  ) => {
    const [id, req] = e.data;
    const { code, options } = req;

    try {
      await init();
      const value = exec(code, options);

      const ans = {
        success: true,
        value,
      } as const satisfies Result<string, BFRuntimeError>;

      postMessage([id, ans]);
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
      } as const satisfies Result<string, BFRuntimeError>;

      postMessage([id, ans]);
    }
  }
);
