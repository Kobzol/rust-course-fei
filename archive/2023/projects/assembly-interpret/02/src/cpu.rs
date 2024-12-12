use crate::memory::{AddrExpr, MemoryCell, ReadExpr, WriteExpr};

pub struct CpuBuilder {
    register_count: usize,
    memory_size: usize,
}

impl CpuBuilder {
    pub fn new() -> Self {
        Self {
            register_count: 16,
            memory_size: 1024,
        }
    }

    pub fn register_count(self, register_count: usize) -> Self {
        Self {
            register_count,
            ..self
        }
    }

    pub fn memory_size(self, memory_size: usize) -> Self {
        Self {
            memory_size,
            ..self
        }
    }

    pub fn zeroed(self) -> Cpu {
        let Self {
            register_count,
            memory_size,
        } = self;
        Cpu {
            registers: vec![MemoryCell::Defined(0); register_count],
            memory: vec![MemoryCell::Defined(0); memory_size],
        }
    }

    pub fn undefined(self) -> Cpu {
        Cpu {
            registers: vec![MemoryCell::Undefined; self.register_count],
            memory: vec![MemoryCell::Undefined; self.memory_size],
        }
    }
}

pub struct Cpu {
    registers: Vec<MemoryCell>,
    memory: Vec<MemoryCell>,
}

impl Cpu {
    pub fn read(&self, expr: ReadExpr) -> Result<u8, ReadExprError> {
        let (slice, address) = match expr {
            ReadExpr::Constant(value) => return Ok(value),
            ReadExpr::Addr(address) => match address {
                AddrExpr::Register(addr) => (self.registers.as_slice(), addr as usize),
                AddrExpr::Memory(addr) => (self.memory.as_slice(), addr as usize),
            },
        };
        match slice.get(address) {
            Some(cell) => match cell {
                MemoryCell::Undefined => Err(ReadExprError::Undefined),
                MemoryCell::Defined(value) => Ok(*value),
            },
            None => Err(ReadExprError::OutOfBounds { index: address }),
        }
    }

    pub fn write(&mut self, expr: WriteExpr, value: u8) -> Result<(), WriteExprError> {
        let (slice, address): (&mut [MemoryCell], usize) = match expr {
            WriteExpr::Addr(address) => match address {
                AddrExpr::Register(addr) => (self.registers.as_mut(), addr as usize),
                AddrExpr::Memory(addr) => (self.memory.as_mut(), addr as usize),
            },
        };
        match slice.get_mut(address) {
            Some(cell) => {
                *cell = MemoryCell::Defined(value);
                Ok(())
            }
            None => Err(WriteExprError::OutOfBounds { index: address }),
        }
    }
}

#[derive(Debug)]
pub enum ReadExprError {
    Undefined,
    OutOfBounds { index: usize },
}

#[derive(Debug)]
pub enum WriteExprError {
    OutOfBounds { index: usize },
}
