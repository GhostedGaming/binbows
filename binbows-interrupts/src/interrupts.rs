use core::arch::asm;

use binbows_logging::serial_println;
use x86_64::structures::idt::{ InterruptStackFrame, InterruptDescriptorTable };

pub fn setup_hardware_interrupts(idt: &mut InterruptDescriptorTable) {
    idt[32].set_handler_fn(timer_interrupt_handler);
}

fn send_eoi() {
    unsafe {
        asm!(
            "mov al, 0x20",
            "out 0x20, al",
            options(nomem, nostack, preserves_flags)
        )
    }
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // serial_println!("Timer Interrupt"); // Uncomment for debugging only!
    send_eoi();
}