#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Arithmetic field instructions.
    ADD = 0,
    SUB = 1,
    MUL = 2,
    DIV = 3,

    // Arithmetic field extension operations.
    EADD = 10,
    ESUB = 11,
    EMUL = 12,
    EDIV = 13,

    // Mixed arithmetic operations.
    EFADD = 20,
    EFSUB = 21,
    EFMUL = 22,
    EFDIV = 23,

    // Memory instructions.
    LW = 4,
    SW = 5,
    LE = 14,
    SE = 15,

    // Branch instructions.
    BEQ = 6,
    BNE = 7,
    EBEQ = 16,
    EBNE = 17,

    // Jump instructions.
    JAL = 8,
    JALR = 9,

    // Custom instructions.
    NUM2BITS32 = 10,

    // System instructions.
    TRAP = 30,
}
