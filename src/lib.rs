#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(feature = "tokio")]
mod behaviour;
#[cfg(feature = "tokio")]
pub mod tokio;

#[cfg(feature = "tokio")]
pub use behaviour::Event;