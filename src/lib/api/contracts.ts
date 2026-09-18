export type ProviderId = "douban" | "clcn" | "nlc";

export interface SearchRequest {
  provider: ProviderId;
  query: string;
  page?: number;
  pageSize?: number;
}

export interface BookResult {
  id: string;
  title: string;
  subtitles: string[];
  documentType?: string;
  classmark?: string;
  authors: string[];
  publisher?: string;
  publicationDate?: string;
  coverUrl?: string;
  isbn?: string;
  summary?: string;
  rating?: number;
  detailUrl?: string;
  providerId: ProviderId;
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
  businessCode?: number;
  response?: string;
}
