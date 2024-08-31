/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function reset(a: number, b: number, c: number, d: number, e: number): void;
export function nextStep(): number;
export function getProgramCounter(): number;
export function getStatus(): number;
export function getGasLeft(): number;
export function getRegisters(a: number): void;
export function getPageDump(a: number, b: number): void;
export function __wbindgen_malloc(a: number, b: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number, c: number): void;
