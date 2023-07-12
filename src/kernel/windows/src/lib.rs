#![no_main]
#![no_std]

use kernel_alloc::KernelAlloc;

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;
