use std::arch::asm;
use std::fs::OpenOptions;
use std::os::unix::fs::FileExt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use criterion::profiler::Profiler;
use libc::{__errno_location, iopl};

struct MSR {
    string: &'static str,
    numeric: u64,
}

static MSR_ENERGY_STATUS: MSR = MSR { string: "0xc001029a", numeric: 0xc001029au64 };
static MSR_POWER_UNIT: MSR = MSR { string: "0xC001_0299", numeric: 0xC001_0299u64 };

struct EnergyProfiler {}

impl Profiler for EnergyProfiler {
    fn start_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {}

    fn stop_profiling(&mut self, _benchmark_id: &str, _benchmark_dir: &Path) {

    }
}

#[allow(unused)]
fn cmd_rdmsr(msr: &MSR) -> Result<u64, String> {
    let out = Command::new("rdmsr").arg("-d").arg(msr.string).output().unwrap();
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

fn read_single_core_msr_file(msr: &MSR, cpu: usize) -> Result<u64, String> {
    let msr_file_name = format!("/dev/cpu/{cpu}/msr");


    let file = match OpenOptions::new().read(true).write(false).open(&msr_file_name) {
        Ok(file) => file,
        Err(err) => return Err(String::from(format!("energy-profiler: could not read file {msr_file_name}: {err}"))),
    };

    let mut buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

    match file.read_at(&mut buf, msr.numeric) {
        Ok(size) => { if size != 8 { return Err(String::from("energy-profiler: : read wrong amount of data")); } }
        Err(err) => return Err(String::from(format!("energy-profiler: : CPU {cpu} cannot read MSR 0xc001029a: {err}")))
    }

    Ok(u64::from_le_bytes(buf))
}

#[allow(unused)]
/*
Loads the contents of a 64-bit model-specific register (MSR) specified in the ECX register into
registers EDX:EAX. The EDX register receives the high-order 32 bits and the EAX register receives
the low order bits.
 */
fn read_single_core_msr(msr: &MSR, cpu: i32) {
    println!("Reading single core {cpu}");
    let msr_num: u64 = msr.numeric; // Core Energy Status MSR
    let _hi: u64;
    let _lo: u64;
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
        in("rcx") msr_num,
        // out("edx") hi,
        // out("eax") lo,
        );
    }

    // println!("{} {}", hi, lo);
}

pub(crate) fn read_raw_energy(cpu: usize) -> u64 {
    read_single_core_msr_file(&MSR_ENERGY_STATUS, cpu).unwrap()
}

pub(crate) fn read_power_unit(cpu: usize) -> f64 {
    let i = read_single_core_msr_file(&MSR_POWER_UNIT, cpu).unwrap();

    // The Energy Status Unit (ESU) is contained in bits 12:8
    let bits: u8 = (i << (63 - 12) >> (63 - 5)) as u8;

    // The value is 1/2^ESU
    0.5f64.powi(bits as i32)
}


#[test]
fn test_asm_rdmsr() {
    println!("Testing");
    read_single_core_msr(&MSR_ENERGY_STATUS, 0);
}

#[test]
fn test_file_msr() {
    read_single_core_msr_file(&MSR_ENERGY_STATUS, 0).unwrap();
    assert!(read_single_core_msr_file(&MSR_ENERGY_STATUS, 0).is_ok())
}