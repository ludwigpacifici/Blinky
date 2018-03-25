#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

extern crate compiler_builtins;

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 600) {
        unsafe {
            asm!("nop" :::: "volatile");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let gpio_16 = 16;

    // Set GPIO 16 as output
    // bits 20-18 set to 001 = GPIO Pin 16 is an output
    //   0b0000'0100'0000'0000'0000'0000 = 0x40000
    //        └20  └16  └12  └8   └4   └0
    GPIO_FSEL1.write_volatile(0b001 << 18);

    loop {
        GPIO_SET0.write_volatile(0b1 << gpio_16);
        spin_sleep_ms(1 * 1000);

        GPIO_CLR0.write_volatile(0b1 << gpio_16);
        spin_sleep_ms(1 * 1000);
    }
}
