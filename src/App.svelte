<script lang="ts">
  import type { BFRuntimeError } from './util';
  import { exec } from './func';
  import { ResultAsync, okAsync } from 'neverthrow';

  const title = 'BF Playground';

  let code = $state(
    '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.'
  );

  let input = $state('');

  const handleClick = () => {
    resultA = exec(code, { input: input });
  };

  let resultA: ResultAsync<string, BFRuntimeError> = $state(okAsync(''));

  $effect(() => {
    resultA.orTee((e) => {
      for (const [k, v] of Object.entries(e)) {
        console.log(`${k}:`, v);
      }
    });
  });
</script>

<header>
  <h1>{title}</h1>
</header>
<main>
  <p><a href="/.">戻る</a></p>
  <p>Brainf*ck の簡易的な実行環境です。</p>

  <div class="flex flex-col gap-2">
    <div class="flex gap-2 *:flex-[1_1_0] *:min-w-0">
      <div class="flex flex-col">
        <label for="code" class="text-center">コード</label>
        <textarea id="code" bind:value={code}></textarea>
      </div>
      <div class="flex flex-col">
        <label for="input" class="text-center">入力</label>
        <textarea id="input" bind:value={input}></textarea>
      </div>
    </div>
    <button type="button" class="self-center" onclick={handleClick}>
      実行
    </button>
    <label for="result" class="text-center">実行結果</label>
    {#await resultA}
      <textarea id="result" readonly>Executing...</textarea>
    {:then result}
      <textarea
        id="result"
        value={result.isOk()
          ? result.value
          : `${result.error.name}: ${result.error.message}${result.error.cause ? `, ${result.error.cause}` : ''}`}
        readonly
        class={result.isErr() ? 'text-red-500' : ''}
      ></textarea>
    {:catch}
      <p>Unintended Error</p>
    {/await}
  </div>
</main>
<div class="h-10"></div>
