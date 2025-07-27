//! tcp-request
//!
//! A Rust library for sending raw TCP requests, with features
//! for handling responses, managing redirects, and working
//! with compressed data over TCP connections.

#[cfg(test)]
mod cfg;
pub(crate) mod common;
pub(crate) mod request;
pub(crate) mod response;

pub use request::*;
pub use response::*;

pub(crate) use common::*;

pub(crate) use std::{
    error::Error as StdError,
    fmt::Debug,
    fmt::{self, Display},
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, RwLock},
    time::Duration,
};
