pub enum ChannelLayout {
    MONO,
    STEREO,
    MultitrackLayer(u16),
}

pub struct MediaEncoding {
    pub rate: u32,
    pub channel_layout: ChannelLayout,
}