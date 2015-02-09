#[derive(Copy, Debug)]
#[allow(dead_code)]
pub enum ByteCode {
    NOOP   = 0,
    IADD   = 1,
    ISUB   = 2,
    IMUL   = 3,
    ILT    = 4,
    IEQ    = 5,
    BR     = 6,
    BRT    = 7,
    BRF    = 8,
    ICONST = 9,
    LOAD   = 10,
    GLOAD  = 11,
    STORE  = 12,
    GSTORE = 13,
    PRINT  = 14,
    POP    = 15,
    HALT   = 16
}
