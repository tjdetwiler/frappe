#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq)]
pub enum Bytecode {
    aaload,
    aastore,
    aconst_null,
    aload {
        index: u8,
    },
    aload_n(u8),
    anewarray {
        index: u16,
    },
    areturn,
    arraylength,
    astore {
        index: u8,
    },
    astore_n(u8),
    athrow,
    baload,
    bastore,
    bipush {
        byte: u8,
    },
    caload,
    castore,
    checkcast {
        index: u16,
    },
    d2f,
    d2i,
    d2l,
    dadd,
    daload,
    dastore,
    dcmpg,
    dcmpl,
    dconst_d(u8),
    ddiv,
    dload {
        byte: u8,
    },
    dload_n(u8),
    dmul,
    dneg,
    drem,
    dreturn,
    dstore {
        index: u8,
    },
    dstore_n(u8),
    dsub,
    dup,
    dup_x1,
    dup_x2,
    dup2,
    dup2_x1,
    dup2_x2,
    f2d,
    f2i,
    f2l,
    fadd,
    faload,
    fastore,
    fcmpg,
    fcmpl,
    fconst_f(u8),
    fdiv,
    fload {
        index: u8,
    },
    fload_n(u8),
    fmul,
    fneg,
    frem,
    freturn,
    fstore {
        index: u8,
    },
    fstore_n(u8),
    fsub,
    getfield {
        index: u16,
    },
    getstatic {
        index: u16,
    },
    goto {
        branchoffset: u16,
    },
    goto_w {
        branchoffset: u32,
    },
    i2b,
    i2c,
    i2d,
    i2f,
    i2l,
    i2s,
    iadd,
    iaload,
    iand,
    iastore,
    /// -1 <= i <= 5
    iconst_i(i8),
    idiv,
    if_acmpeq {
        branchoffset: u16,
    },
    if_acmpne {
        branchoffset: u16,
    },
    if_icmpeq {
        branchoffset: u16,
    },
    if_icmpne {
        branchoffset: u16,
    },
    if_icmplt {
        branchoffset: u16,
    },
    if_icmpge {
        branchoffset: u16,
    },
    if_icmpgt {
        branchoffset: u16,
    },
    if_icmple {
        branchoffset: u16,
    },
    ifeq {
        branchoffset: u16,
    },
    ifne {
        branchoffset: u16,
    },
    iflt {
        branchoffset: u16,
    },
    ifge {
        branchoffset: u16,
    },
    ifgt {
        branchoffset: u16,
    },
    ifle {
        branchoffset: u16,
    },
    ifnonnull {
        branchoffset: u16,
    },
    ifnull {
        branchoffset: u16,
    },
    iinc {
        index: u8,
        constant: u8,
    },
    iload {
        index: u8,
    },
    /// 0 <= n <= 3
    iload_n(u8),
    imul,
    ineg,
    instanceof {
        index: u16,
    },
    invokedynamic {
        index: u16,
    },
    invokeinterface {
        index: u16,
        count: u8,
    },
    invokespecial {
        index: u16,
    },
    invokestatic {
        index: u16,
    },
    invokevirtual {
        index: u16,
    },
    ior,
    irem,
    ireturn,
    ishl,
    ishr,
    istore {
        index: u8
    },
    /// 0 <= n <= 3
    istore_n(u8),
    isub,
    iushr,
    ixor,
    jsr {
        branchoffset: u16,
    },
    jsr_w {
        branchoffset: u32,
    },
    l2d,
    l2f,
    l2i,
    ladd,
    laload,
    land,
    lastore,
    lcmp,
    /// 0 <= l <= 1
    lconst_l(u8),
    ldc {
        index: u8,
    },
    ldc_w {
        index: u16,
    },
    ldc2_w {
        index: u16,
    },
    ldiv,
    lload {
        index: u8,
    },
    /// 0 <= n <= 3
    lload_n(u8),
    lmul,
    lneg,
    lookupswitch {
        default: i32,
        npairs: i32,
        pairs: Vec<(i32, i32)>,
    },
    lor,
    lrem,
    lreturn,
    lshl,
    lshr,
    lstore {
        index: u8
    },
    /// 0 <= n <= 3
    lstore_n(u8),
    lsub,
    lushr,
    lxor,
    monitorenter,
    monitorexit,
    multianewarray {
        index: u16,
        dimensions: u8,
    },
    new {
        index: u16,
    },
    newarray {
        atype: u8,
    },
    nop,
    pop,
    pop2,
    putfield {
        index: u16,
    },
    putstatic {
        index: u16,
    },
    ret {
        index: u8,
    },
    Return,
    saload,
    sastore,
    sipush {
        short: i16,
    },
    swap,
    tableswitch {
        default: i32,
        low: i32,
        high: i32,
        offsets: Vec<i32>,
    },

