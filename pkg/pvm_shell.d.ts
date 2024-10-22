/* tslint:disable */
/* eslint-disable */
/**
* @param {number} pc
* @param {bigint} gas
*/
export function resume(pc: number, gas: bigint): void;
/**
* @param {Uint8Array} program
* @param {Uint8Array} registers
* @param {bigint} gas
*/
export function reset(program: Uint8Array, registers: Uint8Array, gas: bigint): void;
/**
* @param {Uint8Array} program
* @param {Uint8Array} registers
* @param {bigint} gas
*/
export function resetGeneric(program: Uint8Array, registers: Uint8Array, gas: bigint): void;
/**
* @returns {boolean}
*/
export function nextStep(): boolean;
/**
* @returns {number}
*/
export function getProgramCounter(): number;
/**
* @param {number} pc
*/
export function setNextProgramCounter(pc: number): void;
/**
* @returns {Status}
*/
export function getStatus(): Status;
/**
* @returns {number}
*/
export function getExitArg(): number;
/**
* @returns {bigint}
*/
export function getGasLeft(): bigint;
/**
* @param {bigint} gas
*/
export function setGasLeft(gas: bigint): void;
/**
* @returns {Uint8Array}
*/
export function getRegisters(): Uint8Array;
/**
* @param {number} index
* @returns {Uint8Array}
*/
export function getPageDump(index: number): Uint8Array;
/**
*/
export enum Status {
  Ok = 255,
  Halt = 0,
  Panic = 1,
  Fault = 2,
  Host = 3,
  OutOfGas = 4,
}
