use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::raw::c_int;
use std::os::unix::io::{FromRawFd};
use std::path::Path;

use criterion::profiler::Profiler;

use perf_event_open_sys as sys;

pub struct EnergyProfiler {
    fd: c_int,
}

impl EnergyProfiler {
    pub fn new() -> Self {
        EnergyProfiler { fd: -1 }
    }
}

/*
 * Implementation inspired by https://web.eece.maine.edu/~vweaver/projects/rapl/rapl-read.c
 * Read statistics from perf events

 */
impl Profiler for EnergyProfiler {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        // Construct a zero-filled `perf_event_attr`.
        let mut attrs = sys::bindings::perf_event_attr::default();

        // Populate the fields we need.
        attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;
        attrs.type_ = 19 as u32; //TODO make dynamic
        attrs.config = 0x03 as u64; // Decide on the attribute to read
        // attrs.read_format = PERF_FORMAT_GROUP as u64;
        // attrs.set_disabled(1); // Start disabled to attach the breakpoint
        // struct perf_event_attr attr;
        // struct f_owner_ex owner;
        // int fd, c;
        // let mut bp_attrs = sys::bindings::perf_event_attr::default();
        //
        // // memset(&attr, 0, sizeof(attr));
        // bp_attrs.size = std::mem::size_of::<sys::bindings::perf_event_attr>() as u32;
        // bp_attrs.type_ = PERF_TYPE_BREAKPOINT;
        // bp_attrs.bp_type = HW_BREAKPOINT_X;
        // bp_attrs.__bindgen_anon_3.bp_addr  = f as u64;
        // bp_attrs.__bindgen_anon_4.bp_len = std::mem::size_of::<c_long>() as u64;


        // Make the system call.
        //sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'
        self.fd = unsafe {
            // PID = 0, CPU = -1 -> measure calling process on any cpus
            // pid -1 cpu 0 works // Measure everything on cpu 1
            sys::perf_event_open(&mut attrs, -1, 1, -1, 0)
        };

        if self.fd < 0 {
            panic!("Could not open perf_event: {}", std::io::Error::last_os_error())
        }
    }

    fn stop_profiling(&mut self, benchmark_id: &str, benchmark_dir: &Path) {
        let mut buf = [0_u8; 8];
        { // File scope
            let mut f = unsafe { File::from_raw_fd(self.fd) };
            f.read_exact(&mut buf).ok();
        }
        //
        let i1 = (i64::from_le_bytes(buf) as f64) * 2.32831e-10;

        std::fs::create_dir_all(benchmark_dir).unwrap();
        let mut file = OpenOptions::new().create(true).append(true).open(benchmark_dir.join(Path::new(&benchmark_id.replace('/', "_")))).unwrap();

        // TODO safe convert
        file.write_fmt(format_args!("{}\n", i1)).unwrap();
        // println!("{} cost {:6}J to run", benchmark_id, f64::from(i1 as u32) * 2.32831e-10);

    }
}