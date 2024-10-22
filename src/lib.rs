#![allow(non_snake_case)]

use std::sync::Mutex;

use wasm_bindgen::prelude::wasm_bindgen;

#[repr(C)]
#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Status {
    Ok = 255,
    Halt = 0,
    Panic = 1,
    Fault = 2,
    Host = 3,
    OutOfGas = 4,
}

static PVM: Mutex<Option<Pvm>> = Mutex::new(None);

pub struct Pvm {
    pc: u32,
    program: Vec<u8>,
    gas: i64,
    status: Status,
    registers: Vec<u8>,
}
impl Pvm {
    fn new(program: Vec<u8>, registers: Vec<u8>, gas: i64) -> Self {
        Self {
            pc: 0,
            program,
            gas,
            status: Status::Ok,
            registers,
        }
    }

    fn next_step(&mut self) -> bool {
        self.pc += 1;
        if (self.pc as usize) < self.program.len() {
            true
        } else {
            self.status = Status::Halt;
            false
        }
    }
}

fn with_pvm<F, R>(mut f: F, default: R) -> R
where
    F: FnMut(&mut Pvm) -> R,
{
    let pvm_l = PVM.lock();
    match pvm_l.ok() {
        Some(mut guard) => match guard.as_mut() {
            Some(pvm_l) => f(pvm_l),
            None => default,
        },
        None => default,
    }
}


#[deprecated = "Use setGasLeft / setNextProgramCounter instead."]
#[wasm_bindgen]
pub fn resume(pc: u32, gas: i64) {
    setNextProgramCounter(pc);
    setGasLeft(gas);
}

#[deprecated = "Use resetGeneric instead"]
#[wasm_bindgen]
pub fn reset(program: Vec<u8>, registers: Vec<u8>, gas: i64) {
    resetGeneric(program, registers, gas)
}

#[wasm_bindgen]
pub fn resetGeneric(program: Vec<u8>, registers: Vec<u8>, gas: i64) {
    *PVM.lock().unwrap() = Some(Pvm::new(program, registers, gas));
}

#[wasm_bindgen]
pub fn nextStep() -> bool {
    with_pvm(|pvm| pvm.next_step(), false)
}

#[wasm_bindgen]
pub fn getProgramCounter() -> u32 {
    with_pvm(|pvm| pvm.pc, 0)
}

#[wasm_bindgen]
pub fn setNextProgramCounter(pc: u32) {
    with_pvm(|pvm| pvm.pc = pc, ());
}

#[wasm_bindgen]
pub fn getStatus() -> Status {
    with_pvm(|pvm| pvm.status, Status::Ok)
}

#[wasm_bindgen]
pub fn getExitArg() -> u32 {
    0
}

#[wasm_bindgen]
pub fn getGasLeft() -> i64 {
    with_pvm(|pvm| pvm.gas, 0)
}

#[wasm_bindgen]
pub fn setGasLeft(gas: i64) {
    with_pvm(|pvm| pvm.gas = gas, ());
}

#[wasm_bindgen]
pub fn getRegisters() -> Vec<u8> {
    let default_registers = vec![0; 13 * 4];
    with_pvm(|pvm| pvm.registers.clone(), default_registers)
}

#[wasm_bindgen]
pub fn getPageDump(index: u32) -> Vec<u8> {
    return vec![index as u8];
}
