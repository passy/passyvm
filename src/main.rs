#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]

mod vm;

use vm::bytecode::ByteCode;

fn main() {
    let mut code = [ByteCode::NOOP; 16];
    let mut my_vm = vm::VM::new(code);
    my_vm.computer();
}
