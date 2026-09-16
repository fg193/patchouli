export type EngineId = "douban" | "clcn" | "nlc";

export interface SearchRequest {
  engine: EngineId;
  query: string;
  page?: number;
  pageSize?: number;
}

export interface BookResult {
  id: string;
  title: string;
  subtitle?: string;
  authors: string[];
  publisher?: string;
  publishedAt?: string;
  coverUrl?: string;
  isbn?: string;
  summary?: string;
  rating?: number;
  detailUrl?: string;
  source: EngineId;
}

export interface SearchResponse {
  items: BookResult[];
  total: number;
  page: number;
  pageSize: number;
}

export interface ApiFailure {
  code: string;
  message: string;
}
