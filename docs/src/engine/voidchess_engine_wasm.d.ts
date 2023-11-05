/* tslint:disable */
/* eslint-disable */
/**
*/
export function main_js(): void;
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
/**
* @param {string} game_config
* @param {string} move_str
* @returns {any}
*/
export function evaluate_move_after(game_config: string, move_str: string): any;
/**
* @param {string} game_eval_result_array_str
* @returns {any}
*/
export function pick_move_to_play(game_eval_result_array_str: string): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main_js: () => void;
  readonly get_greeting_for: (a: number, b: number) => number;
  readonly get_concatenated_allowed_moves: (a: number, b: number) => number;
  readonly get_fen: (a: number, b: number) => number;
  readonly evaluate_position_after: (a: number, b: number) => number;
  readonly evaluate_move_after: (a: number, b: number, c: number, d: number) => number;
  readonly pick_move_to_play: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
