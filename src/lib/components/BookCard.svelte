<script lang="ts">
  import type { BookResult } from "../api/contracts";
  import { ClipboardPlus } from "@lucide/svelte";
  let { book, index }: { book: BookResult; index: number } = $props();
  let imageFailed = $state(false);
</script>

<article class="book" style={`--delay:${Math.min(index, 8) * 45}ms`}>
  <div class="cover">
    {#if book.coverUrl && !imageFailed}
      <img
        src={book.coverUrl}
        alt={`${book.title}封面`}
        loading="lazy"
        onerror={() => (imageFailed = true)}
      />
    {:else}
      <span>{book.title.slice(0, 1)}</span>
    {/if}
  </div>
  <div class="content">
    <div class="title-line">
      <div>
        {#if book.detailUrl}
          <a
            class="title-link"
            href={book.detailUrl}
            target="_blank"
            rel="noreferrer"><h2>{book.title}</h2></a
          >
        {:else}
          <h2>{book.title}</h2>
        {/if}
        {#each book.subtitles as subtitle}<p class="subtitle">
            {subtitle}
          </p>{/each}
      </div>
      {#if book.rating !== undefined}<span class="rating"
          >★ {book.rating.toFixed(1)}</span
        >{/if}
    </div>
    <p class="byline">
      {book.authors.length ? book.authors.join(" / ") : "作者不详"}
      {#if book.publisher}<span>·</span>{book.publisher}{/if}
      {#if book.publicationDate}<span>·</span>{book.publicationDate}{/if}
    </p>
    {#if book.summary}<p class="summary">{book.summary}</p>{/if}
    <div class="meta-row">
      {#if book.documentType || book.classmark}<div class="meta">
          {#if book.documentType}<span>{book.documentType}</span>{/if}
          {#if book.classmark}<span>{book.classmark}</span>{/if}
        </div>{/if}
      <button class="add-to-wishlist" type="button" aria-label={`将《${book.title}》加入书单`}>
        <ClipboardPlus aria-hidden="true" />
      </button>
    </div>
  </div>
</article>

<style>
  .book {
    display: grid;
    grid-template-columns: 92px 1fr;
    gap: 22px;
    padding: 24px 0;
    border-top: 1px solid var(--line);
    animation: rise 0.42s both;
    animation-delay: var(--delay);
  }
  .cover {
    width: 92px;
    aspect-ratio: 2 / 3;
    border-radius: 5px 8px 8px 5px;
    overflow: hidden;
    background: #d9d4c8;
    box-shadow: 3px 5px 14px rgba(35, 31, 25, 0.13);
  }
  .cover img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .cover span {
    display: grid;
    height: 100%;
    place-items: center;
    color: #747064;
    font-family: var(--serif);
    font-size: 34px;
  }
  .content {
    min-width: 0;
    padding: 2px 0;
  }
  .title-line {
    display: flex;
    justify-content: space-between;
    gap: 24px;
  }
  h2 {
    margin: 0;
    color: var(--ink);
    font-family: var(--serif);
    font-size: 21px;
    font-weight: 650;
    line-height: 1.35;
  }
  .title-link {
    color: inherit;
    text-decoration: none;
  }
  .title-link:hover h2 {
    color: var(--green);
  }
  .subtitle {
    margin: 3px 0 0;
    color: var(--muted);
    font-size: 13px;
  }
  .rating {
    flex: none;
    color: #a26320;
    font-size: 13px;
  }
  .byline {
    margin: 10px 0 0;
    color: #575951;
    font-size: 13px;
  }
  .byline span {
    margin: 0 5px;
    color: #aca99d;
  }
  .summary {
    margin: 13px 0 0;
    color: #707168;
    font-size: 13px;
    line-height: 1.7;
    display: -webkit-box;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .meta {
    display: flex;
    gap: 9px;
    align-items: center;
  }
  .meta-row {
    display: flex;
    align-items: center;
    gap: 12px;
    min-height: 28px;
    margin-top: 15px;
  }
  .meta span {
    padding: 4px 8px;
    border-radius: 5px;
    background: var(--paper-dark);
    color: var(--muted);
    font-size: 10px;
  }
  .add-to-wishlist {
    display: inline-grid;
    place-items: center;
    margin-left: auto;
    padding: 4px;
    border: 0;
    background: transparent;
    color: var(--green);
    cursor: pointer;
  }
  .add-to-wishlist:hover {
    color: var(--ink);
  }
  .add-to-wishlist :global(svg) {
    width: 20px;
    height: 20px;
  }
  @keyframes rise {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
  }
  @media (max-width: 620px) {
    .book {
      grid-template-columns: 70px 1fr;
      gap: 15px;
    }
    .cover {
      width: 70px;
    }
    .summary {
      display: none;
    }
    .title-line {
      gap: 8px;
    }
  }
</style>
