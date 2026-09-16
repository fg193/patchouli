import type { ProviderId } from "./contracts";

export interface Provider {
  id: ProviderId;
  name: string;
  shortName: string;
  description: string;
  placeholder: string;
  accent: string;
}

export const PROVIDERS: readonly Provider[] = [
  {
    id: "douban",
    name: "豆瓣读书",
    shortName: "豆瓣",
    description: "智能匹配题名、作者、出版社等",
    placeholder: "可同时搜标题/作者/出版社/ISBN",
    accent: "#328a45",
  },
  {
    id: "nlc",
    name: "国家图书馆",
    shortName: "国图",
    description: "图书、论文、期刊报纸一网打尽",
    placeholder: "任选其一：标题/作者/出版社/ISBN",
    accent: "#32808a",
  },
  {
    id: "clcn",
    name: "首都图书馆",
    shortName: "首图",
    description: "检索 440 余家北京市公共图书馆",
    placeholder: "仅支持搜索标题",
    accent: "#8a3832",
  },
] as const;

export function getProvider(id: ProviderId): Provider {
  return PROVIDERS.find((provider) => provider.id === id) ?? PROVIDERS[0];
}
