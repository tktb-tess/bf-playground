import { mount } from 'svelte';
import AsyncWorker from '@tktb-tess/async-worker';
import * as U from '@tktb-tess/brainf_ck-interpreter';
import './app.css';
import App from './App.svelte';

const o = {
  AsyncWorker,
  ...U,
  __proto__: null,
} as const;

Object.freeze(o);

Object.defineProperty(window, '__tktb_funcs', {
  value: o,
  enumerable: true,
});

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
