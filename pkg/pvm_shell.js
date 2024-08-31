
import * as wasm from "./pvm_shell_bg.wasm";
import { __wbg_set_wasm } from "./pvm_shell_bg.js";
__wbg_set_wasm(wasm);
export * from "./pvm_shell_bg.js";
