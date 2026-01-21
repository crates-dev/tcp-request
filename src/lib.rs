//! tcp-request
//!
//! A Rust library for sending raw TCP requests, with features
//! for handling responses, managing redirects, and working
//! with compressed data over TCP connections.

pub(crate) mod common;
pub(crate) mod request;
pub(crate) mod response;

pub use {request::*, response::*};

use common::*;

use std::{
    error::Error as StdError,
    fmt::Debug,
    fmt::{self, Display},
    io::{Read, Write},
    net::TcpStream,
    sync::{Arc, RwLock},
    time::Duration,
};

#[cfg(test)]
use std::{
    sync::Mutex,
    thread::{JoinHandle, spawn},
    time::Instant,
};
