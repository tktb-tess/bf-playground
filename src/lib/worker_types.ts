import type { WorkerResult } from '@tktb-tess/util-fns';
import type { BFOptions } from './wasm/wasm_part';

export type BFMessage = { code: string; options: BFOptions };
export type BFResult = WorkerResult<string>;
