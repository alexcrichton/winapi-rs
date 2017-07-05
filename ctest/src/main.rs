#![allow(bad_style)]

extern crate winapi;

use winapi::ctypes::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
