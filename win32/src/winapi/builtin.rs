#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#[doc = r" Generated code, do not edit."]
use crate::{
    machine::Machine,
    winapi::{self, stack_args::*, types::*},
};
pub struct Symbol {
    pub name: &'static str,
    pub ordinal: Option<usize>,
    pub func: fn(&mut Machine),
    pub stack_consumed: fn() -> u32,
}
pub struct BuiltinDLL {
    pub file_name: &'static str,
    pub exports: &'static [Symbol],
}
pub mod bass {
    use super::*;
    use winapi::bass::*;
    pub fn BASS_Init(machine: &mut Machine) {
        todo!()
    }
    pub fn BASS_MusicLoad(machine: &mut Machine) {
        todo!()
    }
    pub fn BASS_Start(machine: &mut Machine) {
        todo!()
    }
    pub fn BASS_MusicPlay(machine: &mut Machine) {
        todo!()
    }
    pub fn BASS_ChannelGetPosition(machine: &mut Machine) {
        todo!()
    }
    const EXPORTS: [Symbol; 5usize] = [
        Symbol {
            name: "BASS_Init",
            ordinal: None,
            func: BASS_Init,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "BASS_MusicLoad",
            ordinal: None,
            func: BASS_MusicLoad,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "BASS_Start",
            ordinal: None,
            func: BASS_Start,
            stack_consumed: || 0,
        },
        Symbol {
            name: "BASS_MusicPlay",
            ordinal: None,
            func: BASS_MusicPlay,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "BASS_ChannelGetPosition",
            ordinal: None,
            func: BASS_ChannelGetPosition,
            stack_consumed: || <u32>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "bass.dll",
        exports: &EXPORTS,
    };
}
pub mod ddraw {
    use super::*;
    use winapi::ddraw::*;
    pub fn DirectDrawCreate(machine: &mut Machine) {
        todo!()
    }
    pub fn DirectDrawCreateEx(machine: &mut Machine) {
        todo!()
    }
    const EXPORTS: [Symbol; 2usize] = [
        Symbol {
            name: "DirectDrawCreate",
            ordinal: None,
            func: DirectDrawCreate,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "DirectDrawCreateEx",
            ordinal: None,
            func: DirectDrawCreateEx,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ddraw.dll",
        exports: &EXPORTS,
    };
}
pub mod dsound {
    use super::*;
    use winapi::dsound::*;
    pub fn DirectSoundCreate(machine: &mut Machine) {
        todo!()
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        name: "DirectSoundCreate",
        ordinal: Some(1usize),
        func: DirectSoundCreate,
        stack_consumed: || {
            <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
        },
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "dsound.dll",
        exports: &EXPORTS,
    };
}
pub mod gdi32 {
    use super::*;
    use winapi::gdi32::*;
    pub fn GetStockObject(machine: &mut Machine) {
        todo!()
    }
    pub fn SelectObject(machine: &mut Machine) {
        todo!()
    }
    pub fn GetObjectA(machine: &mut Machine) {
        todo!()
    }
    pub fn CreateCompatibleDC(machine: &mut Machine) {
        todo!()
    }
    pub fn DeleteDC(machine: &mut Machine) {
        todo!()
    }
    pub fn BitBlt(machine: &mut Machine) {
        todo!()
    }
    pub fn StretchBlt(machine: &mut Machine) {
        todo!()
    }
    const EXPORTS: [Symbol; 7usize] = [
        Symbol {
            name: "GetStockObject",
            ordinal: None,
            func: GetStockObject,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "SelectObject",
            ordinal: None,
            func: SelectObject,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetObjectA",
            ordinal: None,
            func: GetObjectA,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "CreateCompatibleDC",
            ordinal: None,
            func: CreateCompatibleDC,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "DeleteDC",
            ordinal: None,
            func: DeleteDC,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "BitBlt",
            ordinal: None,
            func: BitBlt,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "StretchBlt",
            ordinal: None,
            func: StretchBlt,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "gdi32.dll",
        exports: &EXPORTS,
    };
}
pub mod kernel32 {
    use super::*;
    use winapi::kernel32::*;
    pub fn GetModuleHandleA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetModuleHandleW(machine: &mut Machine) {
        todo!()
    }
    pub fn GetModuleHandleExW(machine: &mut Machine) {
        todo!()
    }
    pub fn LoadLibraryA(machine: &mut Machine) {
        todo!()
    }
    pub fn LoadLibraryExW(machine: &mut Machine) {
        todo!()
    }
    pub fn GetProcAddress(machine: &mut Machine) {
        todo!()
    }
    pub fn GetStdHandle(machine: &mut Machine) {
        todo!()
    }
    pub fn CreateFileA(machine: &mut Machine) {
        todo!()
    }
    pub fn CreateFileW(machine: &mut Machine) {
        todo!()
    }
    pub fn GetFileType(machine: &mut Machine) {
        todo!()
    }
    pub fn SetFilePointer(machine: &mut Machine) {
        todo!()
    }
    pub fn ReadFile(machine: &mut Machine) {
        todo!()
    }
    pub fn WriteFile(machine: &mut Machine) {
        todo!()
    }
    pub fn HeapAlloc(machine: &mut Machine) {
        todo!()
    }
    pub fn HeapFree(machine: &mut Machine) {
        todo!()
    }
    pub fn HeapSize(machine: &mut Machine) {
        todo!()
    }
    pub fn HeapReAlloc(machine: &mut Machine) {
        todo!()
    }
    pub fn HeapCreate(machine: &mut Machine) {
        todo!()
    }
    pub fn HeapDestroy(machine: &mut Machine) {
        todo!()
    }
    pub fn VirtualAlloc(machine: &mut Machine) {
        todo!()
    }
    pub fn VirtualFree(machine: &mut Machine) {
        todo!()
    }
    pub fn IsBadReadPtr(machine: &mut Machine) {
        todo!()
    }
    pub fn IsBadWritePtr(machine: &mut Machine) {
        todo!()
    }
    pub fn SetLastError(machine: &mut Machine) {
        todo!()
    }
    pub fn GetLastError(machine: &mut Machine) {
        todo!()
    }
    pub fn ExitProcess(machine: &mut Machine) {
        todo!()
    }
    pub fn GetACP(machine: &mut Machine) {
        todo!()
    }
    pub fn IsValidCodePage(machine: &mut Machine) {
        todo!()
    }
    pub fn GetCPInfo(machine: &mut Machine) {
        todo!()
    }
    pub fn GetCommandLineA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetCommandLineW(machine: &mut Machine) {
        todo!()
    }
    pub fn GetEnvironmentStrings(machine: &mut Machine) {
        todo!()
    }
    pub fn FreeEnvironmentStringsA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetEnvironmentStringsW(machine: &mut Machine) {
        todo!()
    }
    pub fn GetEnvironmentVariableA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetModuleFileNameA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetModuleFileNameW(machine: &mut Machine) {
        todo!()
    }
    pub fn GetStartupInfoA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetStartupInfoW(machine: &mut Machine) {
        todo!()
    }
    pub fn IsProcessorFeaturePresent(machine: &mut Machine) {
        todo!()
    }
    pub fn IsDebuggerPresent(machine: &mut Machine) {
        todo!()
    }
    pub fn GetCurrentProcessId(machine: &mut Machine) {
        todo!()
    }
    pub fn GetTickCount(machine: &mut Machine) {
        todo!()
    }
    pub fn QueryPerformanceCounter(machine: &mut Machine) {
        todo!()
    }
    pub fn QueryPerformanceFrequency(machine: &mut Machine) {
        todo!()
    }
    pub fn GetSystemTimeAsFileTime(machine: &mut Machine) {
        todo!()
    }
    pub fn GetVersion(machine: &mut Machine) {
        todo!()
    }
    pub fn GetVersionExA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetProcessHeap(machine: &mut Machine) {
        todo!()
    }
    pub fn SetHandleCount(machine: &mut Machine) {
        todo!()
    }
    pub fn OutputDebugStringA(machine: &mut Machine) {
        todo!()
    }
    pub fn InitializeCriticalSectionAndSpinCount(machine: &mut Machine) {
        todo!()
    }
    pub fn DeleteCriticalSection(machine: &mut Machine) {
        todo!()
    }
    pub fn EnterCriticalSection(machine: &mut Machine) {
        todo!()
    }
    pub fn LeaveCriticalSection(machine: &mut Machine) {
        todo!()
    }
    pub fn SetUnhandledExceptionFilter(machine: &mut Machine) {
        todo!()
    }
    pub fn UnhandledExceptionFilter(machine: &mut Machine) {
        todo!()
    }
    pub fn NtCurrentTeb(machine: &mut Machine) {
        todo!()
    }
    pub fn InitializeSListHead(machine: &mut Machine) {
        todo!()
    }
    pub fn MultiByteToWideChar(machine: &mut Machine) {
        todo!()
    }
    pub fn WriteConsoleW(machine: &mut Machine) {
        todo!()
    }
    pub fn GetCurrentThreadId(machine: &mut Machine) {
        todo!()
    }
    pub fn TlsAlloc(machine: &mut Machine) {
        todo!()
    }
    pub fn TlsFree(machine: &mut Machine) {
        todo!()
    }
    pub fn TlsSetValue(machine: &mut Machine) {
        todo!()
    }
    pub fn TlsGetValue(machine: &mut Machine) {
        todo!()
    }
    pub fn CreateThread(machine: &mut Machine) {
        todo!()
    }
    pub fn SetThreadPriority(machine: &mut Machine) {
        todo!()
    }
    pub fn InterlockedIncrement(machine: &mut Machine) {
        todo!()
    }
    const EXPORTS: [Symbol; 69usize] = [
        Symbol {
            name: "GetModuleHandleA",
            ordinal: None,
            func: GetModuleHandleA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleHandleW",
            ordinal: None,
            func: GetModuleHandleW,
            stack_consumed: || <Option<Str16>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleHandleExW",
            ordinal: None,
            func: GetModuleHandleExW,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <Option<Str16>>::stack_consumed()
                    + <Option<&mut HMODULE>>::stack_consumed()
            },
        },
        Symbol {
            name: "LoadLibraryA",
            ordinal: None,
            func: LoadLibraryA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "LoadLibraryExW",
            ordinal: None,
            func: LoadLibraryExW,
            stack_consumed: || {
                <Option<Str16>>::stack_consumed()
                    + <HFILE>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetProcAddress",
            ordinal: None,
            func: GetProcAddress,
            stack_consumed: || <HMODULE>::stack_consumed() + <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "GetStdHandle",
            ordinal: None,
            func: GetStdHandle,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "CreateFileA",
            ordinal: None,
            func: CreateFileA,
            stack_consumed: || {
                <Option<&str>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<CreationDisposition, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HFILE>::stack_consumed()
            },
        },
        Symbol {
            name: "CreateFileW",
            ordinal: None,
            func: CreateFileW,
            stack_consumed: || {
                <Option<Str16>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<CreationDisposition, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HFILE>::stack_consumed()
            },
        },
        Symbol {
            name: "GetFileType",
            ordinal: None,
            func: GetFileType,
            stack_consumed: || <HFILE>::stack_consumed(),
        },
        Symbol {
            name: "SetFilePointer",
            ordinal: None,
            func: SetFilePointer,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "ReadFile",
            ordinal: None,
            func: ReadFile,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&mut [u8]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "WriteFile",
            ordinal: None,
            func: WriteFile,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&[u8]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapAlloc",
            ordinal: None,
            func: HeapAlloc,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapFree",
            ordinal: None,
            func: HeapFree,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapSize",
            ordinal: None,
            func: HeapSize,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapReAlloc",
            ordinal: None,
            func: HeapReAlloc,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapCreate",
            ordinal: None,
            func: HeapCreate,
            stack_consumed: || {
                <Result<HeapCreateFlags, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "HeapDestroy",
            ordinal: None,
            func: HeapDestroy,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "VirtualAlloc",
            ordinal: None,
            func: VirtualAlloc,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "VirtualFree",
            ordinal: None,
            func: VirtualFree,
            stack_consumed: || {
                <u32>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "IsBadReadPtr",
            ordinal: None,
            func: IsBadReadPtr,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "IsBadWritePtr",
            ordinal: None,
            func: IsBadWritePtr,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetLastError",
            ordinal: None,
            func: SetLastError,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetLastError",
            ordinal: None,
            func: GetLastError,
            stack_consumed: || 0,
        },
        Symbol {
            name: "ExitProcess",
            ordinal: None,
            func: ExitProcess,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetACP",
            ordinal: None,
            func: GetACP,
            stack_consumed: || 0,
        },
        Symbol {
            name: "IsValidCodePage",
            ordinal: None,
            func: IsValidCodePage,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetCPInfo",
            ordinal: None,
            func: GetCPInfo,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetCommandLineA",
            ordinal: None,
            func: GetCommandLineA,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetCommandLineW",
            ordinal: None,
            func: GetCommandLineW,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetEnvironmentStrings",
            ordinal: None,
            func: GetEnvironmentStrings,
            stack_consumed: || 0,
        },
        Symbol {
            name: "FreeEnvironmentStringsA",
            ordinal: None,
            func: FreeEnvironmentStringsA,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetEnvironmentStringsW",
            ordinal: None,
            func: GetEnvironmentStringsW,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetEnvironmentVariableA",
            ordinal: None,
            func: GetEnvironmentVariableA,
            stack_consumed: || {
                <Option<&str>>::stack_consumed() + <Option<&mut [u8]>>::stack_consumed()
            },
        },
        Symbol {
            name: "GetModuleFileNameA",
            ordinal: None,
            func: GetModuleFileNameA,
            stack_consumed: || <HMODULE>::stack_consumed() + <Option<&mut [u8]>>::stack_consumed(),
        },
        Symbol {
            name: "GetModuleFileNameW",
            ordinal: None,
            func: GetModuleFileNameW,
            stack_consumed: || {
                <HMODULE>::stack_consumed() + <u32>::stack_consumed() + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetStartupInfoA",
            ordinal: None,
            func: GetStartupInfoA,
            stack_consumed: || <Option<&mut STARTUPINFOA>>::stack_consumed(),
        },
        Symbol {
            name: "GetStartupInfoW",
            ordinal: None,
            func: GetStartupInfoW,
            stack_consumed: || <Option<&mut STARTUPINFOA>>::stack_consumed(),
        },
        Symbol {
            name: "IsProcessorFeaturePresent",
            ordinal: None,
            func: IsProcessorFeaturePresent,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "IsDebuggerPresent",
            ordinal: None,
            func: IsDebuggerPresent,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetCurrentProcessId",
            ordinal: None,
            func: GetCurrentProcessId,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetTickCount",
            ordinal: None,
            func: GetTickCount,
            stack_consumed: || 0,
        },
        Symbol {
            name: "QueryPerformanceCounter",
            ordinal: None,
            func: QueryPerformanceCounter,
            stack_consumed: || <Option<&mut LARGE_INTEGER>>::stack_consumed(),
        },
        Symbol {
            name: "QueryPerformanceFrequency",
            ordinal: None,
            func: QueryPerformanceFrequency,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "GetSystemTimeAsFileTime",
            ordinal: None,
            func: GetSystemTimeAsFileTime,
            stack_consumed: || <Option<&mut FILETIME>>::stack_consumed(),
        },
        Symbol {
            name: "GetVersion",
            ordinal: None,
            func: GetVersion,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetVersionExA",
            ordinal: None,
            func: GetVersionExA,
            stack_consumed: || <Option<&mut OSVERSIONINFO>>::stack_consumed(),
        },
        Symbol {
            name: "GetProcessHeap",
            ordinal: None,
            func: GetProcessHeap,
            stack_consumed: || 0,
        },
        Symbol {
            name: "SetHandleCount",
            ordinal: None,
            func: SetHandleCount,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "OutputDebugStringA",
            ordinal: None,
            func: OutputDebugStringA,
            stack_consumed: || <Option<&str>>::stack_consumed(),
        },
        Symbol {
            name: "InitializeCriticalSectionAndSpinCount",
            ordinal: None,
            func: InitializeCriticalSectionAndSpinCount,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "DeleteCriticalSection",
            ordinal: None,
            func: DeleteCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "EnterCriticalSection",
            ordinal: None,
            func: EnterCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "LeaveCriticalSection",
            ordinal: None,
            func: LeaveCriticalSection,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetUnhandledExceptionFilter",
            ordinal: None,
            func: SetUnhandledExceptionFilter,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "UnhandledExceptionFilter",
            ordinal: None,
            func: UnhandledExceptionFilter,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "NtCurrentTeb",
            ordinal: None,
            func: NtCurrentTeb,
            stack_consumed: || 0,
        },
        Symbol {
            name: "InitializeSListHead",
            ordinal: None,
            func: InitializeSListHead,
            stack_consumed: || <Option<&mut SLIST_HEADER>>::stack_consumed(),
        },
        Symbol {
            name: "MultiByteToWideChar",
            ordinal: None,
            func: MultiByteToWideChar,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <i32>::stack_consumed()
                    + <Option<&mut [u16]>>::stack_consumed()
            },
        },
        Symbol {
            name: "WriteConsoleW",
            ordinal: None,
            func: WriteConsoleW,
            stack_consumed: || {
                <HFILE>::stack_consumed()
                    + <Option<&[u16]>>::stack_consumed()
                    + <Option<&mut u32>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetCurrentThreadId",
            ordinal: None,
            func: GetCurrentThreadId,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TlsAlloc",
            ordinal: None,
            func: TlsAlloc,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TlsFree",
            ordinal: None,
            func: TlsFree,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "TlsSetValue",
            ordinal: None,
            func: TlsSetValue,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "TlsGetValue",
            ordinal: None,
            func: TlsGetValue,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "CreateThread",
            ordinal: None,
            func: CreateThread,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "SetThreadPriority",
            ordinal: None,
            func: SetThreadPriority,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "InterlockedIncrement",
            ordinal: None,
            func: InterlockedIncrement,
            stack_consumed: || <Option<&mut u32>>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "kernel32.dll",
        exports: &EXPORTS,
    };
}
pub mod ole32 {
    use super::*;
    use winapi::ole32::*;
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "ole32.dll",
        exports: &EXPORTS,
    };
}
pub mod oleaut32 {
    use super::*;
    use winapi::oleaut32::*;
    const EXPORTS: [Symbol; 0usize] = [];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "oleaut32.dll",
        exports: &EXPORTS,
    };
}
pub mod user32 {
    use super::*;
    use winapi::user32::*;
    pub fn RegisterClassA(machine: &mut Machine) {
        todo!()
    }
    pub fn RegisterClassExA(machine: &mut Machine) {
        todo!()
    }
    pub fn CreateWindowExA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetForegroundWindow(machine: &mut Machine) {
        todo!()
    }
    pub fn GetActiveWindow(machine: &mut Machine) {
        todo!()
    }
    pub fn GetLastActivePopup(machine: &mut Machine) {
        todo!()
    }
    pub fn UpdateWindow(machine: &mut Machine) {
        todo!()
    }
    pub fn ShowWindow(machine: &mut Machine) {
        todo!()
    }
    pub fn SetFocus(machine: &mut Machine) {
        todo!()
    }
    pub fn SetCursor(machine: &mut Machine) {
        todo!()
    }
    pub fn MessageBoxA(machine: &mut Machine) {
        todo!()
    }
    pub fn DialogBoxParamA(machine: &mut Machine) {
        todo!()
    }
    pub fn PeekMessageA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetMessageA(machine: &mut Machine) {
        todo!()
    }
    pub fn WaitMessage(machine: &mut Machine) {
        todo!()
    }
    pub fn TranslateMessage(machine: &mut Machine) {
        todo!()
    }
    pub fn DispatchMessageA(machine: &mut Machine) {
        todo!()
    }
    pub fn DefWindowProcA(machine: &mut Machine) {
        todo!()
    }
    pub fn LoadIconA(machine: &mut Machine) {
        todo!()
    }
    pub fn LoadCursorA(machine: &mut Machine) {
        todo!()
    }
    pub fn ShowCursor(machine: &mut Machine) {
        todo!()
    }
    pub fn LoadImageA(machine: &mut Machine) {
        todo!()
    }
    pub fn GetSystemMetrics(machine: &mut Machine) {
        todo!()
    }
    const EXPORTS: [Symbol; 23usize] = [
        Symbol {
            name: "RegisterClassA",
            ordinal: None,
            func: RegisterClassA,
            stack_consumed: || <Option<&WNDCLASSA>>::stack_consumed(),
        },
        Symbol {
            name: "RegisterClassExA",
            ordinal: None,
            func: RegisterClassExA,
            stack_consumed: || <Option<&WNDCLASSEXA>>::stack_consumed(),
        },
        Symbol {
            name: "CreateWindowExA",
            ordinal: None,
            func: CreateWindowExA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <Result<WindowStyle, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetForegroundWindow",
            ordinal: None,
            func: GetForegroundWindow,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetActiveWindow",
            ordinal: None,
            func: GetActiveWindow,
            stack_consumed: || 0,
        },
        Symbol {
            name: "GetLastActivePopup",
            ordinal: None,
            func: GetLastActivePopup,
            stack_consumed: || 0,
        },
        Symbol {
            name: "UpdateWindow",
            ordinal: None,
            func: UpdateWindow,
            stack_consumed: || <HWND>::stack_consumed(),
        },
        Symbol {
            name: "ShowWindow",
            ordinal: None,
            func: ShowWindow,
            stack_consumed: || <HWND>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "SetFocus",
            ordinal: None,
            func: SetFocus,
            stack_consumed: || <HWND>::stack_consumed(),
        },
        Symbol {
            name: "SetCursor",
            ordinal: None,
            func: SetCursor,
            stack_consumed: || <u32>::stack_consumed(),
        },
        Symbol {
            name: "MessageBoxA",
            ordinal: None,
            func: MessageBoxA,
            stack_consumed: || {
                <HWND>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <Option<&str>>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "DialogBoxParamA",
            ordinal: None,
            func: DialogBoxParamA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "PeekMessageA",
            ordinal: None,
            func: PeekMessageA,
            stack_consumed: || {
                <Option<&mut MSG>>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <Result<RemoveMsg, u32>>::stack_consumed()
            },
        },
        Symbol {
            name: "GetMessageA",
            ordinal: None,
            func: GetMessageA,
            stack_consumed: || {
                <Option<&mut MSG>>::stack_consumed()
                    + <HWND>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "WaitMessage",
            ordinal: None,
            func: WaitMessage,
            stack_consumed: || 0,
        },
        Symbol {
            name: "TranslateMessage",
            ordinal: None,
            func: TranslateMessage,
            stack_consumed: || <Option<&MSG>>::stack_consumed(),
        },
        Symbol {
            name: "DispatchMessageA",
            ordinal: None,
            func: DispatchMessageA,
            stack_consumed: || <Option<&MSG>>::stack_consumed(),
        },
        Symbol {
            name: "DefWindowProcA",
            ordinal: None,
            func: DefWindowProcA,
            stack_consumed: || {
                <HWND>::stack_consumed()
                    + <Result<WM, u32>>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "LoadIconA",
            ordinal: None,
            func: LoadIconA,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "LoadCursorA",
            ordinal: None,
            func: LoadCursorA,
            stack_consumed: || <u32>::stack_consumed() + <u32>::stack_consumed(),
        },
        Symbol {
            name: "ShowCursor",
            ordinal: None,
            func: ShowCursor,
            stack_consumed: || <bool>::stack_consumed(),
        },
        Symbol {
            name: "LoadImageA",
            ordinal: None,
            func: LoadImageA,
            stack_consumed: || {
                <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
                    + <u32>::stack_consumed()
            },
        },
        Symbol {
            name: "GetSystemMetrics",
            ordinal: None,
            func: GetSystemMetrics,
            stack_consumed: || <u32>::stack_consumed(),
        },
    ];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "user32.dll",
        exports: &EXPORTS,
    };
}
pub mod winmm {
    use super::*;
    use winapi::winmm::*;
    pub fn timeSetEvent(machine: &mut Machine) {
        todo!()
    }
    const EXPORTS: [Symbol; 1usize] = [Symbol {
        name: "timeSetEvent",
        ordinal: None,
        func: timeSetEvent,
        stack_consumed: || {
            <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
                + <u32>::stack_consumed()
        },
    }];
    pub const DLL: BuiltinDLL = BuiltinDLL {
        file_name: "winmm.dll",
        exports: &EXPORTS,
    };
}
