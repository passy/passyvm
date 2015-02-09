#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]

mod vm;

fn main() {
    let code = [0; 16];
    let my_vm = vm::VM::new(code);
    my_vm.computer();
}
