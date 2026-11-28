//! Start and stop the path manager implementation's task group.

use engine_core::{args, ptr::Ptr, Engine};

const TASK_MANAGER: u32 = 0x00b0_0a00;

// Typed from 006eabe0
pub fn initialize(e: &mut Engine, this: Ptr) -> bool {
    let manager = e.call(TASK_MANAGER, &[]).u32();
    if !e.call(0x00b0_0a80, &args![manager, this]).bool()
        || !e.call(0x006c_08e0, &args![this]).bool()
    {
        return false;
    }
    e.mem.set_u32(this.byte_add(0xc24).addr(), 0); // state
    e.mem.set_u32(this.byte_add(0xc28).addr(), 0); // pause retries
    e.mem.set_u8(this.byte_add(0xc34).addr(), 0); // cancel paths
    true
}

// Typed from 006eac40
pub fn destroy(e: &mut Engine, this: Ptr) {
    e.call(0x006c_09d0, &args![this]);
    let manager = e.call(TASK_MANAGER, &[]).u32();
    e.call(0x00b0_0c20, &args![manager, this]);
}
