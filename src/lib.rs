// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![cfg_attr(test, deny(rust_2018_idioms))]
//! # pqkd
//!
//! pqkd is a client implementation in and for Rust for pQKD from
//! [QuantumBlockchains](https://www.quantumblockchains.io/pqkd/). 
//! This library is designed
//! to work with the pQKD device, which means that it makes
//! no sense to use the code without this device.  
//! 
//! ## Features
//! 
//! pqkd allows you to send keys to other pQKD devices,
//! receive them and also receive random values from the
//! pQKD device in hex, bytes, base64 format. 
pub use crate::async_impl::pqkd::BuilderPqkdClient;
pub use crate::async_impl::pqkd::PqkdClient;
pub use crate::async_impl::request_builder::PqkdRequestBuilder;
pub use crate::response::{PqkdStatus, Key, PqkdResponse};
pub(crate) use crate::response::Keys;


pub mod qrng;
pub mod error;
pub mod blocking;
mod async_impl;
pub mod request;
pub mod response;

