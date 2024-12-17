use std::ptr::null_mut;
use crate::roc_ffi;
struct Builder
{
    context: *mut roc_ffi::roc_context,
}

impl Builder {
    fn new() -> Builder {
        unsafe {
            let mut config: roc_ffi::roc_context_config = roc_ffi::roc_context_config {max_packet_size: 0, max_frame_size: 0};
            let mut ctx: *mut roc_ffi::roc_context = null_mut();
            let err = roc_ffi::roc_context_open(&mut config, &mut ctx);
            Builder {context: ctx}
        }

    }
}