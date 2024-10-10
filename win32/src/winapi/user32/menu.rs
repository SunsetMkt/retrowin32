use crate::{
    winapi::types::{HWND, RECT},
    Machine,
};

pub type HMENU = u32;

#[win32_derive::dllexport]
pub fn CreatePopupMenu(_machine: &mut Machine) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn CheckMenuItem(_machine: &mut Machine, hMenu: HMENU, uIDCheckItem: u32, uCheck: u32) -> u32 {
    0 // previous state: unchecked
}

#[win32_derive::dllexport]
pub fn EnableMenuItem(
    _machine: &mut Machine,
    hMenu: HMENU,
    uIDEnableItem: u32,
    uEnable: u32,
) -> bool {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetMenu(_machine: &mut Machine, hWnd: HWND) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn GetSubMenu(_machine: &mut Machine, hMenu: HMENU, nPos: i32) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn LoadMenuA(_machine: &mut Machine, hInstance: u32, lpMenuName: u32) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn SetMenu(_machine: &mut Machine, hWnd: HWND, hMenu: HMENU) -> bool {
    true // success
}

#[win32_derive::dllexport]
pub fn SetMenuItemInfoA(
    _machine: &mut Machine,
    hMenu: HMENU,
    item: u32,
    fByPosition: bool,
    lpmii: u32,
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn GetSystemMenu(_machine: &mut Machine, hWnd: HWND, bRevert: bool) -> HMENU {
    0 // null
}

#[win32_derive::dllexport]
pub fn AppendMenuA(
    _machine: &mut Machine,
    hMenu: HMENU,
    uFlags: u32,
    uIDNewItem: u32,
    lpNewItem: Option<&str>,
) -> bool {
    false // fail
}

#[win32_derive::dllexport]
pub fn GetMenuItemRect(
    _machine: &mut Machine,
    hWnd: HWND,
    hMenu: HMENU,
    uItem: u32,
    lprcItem: Option<&mut RECT>,
) -> bool {
    todo!();
}
