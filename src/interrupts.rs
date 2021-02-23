use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

static IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame
) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}