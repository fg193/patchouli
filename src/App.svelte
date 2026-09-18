<script lang="ts">
  import BookCard from "./lib/components/BookCard.svelte";
  import EngineSelect from "./lib/components/EngineSelect.svelte";
  import LoadingIndicator from "./lib/components/LoadingIndicator.svelte";
  import { ClipboardList, Menu, ScanBarcode, Search } from "@lucide/svelte";
  import { tauriSearchApi } from "./lib/api/client";
  import { getProvider } from "./lib/api/providers";
  import type { ApiFailure, BookResult, ProviderId } from "./lib/api/contracts";

  let provider = $state<ProviderId>("douban");
  let resultProvider = $state<ProviderId>("douban");
  let query = $state("");
  let results = $state<BookResult[]>([]);
  let total = $state(0);
  let page = $state(1);
  let pageSize = 20;
  let loading = $state(false);
  let loadingMore = $state(false);
  let loadMoreError = $state("");
  let loadMoreSentinel: HTMLDivElement | undefined = $state();
  let error = $state("");
  let businessCode = $state<number | undefined>(undefined);
  let errorMessage = $state("");
  let hasSearched = $state(false);
  let route = $state<"home" | "search">("home");
  let headerVisible = $state(true);

  const selectedProvider = $derived(getProvider(provider));
  const resultSource = $derived(getProvider(resultProvider));
  const hasMore = $derived(results.length < total);

  $effect(() => {
    const updateRoute = () => {
      route = window.location.hash.startsWith("#/search") ? "search" : "home";
      hasSearched = route === "search";
    };

    updateRoute();
    window.addEventListener("hashchange", updateRoute);
    return () => window.removeEventListener("hashchange", updateRoute);
  });

  $effect(() => {
    let lastScrollY = window.scrollY;
    let frame: number | undefined;

    const updateHeader = () => {
      frame = undefined;
      const currentScrollY = window.scrollY;
      const delta = currentScrollY - lastScrollY;
      if (currentScrollY <= 8 || delta < 0) headerVisible = true;
      else if (delta > 0) headerVisible = false;
      lastScrollY = currentScrollY;
    };
    const onScroll = () => {
      if (frame === undefined) frame = requestAnimationFrame(updateHeader);
    };

    window.addEventListener("scroll", onScroll, { passive: true });
    return () => {
      window.removeEventListener("scroll", onScroll);
      if (frame !== undefined) cancelAnimationFrame(frame);
    };
  });

  $effect(() => {
    if (!loadMoreSentinel || !hasMore || loading || loadingMore) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0]?.isIntersecting) loadNextPage();
      },
      { rootMargin: "400px" },
    );
    observer.observe(loadMoreSentinel);
    return () => observer.disconnect();
  });

  async function search() {
    const normalized = query.trim();
    if (!normalized || loading) return;

    loading = true;
    page = 1;
    loadMoreError = "";
    error = "";
    businessCode = undefined;
    errorMessage = "";
    hasSearched = true;
    resultProvider = provider;
    if (window.location.hash !== "#/search") window.location.hash = "/search";
    try {
      const response = await tauriSearchApi.search({
        provider,
        query: normalized,
        page: 1,
        pageSize: 20,
      });
      results = response.items;
      total = response.total;
    } catch (cause) {
      const failure = cause as Partial<ApiFailure> | string;
      const message =
        typeof failure === "string"
          ? failure
          : (failure.message ?? "检索暂时不可用，请稍后再试。");
      businessCode =
        typeof failure === "string" ? undefined : failure.businessCode;
      errorMessage = message;
      error =
        typeof failure === "string" ||
        !failure.code ||
        failure.code === "NETWORK_ERROR"
          ? message
          : `${resultSource.name}：${message}`;
      results = [];
      total = 0;
    } finally {
      loading = false;
    }
  }

  async function loadNextPage() {
    const normalized = query.trim();
    if (!normalized || loading || loadingMore || !hasMore) return;

    loadingMore = true;
    loadMoreError = "";
    const nextPage = page + 1;
    try {
      const response = await tauriSearchApi.search({
        provider: resultProvider,
        query: normalized,
        page: nextPage,
        pageSize,
      });
      results = [...results, ...response.items];
      total = response.total;
      page = response.page;
    } catch (cause) {
      const failure = cause as Partial<ApiFailure> | string;
      loadMoreError =
        typeof failure === "string"
          ? failure
          : (failure.message ?? "下一页暂时加载失败，请稍后重试。 ");
    } finally {
      loadingMore = false;
    }
  }
