use std::ptr::null_mut;
use crate::roc_ffi;
use crate::config::*;
use std::os::raw::c_uint;

struct Context
{
    context: *mut roc_ffi::roc_context,
}

impl Context {
    fn new(max_packet_sz: Option<usize>, max_frame_sz: Option<usize>) -> Result<Context, &'static str> {
        let pkt_sz = c_uint::try_from(max_packet_sz.unwrap_or(0))
            .or(Err("max_packet_sz value should fit into c_uint type"))?;
        let frame_sz = c_uint::try_from(max_frame_sz.unwrap_or(0))
            .or(Err("frame_sz value should fit into c_uint type"))?;
        unsafe {
            let config: roc_ffi::roc_context_config =
                roc_ffi::roc_context_config {
                    max_packet_size: pkt_sz,
                    max_frame_size: frame_sz
                };
            let mut ctx: *mut roc_ffi::roc_context = null_mut();
            match roc_ffi::roc_context_open(&config, &mut ctx) {
                0 => Ok(Context {context: ctx}),
                _ => Err("Failed to open context")
            }
        }
    }

    fn register_encoding(&mut self, encoding_id: i32, encoding: MediaEncoding) -> Result<(), &'static str> {
        unsafe {
            let encoder_id: c_uint = encoding_id.try_into()
                .or(Err("encoding_id doesn't fit into c_uint"))?;
            let rate = encoding.rate.try_into()
                .or(Err("sample_rate doesn't fit into c_uint"))?;
            let format = roc_ffi::roc_format_ROC_FORMAT_PCM_FLOAT32;
            let mut encoding_c = roc_ffi::roc_media_encoding {
                rate,
                format,
                channels: roc_ffi::roc_channel_layout_ROC_CHANNEL_LAYOUT_MONO,
                tracks: 0
            };
            match encoding.channel_layout {
                ChannelLayout::MONO => {
                    encoding_c.channels = roc_ffi::roc_channel_layout_ROC_CHANNEL_LAYOUT_MONO;
                    encoding_c.tracks = 0;
                }
                ChannelLayout::STEREO => {
                    encoding_c.channels = roc_ffi::roc_channel_layout_ROC_CHANNEL_LAYOUT_STEREO;
                    encoding_c.tracks = 0;
                }
                ChannelLayout::MultitrackLayer(tracks) => {
                    encoding_c.channels = roc_ffi::roc_channel_layout_ROC_CHANNEL_LAYOUT_MULTITRACK;
                    encoding_c.tracks = tracks.try_into().or(Err("tracks doesn't fit into c_uint"))?;
                }
            }

            let encoding_id_c = encoder_id.try_into().or(Err("encoding_id doesn't fit into c_uint"))?;
            match roc_ffi::roc_context_register_encoding(self.context, encoding_id_c, &mut encoding_c) {
                0 => Ok(()),
                _ => Err("Failed to register context")
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if self.context == null_mut() {
                return;
            }
            if roc_ffi::roc_context_close(self.context) != 0 {
                panic!("roc_ffi::roc_context_close failed");
            }
        }
    }
}
