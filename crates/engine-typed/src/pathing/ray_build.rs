//! Build ray casts for the path task entries still waiting for results.

use crate::{dispatch, multibound};
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::{args, typed_call, Engine};

fn entry(e: &mut Engine, array: Ptr, index: u32) -> u32 {
    let slot = typed_call!(
        leaf::FAMILY,
        0x0087_7a30,
        leaf::array_element(e, array, index)
    );
    e.mem.u32(slot)
}

/// Build pending paths under the task lock until the signed total exceeds ten.
// Typed from 006ea350
pub fn build_pending(e: &mut Engine, this: Ptr) -> i32 {
    let lock = this.byte_add(0x20).cast();
    let requests = this.byte_add(0x40);
    let paths = this.byte_add(0x50);
    typed_call!(
        dispatch::FAMILY,
        0x0040_fbf0,
        dispatch::refs::lock(e, lock, 0)
    );
    let built = e.with_stack(0x94, |e, ray| {
        let mut total = 0i32;
        let mut index = 0;
        while index < typed_call!(leaf::FAMILY, 0x0044_ddc0, leaf::word_at_8(e, requests)) {
            let path = entry(e, paths, index);
            if e.mem.u32(path.wrapping_add(4)) == 0 {
                e.call(0x006e_61a0, &args![ray]);
                let path = entry(e, paths, index);
                let request = entry(e, requests, index);
                e.call(0x006e_6320, &args![ray, request, path]);
                let count = typed_call!(
                    multibound::FAMILY,
                    0x004f_d3e0,
                    multibound::getters::world_space(e, ray.cast())
                );
                total = total.wrapping_add(count as i32);
                e.call(0x006e_62b0, &args![ray]);
                if total > 10 {
                    break;
                }
            }
            index += 1;
        }
        total
    });
    typed_call!(
        dispatch::FAMILY,
        0x0040_fba0,
        dispatch::refs::unlock(e, lock)
    );
    built
}
