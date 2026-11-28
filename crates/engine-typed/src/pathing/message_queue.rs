//! Linked queue of two-word path messages. The tail points to the link that receives the next node.

use crate::image;
use engine_core::ptr::Ptr;
use engine_core::typed::leaf;
use engine_core::{args, typed_call, Engine};

image! {
    pub struct Queue / QueueImage: 0x14 {
        0x0 _base: [u8; 8],
        0x8 pool: u32,
        0xc head: u32,
        0x10 tail_link: u32,
    }
}

const NEXT: u32 = 8;

// Typed from 006ec260
pub fn try_push(e: &mut Engine, this: Ptr<Queue>, message: u32) -> bool {
    let pool = e.get(this, Queue::pool);
    if pool == 0 {
        return false;
    }
    e.with_stack(4, |e, slot| {
        if !e.call(0x006e_cd00, &args![pool, slot, message]).bool() {
            return false;
        }
        if e.get(this, Queue::head) == 0 {
            e.set(
                this,
                Queue::tail_link,
                this.byte_add(Queue::head.off).addr(),
            );
        }
        let link = e.get(this, Queue::tail_link);
        let node = e.mem.u32(slot.addr());
        e.mem.set_u32(link, node);
        let next = node.wrapping_add(NEXT);
        e.mem.set_u32(next, 0);
        e.set(this, Queue::tail_link, next);
        true
    })
}

// Typed from 006ec2e0
pub fn try_pop(e: &mut Engine, this: Ptr<Queue>, output: Ptr) -> bool {
    let head = e.get(this, Queue::head);
    if head == 0 {
        return false;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), head);
        let source = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, head));
        e.call(0x006e_aa00, &args![output, source]);
        let next = e.mem.u32(head.wrapping_add(NEXT));
        e.set(this, Queue::head, next);
        e.call(0x006e_cb10, &args![e.get(this, Queue::pool), slot]);
    });
    true
}
