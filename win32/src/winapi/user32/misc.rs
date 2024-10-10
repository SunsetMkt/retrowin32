use crate::{
    str16::Str16,
    winapi::{stack_args::VarArgs, types::HWND},
    Machine,
};
use memory::{Extensions, ExtensionsMut};
use std::io::{Cursor, Write};

pub type HINSTANCE = u32;

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum SystemMetric {
    CXSCREEN = 0,
    CYSCREEN = 1,
    CYCAPTION = 4,
    CXBORDER = 5,
    CYBORDER = 6,
    CYMENU = 15,
    CXFRAME = 32,
    CYFRAME = 33,
    CXVIRTUALSCREEN = 78,
    CYVIRTUALSCREEN = 79,
}

#[win32_derive::dllexport]
pub fn GetSystemMetrics(_machine: &mut Machine, nIndex: Result<SystemMetric, u32>) -> u32 {
    let metric = match nIndex {
        Ok(metric) => metric,
        Err(val) => {
            log::error!("GetSystemMetrics({val}) => 0");
            return 0;
        }
    };
    match metric {
        SystemMetric::CXSCREEN => 640,
        SystemMetric::CYSCREEN => 480,
        SystemMetric::CYCAPTION => 19,
        SystemMetric::CXBORDER => 1,
        SystemMetric::CYBORDER => 1,
        SystemMetric::CYMENU => 19,
        SystemMetric::CXFRAME => 4,
        SystemMetric::CYFRAME => 4,
        SystemMetric::CXVIRTUALSCREEN => 640,
        SystemMetric::CYVIRTUALSCREEN => 480,
    }
}

#[win32_derive::dllexport]
pub fn GetSysColor(_machine: &mut Machine, nIndex: i32) -> u32 {
    todo!();
}

#[win32_derive::dllexport(cdecl)]
pub fn wsprintfA(machine: &mut Machine, buf: u32, fmt: Option<&str>, mut args: VarArgs) -> u32 {
    const BUF_LEN: u32 = 1024;
    let mem = machine.mem();
    let buf = mem.sub32_mut(buf, BUF_LEN);
    let mut out = Cursor::new(buf);

    fn read_number(c: u8) -> usize {
        // TODO: multiple digits, error handling, etc.
        assert!(c >= b'0' && c <= b'9');
        (c - b'0') as usize
    }

    let mut i = fmt.unwrap().bytes();
    while let Some(c) = i.next() {
        if c == b'%' {
            let mut c = i.next().unwrap();
            let mut width = 0;
            if c >= b'0' && c <= b'9' {
                width = read_number(c);
                c = i.next().unwrap();
            }
            let mut precision = 0;
            if c == b'.' {
                c = i.next().unwrap();
                precision = read_number(c);
                c = i.next().unwrap();
            }

            let mut long = false;
            if c == b'l' {
                long = true;
                c = i.next().unwrap();
            }
            _ = long; // currently ignored

            match c {
                b'u' => write!(
                    out,
                    "{:width$.precision$}",
                    args.pop::<u32>(mem),
                    width = width,
                    precision = precision
                )
                .unwrap(),
                b'd' => write!(
                    out,
                    "{:width$.precision$}",
                    args.pop::<i32>(mem),
                    width = width,
                    precision = precision
                )
                .unwrap(),
                b's' => {
                    let addr = args.pop::<u32>(mem);
                    let str = mem.slicez(addr);
                    write!(out, "{}", std::str::from_utf8(str).unwrap()).unwrap();
                }
                _ => todo!("format string character {:?}", c as char),
            }
        } else {
            out.write(&[c]).unwrap();
        }
    }
    out.write(&[0]).unwrap();
    // let len = out.position() as usize;
    // let buf = &out.into_inner()[..len];
    // log::info!("=> {}", std::str::from_utf8(buf).unwrap());
    // len as u32 - 1
    out.position() as u32 - 1
}

#[win32_derive::dllexport(cdecl)]
pub fn wsprintfW(machine: &mut Machine, buf: u32, fmt: Option<&Str16>, args: VarArgs) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetKeyState(_machine: &mut Machine, nVirtKey: u32) -> u32 {
    0
}

#[win32_derive::dllexport]
pub fn IsIconic(_machine: &mut Machine, hwnd: HWND) -> bool {
    false
}

#[win32_derive::dllexport]
pub fn WinHelpW(
    _machine: &mut Machine,
    hWndMain: HWND,
    lpszHelp: Option<&Str16>,
    uCommand: u32,
    dwData: u32,
) -> bool {
    todo!();
}

#[win32_derive::dllexport]
pub fn SetCursorPos(_machine: &mut Machine, x: i32, y: i32) -> bool {
    todo!();
}
