use crate::addr::IO_BASE;
use volatile::{Volatile};

/// The base address for the `MU` registers.
const V3D_BASE: usize = IO_BASE + 0xC00000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    pub regs: [Volatile<u32>; 969]
}

pub struct V3d {
    registers: &'static mut Registers,
}

impl V3d {
    /// Returns a new instance of `Mailbox`.
    #[inline]
    pub fn new() -> V3d {
        V3d {
            registers: unsafe { &mut *(V3D_BASE as *mut Registers) },
        }
    }

    pub fn read(&self, pos: usize) -> u32 {
       	self.registers.regs[pos].read()
    }

    pub fn write(&mut self, pos: usize, data: u32) {
        self.registers.regs[pos]
            .write(data);
    }
}