    wide_iload {
        index: u16,
    },
    wide_fload {
        index: u16,
    },
    wide_aload {
        index: u16,
    },
    wide_lload {
        index: u16,
    },
    wide_dload {
        index: u16,
    },
    wide_istore {
        index: u16,
    },
    wide_fstore {
        index: u16,
    },
    wide_astore {
        index: u16,
    },
    wide_lstore {
        index: u16,
    },
    wide_dstore {
        index: u16,
    },
    wide_ret {
        index: u16,
    },
    wide_iinc {
        index: u16,
        constant: u16,
    },

    invalid(u8),
}

macro_rules! fetch {
    (u32 $code:expr, $pc:expr) => {{
        let byte1: u32 = $code[$pc] as u32;
        let byte2: u32 = $code[$pc + 1] as u32;
        let byte3: u32 = $code[$pc + 2] as u32;
        let byte4: u32 = $code[$pc + 3] as u32;
        $pc = $pc + 4;
        (byte1 << 24 | byte2 << 16 | byte3 << 8 | byte4)
    }};
    (i32 $code:expr, $pc:expr) => {{
        fetch!(u32 $code, $pc) as i32
    }};
    (u16 $code:expr, $pc:expr) => {{
        let byte1: u16 = $code[$pc] as u16;
        let byte2: u16 = $code[$pc + 1] as u16;
        $pc = $pc + 2;
        (byte1 << 8 | byte2)
    }};
    (i16 $code:expr, $pc:expr) => {{
        fetch!(u16 $code, $pc) as i16
    }};
    (u8 $code:expr, $pc:expr) => {{
        let byte: u8 = $code[$pc] as u8;
        $pc = $pc + 1;
        byte
    }};
}

/// Computes the number of pad bytes to align a given index to an alignment.
#[macro_export]
macro_rules! pad_align {
    ($value:expr, 4) => {
        (4 - ($value & 3)) & 3
    };
    ($value:expr, $alignment:expr) => {
        ($alignment - ($value % $alignment)) % $alignment
    };
}

macro_rules! bytecode {
    ($name:ident, $pc:expr) => {
        DecodeResult {
            bytecode: Bytecode::$name,
            newpc: $pc,
        }
    };
    ($name:ident, $val:expr, $pc:expr) => {
        DecodeResult {
            bytecode: Bytecode::$name($val),
            newpc: $pc,
        }
    };
    ($code:expr, $pc:expr, $name:ident, $field:ident : u8) => {
        DecodeResult {
            bytecode: Bytecode::$name { $field: $code[$pc] },
            newpc: $pc + 1,
        }
    };
    ($code:expr, $pc:expr, $name:ident, $field:ident : i16) => {{
        let value = fetch!(i16 $code, $pc);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            newpc: $pc,
        }
    }};
    ($code:expr, $pc:expr, $name:ident, $field:ident : u16) => {{
        let value = fetch!(u16 $code, $pc);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            newpc: $pc,
        }
    }};
    ($code:expr, $pc:expr, $name:ident, $field:ident : i16) => {{
        let value = fetch!(i16 $code, $pc);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            newpc: $pc,
        }
    }};
    ($code:expr, $pc:expr, $name:ident, $field:ident : u32) => {{
        let value = fetch!(u32 $code, $pc);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            newpc: $pc,
        }
    }};
}

#[derive(Debug)]
pub struct DecodeResult {
    pub bytecode: Bytecode,
    pub newpc: usize,
}

