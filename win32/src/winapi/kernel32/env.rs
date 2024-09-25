use crate::{str16::Str16, winapi::stack_args::ArrayWithSize, Machine};

const TRACE_CONTEXT: &'static str = "kernel32/env";

#[win32_derive::dllexport]
pub fn GetEnvironmentStrings(machine: &mut Machine) -> u32 {
    machine.state.kernel32.env
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsA(_machine: &mut Machine, _penv: u32) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentStringsW(_machine: &mut Machine) -> u32 {
    // CRT startup appears to fallback on non-W version of this if it returns null.
    0
}

#[win32_derive::dllexport]
pub fn FreeEnvironmentStringsW(_machine: &mut Machine) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    buf: ArrayWithSize<u8>,
) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn GetEnvironmentVariableW(
    _machine: &mut Machine,
    name: Option<&Str16>,
    buf: ArrayWithSize<u16>,
) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn SetEnvironmentVariableA(
    _machine: &mut Machine,
    name: Option<&str>,
    value: Option<&str>,
) -> bool {
    true
}