// use std::fs::{File};
// use std::io::{Read};
// use std::os::unix::io::{FromRawFd, RawFd};
//
// use criterion::measurement::{Measurement, ValueFormatter};
//
// use perf_event_open_sys as sys;
// use crate::perf::formatter;
//
// struct Energy;
// impl Measurement for Energy {
//     type Intermediate = RawFd;
//     type Value = u64;
//
//
//     fn start(&self) -> Self::Intermediate {
//
//
// // Construct a zero-filled `perf_event_attr`.
// //         let mut attrs = sys::bindings::perf_event_attr::default();
//
// // Populate the fields we need.
// //         attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;
// //         attrs.type_ = 19;
// //         attrs.config = 2;
//
//         // attrs.set_disabled(1);
//         // attrs.set_exclude_kernel(1);
//         // attrs.set_exclude_hv(1);
//
// // Make the system call.
//
//         // let fd = unsafe {
//         //     sys::perf_event_open(&mut attrs,  0, 0, -1, 0)
//         // };
//         //
//         // if fd < 0 {
//         //     panic!("Could not open file Err: {}", fd)
//             // ... handle error
//         // }
//         // unsafe {
//         //     File::from_raw_fd(fd).read_u64()
//         // }
//         // fd
//     }
//
//     fn end(&self, i: Self::Intermediate) -> Self::Value {
//         let mut f = unsafe { File::from_raw_fd(i) };
//         let mut buf = [0_u8; 8];
//         f.read_exact(&mut buf).ok();
//         u64::from_be_bytes(buf)
//     }
//
//     fn add(&self, v1: &Self::Value, v2: &Self::Value) -> Self::Value {
//         v1 + v2
//     }
//
//     fn zero(&self) -> Self::Value {
//         0u64
//     }
//
//     fn to_f64(&self, value: &Self::Value) -> f64 {
//         f64::from(*value as u32)
//     }
//
//     fn formatter(&self) -> &dyn ValueFormatter {
//         formatter::EnergyFormatter
//     }
// }
