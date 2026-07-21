mod r#fn;

use crate::*;

use std::{
    sync::{Arc, Mutex},
    thread::{JoinHandle, spawn},
    time::Instant,
};
