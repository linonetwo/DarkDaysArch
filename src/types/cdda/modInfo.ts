/* tslint:disable */
/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

export type CDDAModInfoWithExternalOption = CDDAModInfo | CDDAExternalOption | CDDAItemBlackList;
export type CDDAModInfoAuthors = string | string[];
export type MOD_INFO_Literal = "MOD_INFO";
export type EXTERNAL_OPTION_Literal = "EXTERNAL_OPTION";
export type ITEM_BLACKLIST_Literal = "ITEM_BLACKLIST";
export type ArrayOf_CDDAModInfoWithExternalOption = CDDAModInfoWithExternalOption[];

/**
 * @docs https://github.com/CleverRaven/Cataclysm-DDA/blob/master/doc/MODDING.md#modinfojson
 */
export interface CDDAModInfo {
  "//"?: string;
  authors?: CDDAModInfoAuthors | null;
  category?: string | null;
  dependencies?: string[] | null;
  description: string;
  id: string;
  maintainers?: string[] | null;
  name: string;
  type: MOD_INFO_Literal;
  version?: string | null;
  [k: string]: unknown;
}
export interface CDDAExternalOption {
  info?: string | null;
  name: string;
  stype: string;
  type: EXTERNAL_OPTION_Literal;
  value: true;
  [k: string]: unknown;
}
export interface CDDAItemBlackList {
  items: true[];
  type: ITEM_BLACKLIST_Literal;
  whitelist: boolean;
  [k: string]: unknown;
}
