#[allow(non_camel_case_types)]
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
    istore,
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
    lstore,
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
    (u32 $code:expr, $base:expr) => {{
        let byte1: u32 = $code[$base] as u32;
        let byte2: u32 = $code[$base + 1] as u32;
        let byte3: u32 = $code[$base + 2] as u32;
        let byte4: u32 = $code[$base + 3] as u32;
        $base = $base + 4;
        (byte1 << 24 | byte2 << 16 | byte3 << 8 | byte4)
    }};
    (i32 $code:expr, $base:expr) => {{
        fetch!(u32 $code, $base) as i32
    }};
    (u16 $code:expr, $base:expr) => {{
        let byte1: u16 = $code[$base] as u16;
        let byte2: u16 = $code[$base + 1] as u16;
        $base = $base + 2;
        (byte1 << 8 | byte2)
    }};
    (i16 $code:expr, $base:expr) => {{
        fetch!(u16 $code, $base) as i16
    }};
    (u8 $code:expr, $base:expr) => {{
        let byte: u8 = $code[$base] as u8;
        $base = $base + 1;
        byte
    }};
}

macro_rules! bytecode {
    ($name:ident) => {
        DecodeResult {
            bytecode: Bytecode::$name,
            consumed: 1,
        }
    };
    ($name:ident, $val:expr) => {
        DecodeResult {
            bytecode: Bytecode::$name($val),
            consumed: 1,
        }
    };
    ($code:expr, $name:ident, $field:ident : u8) => {
        DecodeResult {
            bytecode: Bytecode::$name { $field: $code[1] },
            consumed: 2,
        }
    };
    ($code:expr, $name:ident, $field:ident : i16) => {{
        let mut consumed: usize = 1;
        let value = fetch!(i16 $code, consumed);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            consumed: consumed,
        }
    }};
    ($code:expr, $name:ident, $field:ident : u16) => {{
        let mut consumed: usize = 1;
        let value = fetch!(u16 $code, consumed);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            consumed: consumed,
        }
    }};
    ($code:expr, $name:ident, $field:ident : i16) => {{
        let mut consumed: usize = 1;
        let value = fetch!(i16 $code, consumed);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            consumed: consumed,
        }
    }};
    ($code:expr, $name:ident, $field:ident : u32) => {{
        let mut consumed: usize = 1;
        let value = fetch!(u32 $code, consumed);
        DecodeResult {
            bytecode: Bytecode::$name { $field: value },
            consumed: consumed,
        }
    }};
}

pub struct DecodeResult {
    bytecode: Bytecode,
    consumed: usize,
}

