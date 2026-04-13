<script lang="ts">
  interface Props {
    readonly title: string;
    readonly url: string;
    readonly description?: string;
  }

  const { title, url: u, description = title }: Props = $props();
  const url = $derived(new URL(u, 'https://tktb-tess.github.io').href);
  const linkCard = 'https://tktb-tess.github.io/link-card.png';
</script>

<svelte:head>
  <!-- OGP -->
  <meta property="og:site_name" content={title} />
  <meta property="og:image" content={linkCard} />
  <meta property="og:type" content="website" />
  <meta property="og:title" content={title} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={url} />
  <!-- twitter card -->
  <meta name="twitter:card" content="summary" />
  <meta name="twitter:site" content="@triethylamineq" />
  <meta name="twitter:image" content={linkCard} />
  <meta name="twitter:title" content={title} />
  <meta name="twitter:description" content={description} />
  <meta name="description" content={description} />
  <title>{title}</title>
</svelte:head>
