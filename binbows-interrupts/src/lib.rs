#![no_std]
#![feature(abi_x86_interrupt)]

use once_cell::unsync::OnceCell;
use x86_64::structures::idt::InterruptDescriptorTable;

pub mod interrupts;

pub mod cpu_exceptions;

static mut IDT: OnceCell<InterruptDescriptorTable> = OnceCell::new();

pub fn idt_init() {
    let idt = unsafe {
        #[allow(static_mut_refs)]
        IDT.get_or_init(|| {
            let mut idt = InterruptDescriptorTable::new();
            cpu_exceptions::setup_cpu_exceptions(&mut idt);
            interrupts::setup_hardware_interrupts(&mut idt);
            idt
        })
    };
    idt.load();
}