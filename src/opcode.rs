use alloc::Vec;

#[derive(Serialize, Deserialize)]
pub enum Opcode {
    Drop,
    Select,

    GetLocal(u32),
    SetLocal(u32),
    TeeLocal(u32),
    GetGlobal(u32),
    SetGlobal(u32),

    CurrentMemory,
    GrowMemory,

    Nop,
    Unreachable,
    Return,
    Call(u32),
    CallIndirect(u32),

    I32Const(i32),

    // iunop
    I32Clz,
    I32Ctz,
    I32Popcnt,

    // ibinop
    I32Add,
    I32Sub,
    I32Mul,
    I32DivU,
    I32DivS,
    I32RemU,
    I32RemS,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrU,
    I32ShrS,
    I32Rotl,
    I32Rotr,

    // itestop
    I32Eqz,

    // irelop
    I32Eq,
    I32Ne,
    I32LtU,
    I32LtS,
    I32LeU,
    I32LeS,
    I32GtU,
    I32GtS,
    I32GeU,
    I32GeS,

    I32WrapI64,

    I32Load(Memarg),
    I32Store(Memarg),
    I32Load8U(Memarg),
    I32Load8S(Memarg),
    I32Load16U(Memarg),
    I32Load16S(Memarg),
    I32Store8(Memarg),
    I32Store16(Memarg),

    I64Const(i64),

    // iunop
    I64Clz,
    I64Ctz,
    I64Popcnt,

    // ibinop
    I64Add,
    I64Sub,
    I64Mul,
    I64DivU,
    I64DivS,
    I64RemU,
    I64RemS,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrU,
    I64ShrS,
    I64Rotl,
    I64Rotr,

    // itestop
    I64Eqz,

    // irelop
    I64Eq,
    I64Ne,
    I64LtU,
    I64LtS,
    I64LeU,
    I64LeS,
    I64GtU,
    I64GtS,
    I64GeU,
    I64GeS,

    I64ExtendI32U,
    I64ExtendI32S,

    I64Load(Memarg),
    I64Store(Memarg),
    I64Load8U(Memarg),
    I64Load8S(Memarg),
    I64Load16U(Memarg),
    I64Load16S(Memarg),
    I64Load32U(Memarg),
    I64Load32S(Memarg),
    I64Store8(Memarg),
    I64Store16(Memarg),
    I64Store32(Memarg),

    // wasm-core specific opcodes here. Should be generated by front-end.
    Jmp(u32),
    JmpIf(u32),
    JmpTable(Vec<u32>, u32)
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Memarg {
    pub offset: u32,
    pub align: u32
}