</script>

<svelte:head
  ><title
    >{hasSearched
      ? `${query} · ${selectedProvider.shortName}`
      : "Patchouli · 图书检索"}</title
  ></svelte:head
>

<main class:searched={route === "search"}>
  <header class:hidden={!headerVisible}>
    <button class="icon-button menu-button" type="button" aria-label="打开菜单">
      <Menu aria-hidden="true" />
    </button>
    <form
      id="search-form"
      class="search-box header-search"
      onsubmit={(event) => {
        event.preventDefault();
        search();
      }}
    >
      <EngineSelect value={provider} onchange={(id) => (provider = id)} />
      <input
        bind:value={query}
        aria-label="搜索关键词"
        placeholder={selectedProvider.placeholder}
        autocomplete="off"
      />
      <button
        class="search-submit"
        type="submit"
        disabled={!query.trim() || loading}
        aria-label="搜索"
      >
        {#if loading}<span class="spinner"></span>{:else}<Search
            aria-hidden="true"
          />{/if}
      </button>
    </form>
    <nav class="header-actions" aria-label="快捷操作">
      <button class="icon-button" type="button" aria-label="扫码">
        <ScanBarcode aria-hidden="true" />
      </button>
      <button class="icon-button" type="button" aria-label="书单">
        <ClipboardList aria-hidden="true" />
      </button>
    </nav>
  </header>

  {#if route === "home"}
    <section class="hero" aria-labelledby="page-title">
      <div class="eyebrow"><span></span>在千万册书中，找到那一页</div>
      <h1 id="page-title">从一处开始，<em>寻遍群书。</em></h1>
      <p class="intro">
        聚合豆瓣、首都图书馆与国家图书馆馆藏，一次检索，少一点辗转。
      </p>
    </section>
  {/if}

  {#if route === "search"}
    <section class="results" aria-live="polite">
      {#if loading}
        <div class="status">
          <LoadingIndicator />
        </div>
      {:else if error}
        <div class="status error-state">
          <span>!</span>
          <h2>这一页暂时翻不开</h2>
          {#if businessCode !== undefined}<p>错误码：{businessCode}</p>{/if}
          <p>错误消息：{errorMessage || error}</p>
          <button type="button">去 GitHub 反馈</button>
        </div>
      {:else if results.length === 0}
        <div class="status">
          <span class="empty-glyph">○</span>
          <h2>未找到相关文献</h2>
          <p>试试更短的书名，或换个检索引擎？</p>
        </div>
      {:else}
        <div class="result-heading">
          <p>
            在<span>{resultSource.shortName}</span>找到 {total.toLocaleString(
              "zh-CN",
            )} 条结果
          </p>
          <small>按相关度排序</small>
        </div>
        <div class="book-list">
          {#each results as book, index (book.providerId + book.id)}<BookCard
              {book}
              {index}
            />{/each}
        </div>
        <div bind:this={loadMoreSentinel} class="load-more" aria-live="polite">
          {#if loadingMore}
            <LoadingIndicator />
          {:else if loadMoreError}
            <span>{loadMoreError}</span>
            <button type="button" onclick={loadNextPage}>重试</button>
          {:else if !hasMore}
            <span>已经到底了</span>
          {/if}
        </div>
      {/if}
    </section>
  {/if}
</main>
