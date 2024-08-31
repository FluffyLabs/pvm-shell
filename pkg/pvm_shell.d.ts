/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} program
* @param {Uint8Array} registers
* @param {bigint} gas
*/
export function reset(program: Uint8Array, registers: Uint8Array, gas: bigint): void;
/**
* @returns {boolean}
*/
export function nextStep(): boolean;
/**
* @returns {number}
*/
export function getProgramCounter(): number;
/**
* @returns {Status}
*/
export function getStatus(): Status;
/**
* @returns {bigint}
*/
export function getGasLeft(): bigint;
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
  Ok = 0,
  Halt = 1,
  Panic = 2,
  OutOfGas = 3,
}
