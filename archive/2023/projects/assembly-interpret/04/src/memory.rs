#[derive(Clone)]
pub enum MemoryCell<T> {
    Undefined,
    Defined(T),
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum AddressableExpr {
    Register(u8),
    Memory(u32),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum WritableExpr {
    Addressable(AddressableExpr),
}

impl WritableExpr {
    pub fn register(index: u8) -> Self {
        Self::Addressable(AddressableExpr::Register(index))
    }

    pub fn memory(address: u32) -> Self {
        Self::Addressable(AddressableExpr::Memory(address))
    }

    pub fn as_read<T>(&self) -> ReadableExpr<T> {
        match self {
            WritableExpr::Addressable(addr) => ReadableExpr::Addressable(addr.clone()),
        }
    }
}

impl<T> Into<ReadableExpr<T>> for WritableExpr {
    fn into(self) -> ReadableExpr<T> {
        match self {
            WritableExpr::Addressable(addr) => ReadableExpr::Addressable(addr),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReadableExpr<T> {
    Addressable(AddressableExpr),
    Constant(T),
}

impl<T> ReadableExpr<T> {
    pub fn constant(value: T) -> Self {
        Self::Constant(value)
    }

    pub fn register(index: u8) -> Self {
        Self::Addressable(AddressableExpr::Register(index))
    }

    pub fn memory(address: u32) -> Self {
        Self::Addressable(AddressableExpr::Memory(address))
    }
}
