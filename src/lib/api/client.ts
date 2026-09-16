import { invoke } from "@tauri-apps/api/core";
import type { SearchRequest, SearchResponse } from "./contracts";

/**
 * The UI depends only on this small port. More resource APIs can live beside
 * search without coupling components to Tauri or individual providers.
 */
export interface SearchApi {
  search(request: SearchRequest): Promise<SearchResponse>;
}

export const tauriSearchApi: SearchApi = {
  search(request) {
    return invoke<SearchResponse>("search_books", { request });
  },
};
