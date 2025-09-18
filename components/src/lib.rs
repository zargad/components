pub mod channels;
pub mod processes;

#[doc(hidden)]
pub mod __private {
    pub use crate::channels::ChannelGet;
    pub use crate::channels::ChannelSet;
}

#[cfg(feature = "macros")]
pub use components_macros::Channels;
