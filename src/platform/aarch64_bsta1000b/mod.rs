pub mod mem;
pub mod misc;

#[cfg(feature = "smp")]
pub mod mp;

#[cfg(feature = "irq")]
pub mod irq {
    pub use crate::platform::aarch64_common::gic::*;
}

pub mod time {
    pub use crate::platform::aarch64_common::generic_timer::*;
}

extern "C" {
    fn exception_vector_base();
    fn rust_main(cpu_id: usize, dtb: usize);
    #[cfg(feature = "smp")]
    fn rust_main_secondary(cpu_id: usize);
}

pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    crate::mem::clear_bss();
    crate::arch::set_exception_vector_base(exception_vector_base as usize);
    crate::cpu::init_primary(cpu_id);
    crate::platform::console::init_early();
    crate::platform::time::init_early();
    rust_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
pub(crate) unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    crate::arch::set_exception_vector_base(exception_vector_base as usize);
    crate::cpu::init_secondary(cpu_id);
    rust_main_secondary(cpu_id);
}

/// Initializes the platform devices for the primary CPU.
///
/// For example, the interrupt controller and the timer.
pub fn platform_init() {
    #[cfg(feature = "irq")]
    super::aarch64_common::gic::init_primary();
    crate::platform::time::init_percpu();
    #[cfg(feature = "irq")]
    crate::platform::console::init_irq();
}

/// Initializes the platform devices for secondary CPUs.
#[cfg(feature = "smp")]
pub fn platform_init_secondary() {
    #[cfg(feature = "irq")]
    super::aarch64_common::gic::init_secondary();
    crate::platform::time::init_percpu();
}
