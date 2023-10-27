#[derive(Copy, Clone)]
enum MemoryCell {
    Undefined,
    Defined(u8),
}

#[derive(Debug)]
enum ReadExprError {
    Undefined,
    OutOfBounds { index: usize },
}

#[derive(Debug)]
enum WriteExprError {
    OutOfBounds { index: usize },
}

struct CpuBuilder {
    register_count: usize,
    memory_size: usize,
    param3: usize,
}

impl CpuBuilder {
    fn new() -> Self {
        Self {
            register_count: 16,
            memory_size: 1024,
            param3: 0,
        }
    }

    fn register_count(self, register_count: usize) -> Self {
        Self {
            register_count,
            ..self
        }
    }

    fn memory_size(self, memory_size: usize) -> Self {
        Self {
            memory_size,
            ..self
        }
    }

    fn zeroed(self) -> Cpu {
        let Self {
            register_count,
            memory_size,
            param3,
        } = self;
        Cpu {
            registers: vec![MemoryCell::Defined(param3); register_count],
            memory: vec![MemoryCell::Defined(param3); memory_size],
        }
    }

    fn undefined(self) -> Cpu {
        Cpu {
            registers: vec![MemoryCell::Undefined; self.register_count],
            memory: vec![MemoryCell::Undefined; self.memory_size],
        }
    }
}

struct Cpu {
    registers: Vec<MemoryCell>,
    memory: Vec<MemoryCell>,
}

impl Cpu {
    fn read(&self, expr: ReadExpr) -> Result<u8, ReadExprError> {
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

    fn write(&mut self, expr: WriteExpr, value: u8) -> Result<(), WriteExprError> {
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

enum AddrExpr {
    Register(u8),
    Memory(u32),
}

enum ReadExpr {
    Constant(u8),
    Addr(AddrExpr),
}

impl ReadExpr {
    fn constant(value: u8) -> Self {
        Self::Constant(value)
    }
    fn memory(address: u32) -> Self {
        Self::Addr(AddrExpr::Memory(address))
    }
    fn register(index: u8) -> Self {
        Self::Addr(AddrExpr::Register(index))
    }
}

enum WriteExpr {
    Addr(AddrExpr),
}

impl WriteExpr {
    fn memory(address: u32) -> Self {
        Self::Addr(AddrExpr::Memory(address))
    }
    fn register(index: u8) -> Self {
        Self::Addr(AddrExpr::Register(index))
    }
}

enum Instruction {
    Set { dest: WriteExpr, src: ReadExpr },
    Print(ReadExpr),
}

#[derive(Debug)]
enum ExecuteError {
    ReadExpr(ReadExprError),
    WriteExpr(WriteExprError),
}

impl From<ReadExprError> for ExecuteError {
    fn from(value: ReadExprError) -> Self {
        Self::ReadExpr(value)
    }
}

impl From<WriteExprError> for ExecuteError {
    fn from(value: WriteExprError) -> Self {
        Self::WriteExpr(value)
    }
}

fn execute_instruction(cpu: &mut Cpu, inst: Instruction) -> Result<(), ExecuteError> {
    match inst {
        Instruction::Print(expr) => {
            let value = cpu.read(expr)?;
            println!("{value}");
        }
        Instruction::Set { dest, src } => {
            let value = cpu.read(src)?;
            cpu.write(dest, value)?;
        }
    };
    Ok(())
}

enum ParseError {}

fn parse(input: &str) -> Result<Vec<Instruction>, ParseError> {
    todo!();
}

fn main() {
    let mut cpu = CpuBuilder::new()
        .register_count(16)
        .memory_size(128)
        .zeroed();

    let inst = Instruction::Set {
        dest: WriteExpr::memory(5),
        src: ReadExpr::constant(50),
    };
    execute_instruction(&mut cpu, inst).unwrap();
    execute_instruction(&mut cpu, Instruction::Print(ReadExpr::memory(5))).unwrap();
}
