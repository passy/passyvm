type ByteCode = u8;

pub struct VM {
    ip: u32, // instruction pointer register
    sp: u32, // stack pointer register
    fp: u32,  // frame pointer register

    code: [ByteCode; 16], // word-addressable code memory as bytecodes
    stack: [u32; 256], // operand stack, grows upwards, fixed in size for now
}

impl VM {
    // 16 bytes ought to be enough for everyone!
    pub fn new(code: [ByteCode; 16]) -> VM {
        VM { ip: 0, sp: -1, fp: -1, code: code, stack: [0; 256] }
    }

    pub fn computer(&self) {
        println!("beep boop beep");

        for code in self.code.iter() {
            println!("code: {}", code);
        }
    }
}
