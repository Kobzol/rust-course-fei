#[derive(Copy, Clone)]
pub enum MemoryCell {
    Undefined,
    Defined(u8),
}

#[derive(Debug)]
pub enum AddrExpr {
    Register(u8),
    Memory(u32),
}

#[derive(Debug)]
pub enum ReadExpr {
    Constant(u8),
    Addr(AddrExpr),
}

impl ReadExpr {
    pub fn constant(value: u8) -> Self {
        Self::Constant(value)
    }
    pub fn memory(address: u32) -> Self {
        Self::Addr(AddrExpr::Memory(address))
    }
    pub fn register(index: u8) -> Self {
        Self::Addr(AddrExpr::Register(index))
    }
}

#[derive(Debug)]
pub enum WriteExpr {
    Addr(AddrExpr),
}

impl WriteExpr {
    pub fn memory(address: u32) -> Self {
        Self::Addr(AddrExpr::Memory(address))
    }
    pub fn register(index: u8) -> Self {
        Self::Addr(AddrExpr::Register(index))
    }
}
