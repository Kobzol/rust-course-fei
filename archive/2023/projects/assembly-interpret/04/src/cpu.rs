use crate::memory::{AddressableExpr, MemoryCell, ReadableExpr, WritableExpr};

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

    pub fn default<T: Default + Clone>(self) -> Cpu<T> {
        let Self {
            register_count,
            memory_size,
        } = self;
        Cpu {
            registers: vec![MemoryCell::Defined(T::default()); register_count],
            memory: vec![MemoryCell::Defined(T::default()); memory_size],
            instruction_pointer: 0,
        }
    }

    pub fn undefined<T: Clone>(self) -> Cpu<T> {
        let Self {
            register_count,
            memory_size,
        } = self;
        Cpu {
            registers: vec![MemoryCell::Undefined; register_count],
            memory: vec![MemoryCell::Undefined; memory_size],
            instruction_pointer: 0,
        }
    }
}

pub struct Cpu<T> {
    registers: Vec<MemoryCell<T>>,
    memory: Vec<MemoryCell<T>>,
    instruction_pointer: u64,
}

impl<T> Cpu<T> {
    pub fn read(&self, expr: ReadableExpr<T>) -> Result<T, ReadError>
    where
        T: Clone,
    {
        match expr {
            ReadableExpr::Addressable(addr) => {
                let Some(cell) = self.get_cell(addr) else {
                    return Err(ReadError::OutOfBounds(addr));
                };

                match cell {
                    MemoryCell::Defined(value) => Ok(value.clone()),
                    MemoryCell::Undefined => Err(ReadError::Undefined(addr)),
                }
            }
            ReadableExpr::Constant(value) => Ok(value),
        }
    }
    pub fn write(&mut self, expr: WritableExpr, value: T) -> Result<(), WriteError> {
        match expr {
            WritableExpr::Addressable(address) => {
                let Some(cell) = self.get_cell_mut(address) else {
                    return Err(WriteError::OutOfBounds(address));
                };
                *cell = MemoryCell::Defined(value);
            }
        }
        Ok(())
    }

    pub fn get_ip(&self) -> u64 {
        self.instruction_pointer
    }
    pub fn set_ip(&mut self, ip: u64) {
        self.instruction_pointer = ip;
    }

    fn get_cell(&self, expr: AddressableExpr) -> Option<&MemoryCell<T>> {
        let (memory, address) = match expr {
            AddressableExpr::Register(reg) => (self.registers.as_slice(), reg as u32),
            AddressableExpr::Memory(address) => (self.memory.as_slice(), address),
        };
        memory.get(address as usize)
    }

    fn get_cell_mut(&mut self, expr: AddressableExpr) -> Option<&mut MemoryCell<T>> {
        let (memory, address) = match expr {
            AddressableExpr::Register(reg) => (self.registers.as_mut_slice(), reg as u32),
            AddressableExpr::Memory(address) => (self.memory.as_mut_slice(), address),
        };
        memory.get_mut(address as usize)
    }
}

#[derive(Debug)]
pub enum ReadError {
    Undefined(AddressableExpr),
    OutOfBounds(AddressableExpr),
}

#[derive(Debug)]
pub enum WriteError {
    OutOfBounds(AddressableExpr),
}

#[cfg(test)]
mod tests {
    use crate::{Cpu, ReadableExpr};

    #[test]
    fn test_cpu_init_zero() {
        let cpu = Cpu::zeroed(16, 1024);
        assert_eq!(cpu.read_value(ReadableExpr::memory(100)).unwrap(), 0);
        assert_eq!(cpu.read_value(ReadableExpr::register(0)).unwrap(), 0);
    }
}
