import type { EngineId } from "./contracts";

export interface SearchEngine {
  id: EngineId;
  name: string;
  shortName: string;
  description: string;
  accent: string;
}

export const SEARCH_ENGINES: readonly SearchEngine[] = [
  {
    id: "douban",
    name: "豆瓣读书",
    shortName: "豆瓣",
    description: "书评、版本与出版信息",
    accent: "#287e5b",
  },
  {
    id: "clcn",
    name: "首都图书馆",
    shortName: "首图",
    description: "首图与全市联合馆藏",
    accent: "#9d552f",
  },
  {
    id: "nlc",
    name: "国家图书馆",
    shortName: "国图",
    description: "国家图书馆馆藏目录",
    accent: "#495d8b",
  },
] as const;

export function getEngine(id: EngineId): SearchEngine {
  return SEARCH_ENGINES.find((engine) => engine.id === id) ?? SEARCH_ENGINES[0];
}
