import { mount } from 'svelte';
import AsyncWorker from '@tktb-tess/async-worker';
import * as U from '@tktb-tess/brainf_ck-interpreter';
import './app.css';
import App from './App.svelte';

const iii = { ...U };
Object.defineProperty(iii, Symbol.toStringTag, {
  value: 'BFInterpreter',
});
Object.setPrototypeOf(iii, null);
Object.freeze(iii);

Object.defineProperties(window, {
  BFInterpreter: {
    value: iii,
    enumerable: true,
  },
  AsyncWorker: {
    value: AsyncWorker,
    enumerable: true,
  },
});

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
