//! Two tasks running at different priorities with access to the same resource
#![deny(unsafe_code)]
#![feature(abi_msp430_interrupt)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate msp430_rtfm as rtfm;
extern crate msp430g2553;

use rtfm::{app, Resource, Threshold};

app! {
    device: msp430g2553,

    resources: {
        static COUNTER: u32 = 0;
    },

    idle: {
        resources: [COUNTER],
    },

    tasks: {
        // the task `SYS_TICK` has higher priority than `TIM2`
        TIMER0_A0: {
            path: timer0_a0,
            resources: [COUNTER],
        },
    },
}

fn init(_p: init::Peripherals, _r: init::Resources) {
    // ..
}

fn idle(t: &mut Threshold, mut r: idle::Resources) -> ! {
    loop {
        // as this task runs at lower priority it needs a critical section to
        // prevent `sys_tick` from preempting it while it modifies this resource
        // data. The critical section is required to prevent data races which
        // can lead to data corruption or data loss
        rtfm::atomic(t, |t| { **r.COUNTER.borrow_mut(t) += 1; })
    }
}

fn timer0_a0(_t: &mut Threshold, r: TIMER0_A0::Resources) {
    // ..

    // this task can't be preempted by `tim2` so it has direct access to the
    // resource data
    **r.COUNTER += 1;

    // ..
}
