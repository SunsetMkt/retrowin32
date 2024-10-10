//! The C runtime library.  This module is also the implementation of msvcrt.dll.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::Machine;
use memory::Extensions;

#[win32_derive::dllexport(cdecl)]
pub async fn _initterm(machine: &mut Machine, start: u32, end: u32) -> u32 {
    if (end - start) % 4 != 0 {
        panic!("unaligned _initterm");
    }
    let slice = machine.mem().sub32(start, end - start).to_vec();
    for addr in slice.into_iter_pod::<u32>() {
        if addr != 0 {
            machine.call_x86(addr, vec![]).await;
        }
    }
    0
}

#[win32_derive::dllexport(cdecl)]
pub async fn _initterm_e(machine: &mut Machine, start: u32, end: u32) -> u32 {
    if (end - start) % 4 != 0 {
        panic!("unaligned _initterm_e");
    }
    let slice = machine.mem().sub32(start, end - start).to_vec();
    for addr in slice.into_iter_pod::<u32>() {
        if addr != 0 {
            let err = machine.call_x86(addr, vec![]).await;
            if err != 0 {
                return err;
            }
        }
    }
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _get_initial_narrow_environment(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argv(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p___argc(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p__fmode(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __p__commode(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_app_type(_machine: &mut Machine, _app_type: u32) -> u32 {
    0
}

// Not a typo!  Both __set_app_type and _set_app_type are defined and even have different documentation.
#[win32_derive::dllexport(cdecl)]
pub fn __set_app_type(_machine: &mut Machine, _app_type: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_fmode(_machine: &mut Machine, _mode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _crt_atexit(_machine: &mut Machine, _function: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _configure_narrow_argv(_machine: &mut Machine, _mode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn exit(machine: &mut Machine, status: u32) {
    machine.exit(status);
}

#[win32_derive::dllexport(cdecl)]
pub fn _lock(_machine: &mut Machine, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _unlock(_machine: &mut Machine, locknum: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __dllonexit(_machine: &mut Machine, func: u32, d: u32, f: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _controlfp(_machine: &mut Machine, _new: u32, _mask: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _controlfp_s(
    _machine: &mut Machine,
    _currentControl: u32,
    _newControl: u32,
    _mask: u32,
) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _configthreadlocale(_machine: &mut Machine, per_thread_locale_type: i32) -> i32 {
    const _ENABLE_PER_THREAD_LOCALE: i32 = 1;
    const _DISABLE_PER_THREAD_LOCALE: i32 = 2;

    match per_thread_locale_type {
        0 => _DISABLE_PER_THREAD_LOCALE, // retrieve current setting
        _ENABLE_PER_THREAD_LOCALE | _DISABLE_PER_THREAD_LOCALE => {
            // ignore setting
            _DISABLE_PER_THREAD_LOCALE // return previous setting
        }
        _ => -1,
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn _initialize_narrow_environment(_machine: &mut Machine) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn _set_new_mode(_machine: &mut Machine, newhandlermode: u32) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn __getmainargs(
    _machine: &mut Machine,
    argc: Option<&mut u32>,
    argv: Option<&mut u32>,
    env: Option<&mut u32>,
    doWildCard: u32,
    startInfo: u32,
) -> u32 {
    0
}

#[win32_derive::dllexport(cdecl)]
pub fn malloc(machine: &mut Machine, size: u32) -> u32 {
    let heap = machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory); // lazy init process_heap
    heap.alloc(machine.emu.memory.mem(), size)
}

#[win32_derive::dllexport(cdecl)]
pub fn free(machine: &mut Machine, ptr: u32) -> u32 {
    let heap = machine
        .state
        .kernel32
        .get_process_heap(&mut machine.emu.memory); // lazy init process_heap
    heap.free(machine.emu.memory.mem(), ptr);
    0
}

// MSDN: "Calling rand before any call to srand generates the same sequence as calling srand with seed passed as 1."
static mut RAND_STATE: u32 = 1;

#[win32_derive::dllexport(cdecl)]
pub fn srand(machine: &mut Machine, seed: u32) {
    unsafe {
        RAND_STATE = seed % (1 << 31);
    }
}

#[win32_derive::dllexport(cdecl)]
pub fn rand(machine: &mut Machine) -> u32 {
    // https://en.wikipedia.org/wiki/Linear_congruential_generator
    unsafe {
        RAND_STATE = ((RAND_STATE.wrapping_mul(134775813)).wrapping_add(1)) % (1 << 31);
        RAND_STATE
    }
}

fn time64(machine: &mut Machine, destTime: Option<&mut u64>) -> u32 {
    let time = machine.host.system_time().timestamp() as u64;
    if let Some(destTime) = destTime {
        *destTime = time;
    }

    // TODO: 64-bit return values go through edx:eax, which is not yet modeled in the shims
    // machinery, so we only return 32 bits here.
    // Thankfully 32-bit time_t only overflows in 2038 anyway.
    time as u32
}

#[win32_derive::dllexport(cdecl)]
pub fn time(machine: &mut Machine, destTime: Option<&mut u64>) -> u32 {
    time64(machine, destTime)
}

#[win32_derive::dllexport(cdecl)]
pub fn _time64(machine: &mut Machine, destTime: Option<&mut u64>) -> u32 {
    time64(machine, destTime)
}

#[win32_derive::dllexport(cdecl)]
pub fn _XcptFilter(machine: &mut Machine, xcptnum: u32, pxcptinfoptrs: u32) -> u32 {
    todo!();
}

#[win32_derive::dllexport(cdecl)]
pub fn _except_handler3(
    _machine: &mut Machine,
    exception_record: u32,
    registration: u32,
    context: u32,
    dispatcher: u32,
) -> i32 {
    todo!();
}

#[win32_derive::dllexport(cdecl)]
pub fn __setusermatherr(machine: &mut Machine, pf: u32) {
    todo!();
}

#[win32_derive::dllexport]
pub const _adjust_fdiv: &'static str = "_adjust_fdiv";

#[win32_derive::dllexport]
pub const _acmdln: &'static str = "_acmdln";
