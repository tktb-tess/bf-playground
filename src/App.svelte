<script lang="ts">
  import type { BFRuntimeError } from './util';
  import { exec } from './func';
  import { ResultAsync, okAsync } from 'neverthrow';

  const title = 'BF Playground';

  let code = $state(
    '++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.',
  );

  let input = $state('');

  const handleClick = () => {
    resultA = exec(code, { input });
  };

  let resultA: ResultAsync<string, BFRuntimeError> = $state(okAsync(''));

  $effect(() => {
    resultA.orTee((e) => {
      Object.entries(e).forEach(([k, v]) => {
        console.log(k, v);
      });
    });
  });
</script>

<header>
  <h1 id="title">{title}</h1>
</header>
<main>
  <p><a href="/.">戻る</a></p>
  <p>Brainf*ck の簡易的な実行環境です。</p>

  <div id="bf-playground">
    <div class="__inputs">
      <section class="__input-section">
        <label for="code">コード</label>
        <textarea id="code" bind:value={code}></textarea>
      </section>
      <section class="__input-section">
        <label for="input">入力</label>
        <textarea id="input" bind:value={input}></textarea>
      </section>
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
          : `${result.error.name}: ${result.error.message}`}
        readonly
        class={result.isErr() ? 'text-red-500' : ''}
      ></textarea>
    {:catch}
      <p>Unexpected Error</p>
    {/await}
  </div>
</main>
<div class="h-10"></div>

<style lang="postcss">
  @reference './app.css';
  @layer components {
    #bf-playground {
      @apply flex flex-col gap-2;
    }

    .__inputs {
      @apply flex max-lg:flex-col gap-2 *:flex-[1_1_0] *:min-w-0;
    }

    .__input-section {
      @apply flex flex-col;

      > :where(label) {
        @apply text-center;
      }

      > :where(textarea#code) {
        @apply break-all;
      }
    }
  }
</style>
