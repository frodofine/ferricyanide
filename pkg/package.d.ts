/* tslint:disable */
/* eslint-disable */
/**
* # Errors
*
* This function cannot fail
*/
export function main(): void;
/**
*/
export class FerricyanideDisplay {
  free(): void;
/**
* Create a new web client
* @param {string} app_div_id
* @param {number} width
* @param {number} height
*/
  constructor(app_div_id: string, width: number, height: number);
/**
* Start our WebGL and initialize the molecules to the value in contents
* @param {Uint8Array} contents
* @param {string} format
*/
  add_molecule(contents: Uint8Array, format: string): void;
/**
* Update our simulation
* @param {number} dt
*/
  update_time(dt: number): void;
/**
* Render the scene. `index.html` will call this once every requestAnimationFrame
*/
  render(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_ferricyanidedisplay_free: (a: number) => void;
  readonly ferricyanidedisplay_new: (a: number, b: number, c: number, d: number) => number;
  readonly ferricyanidedisplay_add_molecule: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly ferricyanidedisplay_update_time: (a: number, b: number) => void;
  readonly ferricyanidedisplay_render: (a: number) => void;
  readonly main: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h01b361bc1b291831: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__heae1c0c5c9480408: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h530e4f0db8cc75dc: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h98c1b0a468d8e21f: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
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
        