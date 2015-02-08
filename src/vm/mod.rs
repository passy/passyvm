pub struct VM {
    ip: u32, // instruction pointer register
    sp: u32, // stack pointer register
    fp: u32  // frame pointer register
}

impl VM {
    pub fn new() -> VM {
        VM { ip: 0, sp: -1, fp: -1 }
    }

    pub fn computer(&self) {
        println!("beep boop beep")
    }
}
