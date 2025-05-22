// Copyright (C) 2024  Jonathan Thomason

#[cfg(target_arch = "x86_64")]
#[path = "x86-64/mod.rs"]
pub mod x86_64;

use core::arch::asm;

/// Halt and catch fire.
pub fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}
