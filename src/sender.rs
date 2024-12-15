use std::ptr::null;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

struct Builder
{
    context: *mut roc_context,
}

impl Builder {
    fn new() -> Builder {
        unsafe {
            let mut config: roc_context_config = roc_context_config {max_packet_size: 0, max_frame_size: 0};
            let mut ctx: *mut roc_context = std::ptr::null_mut();
            let err = roc_context_open(&mut config, &mut ctx);
            Builder {context: ctx}
        }

    }
}