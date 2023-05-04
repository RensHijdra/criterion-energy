use std::arch::asm;
use std::fs::OpenOptions;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use criterion::profiler::Profiler;
use libc::{__errno_location, iopl};

struct EnergyProfiler {}

impl Profiler for EnergyProfiler {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {

    }

    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {
        todo!()
    }
}


pub fn cmd_rdmsr() -> Result<u64, String> {
    let out = Command::new("rdmsr").arg("-d").arg("0xc001029a").output().unwrap();
    if out.status.success() {
        let string = String::from_utf8(out.stdout).unwrap();
        match u64::from_str(&string) {
            Ok(u) => Ok(u),
            Err(_) => Err(String::from("ParseIntErr"))
        }
    } else {
        Err(String::from("rdmsr command failed with a non-zero error code"))
    }
}


pub fn read_single_core_msr_file(cpu: usize) -> Result<u64, String> {
    let msr_file_name = format!("/dev/cpu/{cpu}/msr");


    let file = match OpenOptions::new().read(true).write(false).open(&msr_file_name) {
        Ok(file) => file,
        Err(err) => return Err(String::from(format!("energy-profiler: could not read file {msr_file_name}: {err}"))),
    };

    let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    match file.read_at(&mut buf, 0xc001029a) {
        Ok(size) => { if size != 8 { return Err(String::from("energy-profiler: : read wrong amount of data")); } }
        Err(err) => return Err(String::from(format!("energy-profiler: : CPU {cpu} cannot read MSR 0xc001029a: {err}")))
    }

    Ok(u64::from_le_bytes(buf))
}

/*
Loads the contents of a 64-bit model-specific register (MSR) specified in the ECX register into
registers EDX:EAX. The EDX register receives the high-order 32 bits and the EAX register receives
the low order bits.
 */
pub fn read_single_core_msr(cpu: i32) {
    println!("Reading single core {cpu}");
    let msr: u64 = 3221291674; // Core Energy Status MSR
    let hi: u64;
    let lo: u64;
    unsafe {
        let i = iopl(3);
        println!("iopl ret: {i}");
        if i == -1 {
            println!("errno {}", __errno_location().read());
        }
        asm!(
        // "push rcx",
        // "push rdx",
        // "push rax",
        // "mov ecx, edx",
        "rdmsr",
        // "pop rax",
        // "pop rdx",
        // "pop rcx",
        in("rcx") 3221291674u64,
        // out("edx") hi,
        // out("eax") lo,
        );
    }

    // println!("{} {}", hi, lo);
}


#[test]
fn test_asm_rdmsr() {
    println!("Testing");
    read_single_core_msr(0);
}

#[test]
fn test_file_msr() {
    read_single_core_msr_file(0).unwrap();
    assert!(read_single_core_msr_file(0).is_ok())
}