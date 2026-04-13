<svelte:options runes />

<script lang="ts">
  import '../app.css';
  import Fields from '$lib/Fields.svelte';
  import OGP from '$lib/OGP.svelte';

  const title = 'BF Playground';
</script>

<OGP {title} url="/bf-playground/" description="brainf*ckのplayground" />

<div class="_cont_">
  <header>
    <h1 id="title">{title}</h1>
  </header>
  <main>
    <p class="back-btn"><a href="https://tktb-tess.github.io">戻る</a></p>
    <p>Brainf*ck の簡易的な実行環境です。</p>
    <Fields />
  </main>
  <footer></footer>
</div>

<style lang="postcss">
  @reference '../app.css';
  @layer components {
    ._cont_ {
      @apply max-xl:flow-root xl:grid;
      --side: minmax(min(14rem, 50%), 1fr);
      --main: minmax(0, 72rem);

      grid-template-areas:
        'h h h'
        'l m r'
        'f f f';

      grid-template-columns:
        var(--side)
        var(--main)
        var(--side);

      grid-template-rows: auto 1fr auto;
    }
    header,
    main,
    footer {
      @apply flow-root;
    }

    main {
      @apply px-4 pbe-36 bg-main min-block-lvh;
      grid-area: m;
    }

    #title {
      @apply max-md:text-4xl md:text-5xl font-extralight text-balance my-10 text-center;
    }

    header {
      grid-area: h;
    }

    footer {
      grid-area: f;
    }

    .back-btn {
      @apply mbs-paragraph;
    }
  }
</style>
