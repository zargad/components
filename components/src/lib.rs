pub mod components;
pub mod channels;
pub mod processes;

#[doc(hidden)]
pub mod __private {
    pub use crate::channels::Channel;
    pub use crate::channels::DuelChannel;
}

#[cfg(feature = "macros")]
pub use components_macros::Channels;
