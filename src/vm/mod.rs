pub mod bytecode;

use self::bytecode::ByteCode;

#[allow(dead_code)]
pub struct VM {
    ip: usize, // instruction pointer register
    sp: usize, // stack pointer register
    fp: usize,  // frame pointer register

    code: [ByteCode; 16], // word-addressable code memory as bytecodes
    stack: [u32; 256], // operand stack, grows upwards, fixed in size for now
}

impl VM {
    // 16 bytes ought to be enough for everyone!
    pub fn new(code: [ByteCode; 16]) -> VM {
        VM { ip: 0, sp: -1, fp: -1, code: code, stack: [0; 256] }
    }

    pub fn computer(&mut self) {

        while self.ip < self.code.len() {
            let opcode = self.code[self.ip];
            self.ip += 1;

            match opcode {
                ByteCode::NOOP => continue,
                ByteCode::HALT => break,
                _ => panic!("Received unknown opcode {:?}!", opcode)
            }
        }
    }
}