impl Bytecode {
    /// Decodes a single instruction from the code slice.
    ///
    /// Returns the decoded bytecode and how many bytes from the instruction
    /// stream were consumed by this decoded instruction.
    pub fn decode(code: &[u8], pc: u16) -> DecodeResult {
        match code[0] {
            0x00 => bytecode!(nop),
            0x01 => bytecode!(aconst_null),
            i @ 0x02...0x08 => bytecode!(iconst_i, i as i8 - 0x03),
            l @ 0x09...0x0a => bytecode!(lconst_l, l as u8 - 0x09),
            f @ 0x0b...0x0d => bytecode!(fconst_f, f as u8 - 0x0b),
            d @ 0x0e...0x0f => bytecode!(dconst_d, d as u8 - 0x0e),
            0x10 => bytecode!(code, bipush, byte: u8),
            0x11 => bytecode!(code, sipush, short: i16),
            0x12 => bytecode!(code, ldc, index: u8),
            0x13 => bytecode!(code, ldc_w, index: u16),
            0x14 => bytecode!(code, ldc2_w, index: u16),
            0x15 => bytecode!(code, iload, index: u8),
            0x16 => bytecode!(code, lload, index: u8),
            0x17 => bytecode!(code, fload, index: u8),
            0x18 => bytecode!(code, dload, byte: u8),
            0x19 => bytecode!(code, aload, index: u8),
            n @ 0x1a...0x1d => bytecode!(iload_n, n as u8 - 0x1a),
            n @ 0x1e...0x21 => bytecode!(lload_n, n as u8 - 0x1e),
            n @ 0x22...0x25 => bytecode!(fload_n, n as u8 - 0x22),
            n @ 0x26...0x29 => bytecode!(dload_n, n as u8 - 0x26),
            n @ 0x2a...0x2d => bytecode!(aload_n, n as u8 - 0x2a),
            0x2e => bytecode!(iaload),
            0x2f => bytecode!(laload),
            0x30 => bytecode!(faload),
            0x31 => bytecode!(daload),
            0x32 => bytecode!(aaload),
            0x33 => bytecode!(baload),
            0x34 => bytecode!(caload),
            0x35 => bytecode!(saload),
            0x36 => bytecode!(istore),
            0x37 => bytecode!(lstore),
            0x38 => bytecode!(code, fstore, index: u8),
            0x39 => bytecode!(code, dstore, index: u8),
            0x3a => bytecode!(code, astore, index: u8),
            n @ 0x3b...0x3e => bytecode!(istore_n, n as u8 - 0x3b),
            n @ 0x3f...0x42 => bytecode!(lstore_n, n as u8 - 0x3f),
            n @ 0x43...0x46 => bytecode!(fstore_n, n as u8 - 0x43),
            n @ 0x47...0x4a => bytecode!(dstore_n, n as u8 - 0x47),
            n @ 0x4b...0x4e => bytecode!(astore_n, n as u8 - 0x4b),
            0x4f => bytecode!(iastore),
            0x50 => bytecode!(lastore),
            0x51 => bytecode!(fastore),
            0x52 => bytecode!(dastore),
            0x53 => bytecode!(aastore),
            0x54 => bytecode!(bastore),
            0x55 => bytecode!(castore),
            0x56 => bytecode!(sastore),
            0x57 => bytecode!(pop),
            0x58 => bytecode!(pop2),
            0x59 => bytecode!(dup),
            0x5a => bytecode!(dup_x1),
            0x5b => bytecode!(dup_x2),
            0x5c => bytecode!(dup2),
            0x5d => bytecode!(dup2_x1),
            0x5e => bytecode!(dup2_x2),
            0x5f => bytecode!(swap),
            0x60 => bytecode!(iadd),
            0x61 => bytecode!(ladd),
            0x62 => bytecode!(fadd),
            0x63 => bytecode!(dadd),
            0x64 => bytecode!(isub),
            0x65 => bytecode!(lsub),
            0x66 => bytecode!(fsub),
            0x67 => bytecode!(dsub),
            0x68 => bytecode!(imul),
            0x69 => bytecode!(lmul),
            0x6a => bytecode!(fmul),
            0x6b => bytecode!(dmul),
            0x6c => bytecode!(idiv),
            0x6d => bytecode!(ldiv),
            0x6e => bytecode!(fdiv),
            0x6f => bytecode!(ddiv),
            0x70 => bytecode!(irem),
            0x71 => bytecode!(lrem),
            0x72 => bytecode!(frem),
            0x73 => bytecode!(drem),
            0x74 => bytecode!(ineg),
            0x75 => bytecode!(lneg),
            0x76 => bytecode!(fneg),
            0x77 => bytecode!(dneg),
            0x78 => bytecode!(ishl),
            0x79 => bytecode!(lshl),
            0x7a => bytecode!(ishr),
            0x7b => bytecode!(lshr),
            0x7c => bytecode!(iushr),
            0x7d => bytecode!(lushr),
            0x7e => bytecode!(iand),
            0x7f => bytecode!(land),
            0x80 => bytecode!(ior),
            0x81 => bytecode!(lor),
            0x82 => bytecode!(ixor),
            0x83 => bytecode!(lxor),
            0x84 => {
                DecodeResult {
                    bytecode: Bytecode::iinc {
                        index: code[1],
                        constant: code[2],
                    },
                    consumed: 3,
                }
            }
            0x85 => bytecode!(i2l),
            0x86 => bytecode!(i2f),
            0x87 => bytecode!(i2d),
            0x88 => bytecode!(l2i),
            0x89 => bytecode!(l2f),
            0x8a => bytecode!(l2d),
            0x8b => bytecode!(f2i),
            0x8c => bytecode!(f2l),
            0x8d => bytecode!(f2d),
            0x8e => bytecode!(d2i),
            0x8f => bytecode!(d2l),
            0x90 => bytecode!(d2f),
            0x91 => bytecode!(i2b),
            0x92 => bytecode!(i2c),
            0x93 => bytecode!(i2s),
            0x94 => bytecode!(lcmp),
            0x95 => bytecode!(fcmpl),
            0x96 => bytecode!(fcmpg),
            0x97 => bytecode!(dcmpl),
            0x98 => bytecode!(dcmpg),
            0x99 => bytecode!(code, ifeq, branchoffset: u16),
            0x9a => bytecode!(code, ifne, branchoffset: u16),
            0x9b => bytecode!(code, iflt, branchoffset: u16),
            0x9c => bytecode!(code, ifge, branchoffset: u16),
            0x9d => bytecode!(code, ifgt, branchoffset: u16),
            0x9e => bytecode!(code, ifle, branchoffset: u16),
            0x9f => bytecode!(code, if_icmpeq, branchoffset: u16),
            0xa0 => bytecode!(code, if_icmpne, branchoffset: u16),
            0xa1 => bytecode!(code, if_icmplt, branchoffset: u16),
            0xa2 => bytecode!(code, if_icmpge, branchoffset: u16),
            0xa3 => bytecode!(code, if_icmpgt, branchoffset: u16),
            0xa4 => bytecode!(code, if_icmple, branchoffset: u16),
            0xa5 => bytecode!(code, if_acmpeq, branchoffset: u16),
            0xa6 => bytecode!(code, if_acmpne, branchoffset: u16),
            0xa7 => bytecode!(code, goto, branchoffset: u16),
            0xa8 => bytecode!(code, jsr, branchoffset: u16),
            0xa9 => bytecode!(code, ret, index: u8),
            0xaa => {
                let mut index: usize = 4 - (pc % 4) as usize;
                let default = fetch!(i32 code, index);
                let low = fetch!(i32 code, index);
                let high = fetch!(i32 code, index);
                let mut offsets: Vec<i32> = vec![];
                let offset_count = high - low + 1;
                for _ in 0..offset_count {
                    let offset = fetch!(i32 code, index);
                    offsets.push(offset);
                }
                DecodeResult {
                    bytecode: Bytecode::tableswitch {
                        default: default,
                        low: low,
                        high: high,
                        offsets: offsets,
                    },
                    consumed: index,
                }
            }
            0xab => {
                let mut index: usize = 4 - (pc % 4) as usize;
                let default = fetch!(i32 code, index);
                let npairs = fetch!(i32 code, index);
                let mut pairs: Vec<(i32, i32)> = vec![];
                for _ in 0..npairs {
                    let first = fetch!(i32 code, index);
                    let second = fetch!(i32 code, index);
                    pairs.push((first, second));
                }
                DecodeResult {
                    bytecode: Bytecode::lookupswitch {
                        default: default,
                        npairs: npairs,
                        pairs: pairs,
                    },
                    consumed: index,
                }
            }
            0xac => bytecode!(ireturn),
            0xad => bytecode!(lreturn),
            0xae => bytecode!(freturn),
            0xaf => bytecode!(dreturn),
            0xb0 => bytecode!(areturn),
            0xb1 => bytecode!(Return),
            0xb2 => bytecode!(code, getstatic, index: u16),
            0xb3 => bytecode!(code, putstatic, index: u16),
            0xb4 => bytecode!(code, getfield, index: u16),
            0xb5 => bytecode!(code, putfield, index: u16),
            0xb6 => bytecode!(code, invokevirtual, index: u16),
            0xb7 => bytecode!(code, invokespecial, index: u16),
            0xb8 => bytecode!(code, invokestatic, index: u16),
            0xb9 => {
                let mut offset: usize = 0;
                let index = fetch!(u16 code, offset);
                let count = fetch!(u8 code, offset);
                let _ignore = fetch!(u8 code, offset);
                DecodeResult {
                    bytecode: Bytecode::invokeinterface {
                        index: index,
                        count: count,
                    },
                    consumed: offset,
                }
            }
            0xba => {
                let mut offset: usize = 0;
                let index = fetch!(u16 code, offset);
                let _ignore = fetch!(u8 code, offset);
                let _ignore = fetch!(u8 code, offset);
                DecodeResult {
                    bytecode: Bytecode::invokedynamic { index: index },
                    consumed: offset,
                }
            }
            0xbb => bytecode!(code, new, index: u16),
            0xbc => bytecode!(code, newarray, atype: u8),
            0xbd => bytecode!(code, anewarray, index: u16),
            0xbe => bytecode!(arraylength),
            0xbf => bytecode!(athrow),
            0xc0 => bytecode!(code, checkcast, index: u16),
            0xc1 => bytecode!(code, instanceof, index: u16),
            0xc2 => bytecode!(monitorenter),
            0xc3 => bytecode!(monitorexit),
            0xc4 => {
                let mut _offset: usize = 0;
                let opcode = fetch!(u8 code, _offset);
                let mut result = match opcode {
                    0x15 => bytecode!(code, wide_iload, index: u16),
                    0x16 => bytecode!(code, wide_lload, index: u16),
                    0x17 => bytecode!(code, wide_fload, index: u16),
                    0x18 => bytecode!(code, wide_dload, index: u16),
                    0x19 => bytecode!(code, wide_aload, index: u16),
                    0x36 => bytecode!(code, wide_istore, index: u16),
                    0x37 => bytecode!(code, wide_lstore, index: u16),
                    0x38 => bytecode!(code, wide_fstore, index: u16),
                    0x39 => bytecode!(code, wide_dstore, index: u16),
                    0x3a => bytecode!(code, wide_astore, index: u16),
                    0x84 => {
                        let mut offset: usize = 0;
                        let index = fetch!(u16 code, offset);
                        let constant = fetch!(u16 code, offset);
                        DecodeResult {
                            bytecode: Bytecode::wide_iinc {
                                index: index,
                                constant: constant,
                            },
                            consumed: offset,
                        }
                    }
                    0xa9 => bytecode!(code, wide_ret, index: u16),
                    op @ _ => bytecode!(invalid, op),
                };
                // Add in the consumed opcode byte
                result.consumed += 1;
                result
            }
            0xc5 => {
                let mut offset: usize = 0;
                let index = fetch!(u16 code, offset);
                let dimensions = fetch!(u8 code, offset);
                DecodeResult {
                    bytecode: Bytecode::multianewarray {
                        index: index,
                        dimensions: dimensions,
                    },
                    consumed: offset,
                }
            }
            0xc6 => bytecode!(code, ifnull, branchoffset: u16),
            0xc7 => bytecode!(code, ifnonnull, branchoffset: u16),
            0xc8 => bytecode!(code, goto_w, branchoffset: u32),
            0xc9 => bytecode!(code, jsr_w, branchoffset: u32),
            op @ _ => bytecode!(invalid, op),
        }
    }
}
