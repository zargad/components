pub mod components;
pub mod channels;
pub mod processes;

#[doc(hidden)]
pub mod __private {
    pub use crate::channels::Channel;
}

#[cfg(feature = "macros")]
pub use components_macros::Channels;