impl Bytecode {
    /// Decodes a single instruction from the code slice.
    ///
    /// Returns the decoded bytecode and how many bytes from the instruction
    /// stream were consumed by this decoded instruction.
    ///
    /// # Examples
    /// ```rust
    /// let mut pc = 0;
    /// let code: Vec<u8> = vec![];
    /// while pc < code.len() {
    /// }
    /// ```
    pub fn decode(code: &[u8], mut pc: usize) -> DecodeResult {
        let opcode = code[pc];
        pc = pc + 1;
        match opcode {
            0x00 => bytecode!(nop, pc),
            0x01 => bytecode!(aconst_null, pc),
            i @ 0x02...0x08 => bytecode!(iconst_i, i as i8 - 0x03, pc),
            l @ 0x09...0x0a => bytecode!(lconst_l, l as u8 - 0x09, pc),
            f @ 0x0b...0x0d => bytecode!(fconst_f, f as u8 - 0x0b, pc),
            d @ 0x0e...0x0f => bytecode!(dconst_d, d as u8 - 0x0e, pc),
            0x10 => bytecode!(code, pc, bipush, byte: u8),
            0x11 => bytecode!(code, pc, sipush, short: i16),
            0x12 => bytecode!(code, pc, ldc, index: u8),
            0x13 => bytecode!(code, pc, ldc_w, index: u16),
            0x14 => bytecode!(code, pc, ldc2_w, index: u16),
            0x15 => bytecode!(code, pc, iload, index: u8),
            0x16 => bytecode!(code, pc, lload, index: u8),
            0x17 => bytecode!(code, pc, fload, index: u8),
            0x18 => bytecode!(code, pc, dload, byte: u8),
            0x19 => bytecode!(code, pc, aload, index: u8),
            n @ 0x1a...0x1d => bytecode!(iload_n, n as u8 - 0x1a, pc),
            n @ 0x1e...0x21 => bytecode!(lload_n, n as u8 - 0x1e, pc),
            n @ 0x22...0x25 => bytecode!(fload_n, n as u8 - 0x22, pc),
            n @ 0x26...0x29 => bytecode!(dload_n, n as u8 - 0x26, pc),
            n @ 0x2a...0x2d => bytecode!(aload_n, n as u8 - 0x2a, pc),
            0x2e => bytecode!(iaload, pc),
            0x2f => bytecode!(laload, pc),
            0x30 => bytecode!(faload, pc),
            0x31 => bytecode!(daload, pc),
            0x32 => bytecode!(aaload, pc),
            0x33 => bytecode!(baload, pc),
            0x34 => bytecode!(caload, pc),
            0x35 => bytecode!(saload, pc),
            0x36 => bytecode!(code, pc, istore, index: u8),
            0x37 => bytecode!(code, pc, lstore, index: u8),
            0x38 => bytecode!(code, pc, fstore, index: u8),
            0x39 => bytecode!(code, pc, dstore, index: u8),
            0x3a => bytecode!(code, pc, astore, index: u8),
            n @ 0x3b...0x3e => bytecode!(istore_n, n as u8 - 0x3b, pc),
            n @ 0x3f...0x42 => bytecode!(lstore_n, n as u8 - 0x3f, pc),
            n @ 0x43...0x46 => bytecode!(fstore_n, n as u8 - 0x43, pc),
            n @ 0x47...0x4a => bytecode!(dstore_n, n as u8 - 0x47, pc),
            n @ 0x4b...0x4e => bytecode!(astore_n, n as u8 - 0x4b, pc),
            0x4f => bytecode!(iastore, pc),
            0x50 => bytecode!(lastore, pc),
            0x51 => bytecode!(fastore, pc),
            0x52 => bytecode!(dastore, pc),
            0x53 => bytecode!(aastore, pc),
            0x54 => bytecode!(bastore, pc),
            0x55 => bytecode!(castore, pc),
            0x56 => bytecode!(sastore, pc),
            0x57 => bytecode!(pop, pc),
            0x58 => bytecode!(pop2, pc),
            0x59 => bytecode!(dup, pc),
            0x5a => bytecode!(dup_x1, pc),
            0x5b => bytecode!(dup_x2, pc),
            0x5c => bytecode!(dup2, pc),
            0x5d => bytecode!(dup2_x1, pc),
            0x5e => bytecode!(dup2_x2, pc),
            0x5f => bytecode!(swap, pc),
            0x60 => bytecode!(iadd, pc),
            0x61 => bytecode!(ladd, pc),
            0x62 => bytecode!(fadd, pc),
            0x63 => bytecode!(dadd, pc),
            0x64 => bytecode!(isub, pc),
            0x65 => bytecode!(lsub, pc),
            0x66 => bytecode!(fsub, pc),
            0x67 => bytecode!(dsub, pc),
            0x68 => bytecode!(imul, pc),
            0x69 => bytecode!(lmul, pc),
            0x6a => bytecode!(fmul, pc),
            0x6b => bytecode!(dmul, pc),
            0x6c => bytecode!(idiv, pc),
            0x6d => bytecode!(ldiv, pc),
            0x6e => bytecode!(fdiv, pc),
            0x6f => bytecode!(ddiv, pc),
            0x70 => bytecode!(irem, pc),
            0x71 => bytecode!(lrem, pc),
            0x72 => bytecode!(frem, pc),
            0x73 => bytecode!(drem, pc),
            0x74 => bytecode!(ineg, pc),
            0x75 => bytecode!(lneg, pc),
            0x76 => bytecode!(fneg, pc),
            0x77 => bytecode!(dneg, pc),
            0x78 => bytecode!(ishl, pc),
            0x79 => bytecode!(lshl, pc),
            0x7a => bytecode!(ishr, pc),
            0x7b => bytecode!(lshr, pc),
            0x7c => bytecode!(iushr, pc),
            0x7d => bytecode!(lushr, pc),
            0x7e => bytecode!(iand, pc),
            0x7f => bytecode!(land, pc),
            0x80 => bytecode!(ior, pc),
            0x81 => bytecode!(lor, pc),
            0x82 => bytecode!(ixor, pc),
            0x83 => bytecode!(lxor, pc),
            0x84 => {
                let index = fetch!(u8 code, pc);
                let constant = fetch!(u8 code, pc);
                DecodeResult {
                    bytecode: Bytecode::iinc {
                        index: index,
                        constant: constant,
                    },
                    newpc: pc,
                }
            }
            0x85 => bytecode!(i2l, pc),
            0x86 => bytecode!(i2f, pc),
            0x87 => bytecode!(i2d, pc),
            0x88 => bytecode!(l2i, pc),
            0x89 => bytecode!(l2f, pc),
            0x8a => bytecode!(l2d, pc),
            0x8b => bytecode!(f2i, pc),
            0x8c => bytecode!(f2l, pc),
            0x8d => bytecode!(f2d, pc),
            0x8e => bytecode!(d2i, pc),
            0x8f => bytecode!(d2l, pc),
            0x90 => bytecode!(d2f, pc),
            0x91 => bytecode!(i2b, pc),
            0x92 => bytecode!(i2c, pc),
            0x93 => bytecode!(i2s, pc),
            0x94 => bytecode!(lcmp, pc),
            0x95 => bytecode!(fcmpl, pc),
            0x96 => bytecode!(fcmpg, pc),
            0x97 => bytecode!(dcmpl, pc),
            0x98 => bytecode!(dcmpg, pc),
            0x99 => bytecode!(code, pc, ifeq, branchoffset: u16),
            0x9a => bytecode!(code, pc, ifne, branchoffset: u16),
            0x9b => bytecode!(code, pc, iflt, branchoffset: u16),
            0x9c => bytecode!(code, pc, ifge, branchoffset: u16),
            0x9d => bytecode!(code, pc, ifgt, branchoffset: u16),
            0x9e => bytecode!(code, pc, ifle, branchoffset: u16),
            0x9f => bytecode!(code, pc, if_icmpeq, branchoffset: u16),
            0xa0 => bytecode!(code, pc, if_icmpne, branchoffset: u16),
            0xa1 => bytecode!(code, pc, if_icmplt, branchoffset: u16),
            0xa2 => bytecode!(code, pc, if_icmpge, branchoffset: u16),
            0xa3 => bytecode!(code, pc, if_icmpgt, branchoffset: u16),
            0xa4 => bytecode!(code, pc, if_icmple, branchoffset: u16),
            0xa5 => bytecode!(code, pc, if_acmpeq, branchoffset: u16),
            0xa6 => bytecode!(code, pc, if_acmpne, branchoffset: u16),
            0xa7 => bytecode!(code, pc, goto, branchoffset: u16),
            0xa8 => bytecode!(code, pc, jsr, branchoffset: u16),
            0xa9 => bytecode!(code, pc, ret, index: u8),
            0xaa => {
                pc += pad_align!(pc, 4);
                let default = fetch!(i32 code, pc);
                let low = fetch!(i32 code, pc);
                let high = fetch!(i32 code, pc);
                let mut offsets: Vec<i32> = vec![];
                let offset_count = high - low + 1;
                for _ in 0..offset_count {
                    let offset = fetch!(i32 code, pc);
                    offsets.push(offset);
                }
                DecodeResult {
                    bytecode: Bytecode::tableswitch {
                        default: default,
                        low: low,
                        high: high,
                        offsets: offsets,
                    },
                    newpc: pc,
                }
            }
            0xab => {
                pc += pad_align!(pc, 4);
                let default = fetch!(i32 code, pc);
                let npairs = fetch!(i32 code, pc);
                let mut pairs: Vec<(i32, i32)> = vec![];
                for _ in 0..npairs {
                    let first = fetch!(i32 code, pc);
                    let second = fetch!(i32 code, pc);
                    pairs.push((first, second));
                }
                DecodeResult {
                    bytecode: Bytecode::lookupswitch {
                        default: default,
                        npairs: npairs,
                        pairs: pairs,
                    },
                    newpc: pc,
                }
            }
            0xac => bytecode!(ireturn, pc),
            0xad => bytecode!(lreturn, pc),
            0xae => bytecode!(freturn, pc),
            0xaf => bytecode!(dreturn, pc),
            0xb0 => bytecode!(areturn, pc),
            0xb1 => bytecode!(Return, pc),
            0xb2 => bytecode!(code, pc, getstatic, index: u16),
            0xb3 => bytecode!(code, pc, putstatic, index: u16),
            0xb4 => bytecode!(code, pc, getfield, index: u16),
            0xb5 => bytecode!(code, pc, putfield, index: u16),
            0xb6 => bytecode!(code, pc, invokevirtual, index: u16),
            0xb7 => bytecode!(code, pc, invokespecial, index: u16),
            0xb8 => bytecode!(code, pc, invokestatic, index: u16),
            0xb9 => {
                let index = fetch!(u16 code, pc);
                let count = fetch!(u8 code, pc);
                let _ignore = fetch!(u8 code, pc);
                DecodeResult {
                    bytecode: Bytecode::invokeinterface {
                        index: index,
                        count: count,
                    },
                    newpc: pc,
                }
            }
            0xba => {
                let index = fetch!(u16 code, pc);
                let _ignore = fetch!(u8 code, pc);
                let _ignore = fetch!(u8 code, pc);
                DecodeResult {
                    bytecode: Bytecode::invokedynamic { index: index },
                    newpc: pc,
                }
            }
            0xbb => bytecode!(code, pc, new, index: u16),
            0xbc => bytecode!(code, pc, newarray, atype: u8),
            0xbd => bytecode!(code, pc, anewarray, index: u16),
            0xbe => bytecode!(arraylength, pc),
            0xbf => bytecode!(athrow, pc),
            0xc0 => bytecode!(code, pc, checkcast, index: u16),
            0xc1 => bytecode!(code, pc, instanceof, index: u16),
            0xc2 => bytecode!(monitorenter, pc),
            0xc3 => bytecode!(monitorexit, pc),
            0xc4 => {
                let opcode = fetch!(u8 code, pc);
                match opcode {
                    0x15 => bytecode!(code, pc, wide_iload, index: u16),
                    0x16 => bytecode!(code, pc, wide_lload, index: u16),
                    0x17 => bytecode!(code, pc, wide_fload, index: u16),
                    0x18 => bytecode!(code, pc, wide_dload, index: u16),
                    0x19 => bytecode!(code, pc, wide_aload, index: u16),
                    0x36 => bytecode!(code, pc, wide_istore, index: u16),
                    0x37 => bytecode!(code, pc, wide_lstore, index: u16),
                    0x38 => bytecode!(code, pc, wide_fstore, index: u16),
                    0x39 => bytecode!(code, pc, wide_dstore, index: u16),
                    0x3a => bytecode!(code, pc, wide_astore, index: u16),
                    0x84 => {
                        let index = fetch!(u16 code, pc);
                        let constant = fetch!(u16 code, pc);
                        DecodeResult {
                            bytecode: Bytecode::wide_iinc {
                                index: index,
                                constant: constant,
                            },
                            newpc: pc,
                        }
                    }
                    0xa9 => bytecode!(code, pc, wide_ret, index: u16),
                    op @ _ => bytecode!(invalid, op, pc),
                }
            },
            0xc5 => {
                let index = fetch!(u16 code, pc);
                let dimensions = fetch!(u8 code, pc);
                DecodeResult {
                    bytecode: Bytecode::multianewarray {
                        index: index,
                        dimensions: dimensions,
                    },
                    newpc: pc,
                }
            },
            0xc6 => bytecode!(code, pc, ifnull, branchoffset: u16),
            0xc7 => bytecode!(code, pc, ifnonnull, branchoffset: u16),
            0xc8 => bytecode!(code, pc, goto_w, branchoffset: u32),
            0xc9 => bytecode!(code, pc, jsr_w, branchoffset: u32),
            op @ _ => bytecode!(invalid, op, pc),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bytecode;

    macro_rules! test_parameterized_bytecode {
        ($opcode:expr => $bytecode:expr) => {{
            let result = Bytecode::decode(&[$opcode], 0);
            assert_eq!(1, result.newpc);
            assert_eq!($bytecode, result.bytecode);
        }}
    }

    #[test]
    fn test_decode_iconst_i() {
        test_parameterized_bytecode!(0x02 => Bytecode::iconst_i(-1));
        test_parameterized_bytecode!(0x03 => Bytecode::iconst_i(0));
        test_parameterized_bytecode!(0x04 => Bytecode::iconst_i(1));
        test_parameterized_bytecode!(0x05 => Bytecode::iconst_i(2));
        test_parameterized_bytecode!(0x06 => Bytecode::iconst_i(3));
        test_parameterized_bytecode!(0x07 => Bytecode::iconst_i(4));
        test_parameterized_bytecode!(0x08 => Bytecode::iconst_i(5));
    }

    #[test]
    fn test_decode_lconst_l() {
        test_parameterized_bytecode!(0x09 => Bytecode::lconst_l(0));
        test_parameterized_bytecode!(0x0a => Bytecode::lconst_l(1));
    }

    #[test]
    fn test_decode_fconst_f() {
        test_parameterized_bytecode!(0x0b => Bytecode::fconst_f(0));
        test_parameterized_bytecode!(0x0c => Bytecode::fconst_f(1));
        test_parameterized_bytecode!(0x0d => Bytecode::fconst_f(2));
    }

    #[test]
    fn test_decode_dconst_d() {
        test_parameterized_bytecode!(0x0e => Bytecode::dconst_d(0));
        test_parameterized_bytecode!(0x0f => Bytecode::dconst_d(1));
    }

    #[test]
    fn test_decode_iload_n() {
        test_parameterized_bytecode!(0x1a => Bytecode::iload_n(0));
        test_parameterized_bytecode!(0x1b => Bytecode::iload_n(1));
        test_parameterized_bytecode!(0x1c => Bytecode::iload_n(2));
        test_parameterized_bytecode!(0x1d => Bytecode::iload_n(3));
    }

    #[test]
    fn test_decode_lload_n() {
        test_parameterized_bytecode!(0x1e => Bytecode::lload_n(0));
        test_parameterized_bytecode!(0x1f => Bytecode::lload_n(1));
        test_parameterized_bytecode!(0x20 => Bytecode::lload_n(2));
        test_parameterized_bytecode!(0x21 => Bytecode::lload_n(3));
    }

    #[test]
    fn test_decode_fload_n() {
        test_parameterized_bytecode!(0x22 => Bytecode::fload_n(0));
        test_parameterized_bytecode!(0x23 => Bytecode::fload_n(1));
        test_parameterized_bytecode!(0x24 => Bytecode::fload_n(2));
        test_parameterized_bytecode!(0x25 => Bytecode::fload_n(3));
    }

    #[test]
    fn test_decode_dload_n() {
        test_parameterized_bytecode!(0x26 => Bytecode::dload_n(0));
        test_parameterized_bytecode!(0x27 => Bytecode::dload_n(1));
        test_parameterized_bytecode!(0x28 => Bytecode::dload_n(2));
        test_parameterized_bytecode!(0x29 => Bytecode::dload_n(3));
    }

    #[test]
    fn test_decode_aload_n() {
        test_parameterized_bytecode!(0x2a => Bytecode::aload_n(0));
        test_parameterized_bytecode!(0x2b => Bytecode::aload_n(1));
        test_parameterized_bytecode!(0x2c => Bytecode::aload_n(2));
        test_parameterized_bytecode!(0x2d => Bytecode::aload_n(3));
    }

    #[test]
    fn test_decode_istore_n() {
        test_parameterized_bytecode!(0x3b => Bytecode::istore_n(0));
        test_parameterized_bytecode!(0x3c => Bytecode::istore_n(1));
        test_parameterized_bytecode!(0x3d => Bytecode::istore_n(2));
        test_parameterized_bytecode!(0x3e => Bytecode::istore_n(3));
    }

    #[test]
    fn test_decode_lstore_n() {
        test_parameterized_bytecode!(0x3f => Bytecode::lstore_n(0));
        test_parameterized_bytecode!(0x40 => Bytecode::lstore_n(1));
        test_parameterized_bytecode!(0x41 => Bytecode::lstore_n(2));
        test_parameterized_bytecode!(0x42 => Bytecode::lstore_n(3));
    }

    #[test]
    fn test_decode_fstore_n() {
        test_parameterized_bytecode!(0x43 => Bytecode::fstore_n(0));
        test_parameterized_bytecode!(0x44 => Bytecode::fstore_n(1));
        test_parameterized_bytecode!(0x45 => Bytecode::fstore_n(2));
        test_parameterized_bytecode!(0x46 => Bytecode::fstore_n(3));
    }

    #[test]
    fn test_decode_dstore_n() {
        test_parameterized_bytecode!(0x47 => Bytecode::dstore_n(0));
        test_parameterized_bytecode!(0x48 => Bytecode::dstore_n(1));
        test_parameterized_bytecode!(0x49 => Bytecode::dstore_n(2));
        test_parameterized_bytecode!(0x4a => Bytecode::dstore_n(3));
    }

    #[test]
    fn test_decode_astore_n() {
        test_parameterized_bytecode!(0x4b => Bytecode::astore_n(0));
        test_parameterized_bytecode!(0x4c => Bytecode::astore_n(1));
        test_parameterized_bytecode!(0x4d => Bytecode::astore_n(2));
        test_parameterized_bytecode!(0x4e => Bytecode::astore_n(3));
    }

    #[test]
    fn test_decode_tableswitch() {
        // Given
        let (pc, code) = (0, vec![
            0xaa, // tableswitch
            0, 0, 0, // 0 pad bytes
            0xaa, 0xbb, 0xcc, 0xdd, // default   = -1430532899
            0x00, 0x00, 0x00, 0x00, // low       = 0x00000000
            0x00, 0x00, 0x00, 0x02, // high      = 0x00000002
            0xff, 0xff, 0xff, 0xff, // offset[0] = -1
            0x07, 0x5b, 0xcd, 0x15, // offset[1] = 123456789
            0x00, 0x00, 0x00, 0x00, // offset[2] = 0
        ]);

        // When
        let result = Bytecode::decode(&code, pc);

        // Then
        let expected = Bytecode::tableswitch {
            default: -1430532899,
            low: 0,
            high: 2,
            offsets: vec![-1, 123456789, 0],
        };
        assert_eq!(expected, result.bytecode);
        assert_eq!(code.len(), result.newpc);
    }

    #[test]
    fn test_decode_tableswitch_with_offset() {
        // Given
        let (pc, code) = (3, vec![
            0x00, 0x00, 0x00,       // skip bytes
            0xaa,                   // tableswitch
            // No pad bytes, requires inital PC of 3
            0xaa, 0xbb, 0xcc, 0xdd, // default   = -1430532899
            0x00, 0x00, 0x00, 0x00, // low       = 0x00000000
            0x00, 0x00, 0x00, 0x02, // high      = 0x00000002
            0xff, 0xff, 0xff, 0xff, // offset[0] = -1
            0x07, 0x5b, 0xcd, 0x15, // offset[1] = 123456789
            0x00, 0x00, 0x00, 0x00, // offset[2] = 0
        ]);

        // When
        let result = Bytecode::decode(&code, pc);

        // Then
        let expected = Bytecode::tableswitch {
            default: -1430532899,
            low: 0,
            high: 2,
            offsets: vec![-1, 123456789, 0],
        };
        assert_eq!(expected, result.bytecode);
        assert_eq!(code.len(), result.newpc);
    }

    #[test]
    fn test_decode_lookupswitch() {
        // Given
        let (pc, code) = (0, vec![
            0xab, // tableswitch
            0, 0, 0, // 0 pad bytes
            0xaa, 0xbb, 0xcc, 0xdd, // default    = -1430532899
            0x00, 0x00, 0x00, 0x02, // npairs     = 0x00000002
            0xff, 0xff, 0xff, 0xff, // pairs[0].0 = -1
            0xff, 0xff, 0xff, 0xff, // pairs[0].1 = -1
            0x00, 0x00, 0x00, 0xff, // pairs[1].0 = 0xff
            0x00, 0x00, 0x00, 0xff, // pairs[1].1 = 0xff
        ]);

        // When
        let result = Bytecode::decode(&code, pc);

        // Then
        let expected = Bytecode::lookupswitch {
            default: -1430532899,
            npairs: 2,
            pairs: vec![(-1, -1), (0xff, 0xff)], 
        };
        assert_eq!(expected, result.bytecode);
        assert_eq!(pc + code.len(), result.newpc);
    }

    #[test]
    fn test_decode_lookupswitch_with_offset() {
        // Given
        let (pc, code) = (2, vec![
            0x00, 0x00,             // skip bytes
            0xab,                   // lookupswitch
            0,                      // 0 pad bytes (pc must be 2)
            0xaa, 0xbb, 0xcc, 0xdd, // default    = -1430532899
            0x00, 0x00, 0x00, 0x02, // npairs     = 0x00000002
            0xff, 0xff, 0xff, 0xff, // pairs[0].0 = -1
            0xff, 0xff, 0xff, 0xff, // pairs[0].1 = -1
            0x00, 0x00, 0x00, 0xff, // pairs[1].0 = 0xff
            0x00, 0x00, 0x00, 0xff, // pairs[1].1 = 0xff
        ]);

        // When
        let result = Bytecode::decode(&code, pc);

        // Then
        let expected = Bytecode::lookupswitch {
            default: -1430532899,
            npairs: 2,
            pairs: vec![(-1, -1), (0xff, 0xff)], 
        };
        assert_eq!(expected, result.bytecode);
        assert_eq!(code.len(), result.newpc);
    }

    #[test]
    fn test_decode_invokeinterface_skips_pad_byte() {
        // Given
        let (pc, code) = (0, vec![
            0xb9,       // invokeinterface
            0x0f, 0xff, // index = 4095,
            0xff,       // count = 255,
            0xff,       // unused
            0xac,       // ireturn
        ]);

        // When
        let result = Bytecode::decode(&code, pc);

        // Then
        let expected = Bytecode::invokeinterface {
            index: 4095,
            count: 255,
        };
        assert_eq!(expected, result.bytecode);
        assert_eq!(pc + code.len() - 1, result.newpc);

        // When - next bytecode
        let result = Bytecode::decode(&code, result.newpc);

        // Then
        let expected = Bytecode::ireturn;
        assert_eq!(expected, result.bytecode);
        assert_eq!(pc + code.len(), result.newpc);
    }

    #[test]
    fn test_decode_invokedynamic_skips_pad_bytes() {
        // Given
        let (pc, code) = (0, vec![
            0xba,       // invokedynamic
            0x0f, 0xff, // index = 4095,
            0xff, 0xff, // unused
            0xbf,       // athrow
        ]);

        // When
        let result = Bytecode::decode(&code, pc);

        // Then
        let expected = Bytecode::invokedynamic {
            index: 4095,
        };
        assert_eq!(expected, result.bytecode);
        assert_eq!(pc + code.len() - 1, result.newpc);

        // When - next bytecode
        let result = Bytecode::decode(&code, result.newpc);

        // Then
        let expected = Bytecode::athrow;
        assert_eq!(expected, result.bytecode);
        assert_eq!(pc + code.len(), result.newpc);
    }
}
