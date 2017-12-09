#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate stm32f103xx;

use rtfm::app;

app! { //~ error attempt to subtract with overflow
    device: stm32f103xx,

    tasks: {
        SYS_TICK: {
            path: sys_tick,
            // ERROR priority must be in the range [1, 16]
            priority: 0,
        },
    },
}

fn init(_p: init::Peripherals) {}

fn idle() -> ! {
    loop {}
}

fn sys_tick() {}
