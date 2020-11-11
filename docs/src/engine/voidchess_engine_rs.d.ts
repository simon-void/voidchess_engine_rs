/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
* @returns {any}
*/
export function get_greeting_for(name: string): any;
/**
* @param {string} game_config
* @returns {any}
*/
export function get_concatenated_allowed_moves(game_config: string): any;
/**
* @param {string} game_config
* @returns {any}
*/
export function get_fen(game_config: string): any;
/**
* @param {string} game_config
* @returns {any}
*/
export function evaluate_position_after(game_config: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_greeting_for: (a: number, b: number) => number;
  readonly get_concatenated_allowed_moves: (a: number, b: number) => number;
  readonly get_fen: (a: number, b: number) => number;
  readonly evaluate_position_after: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        