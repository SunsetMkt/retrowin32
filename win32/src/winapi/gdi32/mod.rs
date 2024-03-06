#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bitmap;
mod dc;
mod draw;
mod text;
pub use bitmap::*;
pub use dc::*;
pub use draw::*;
pub use text::*;

pub use super::bitmap::BITMAPINFOHEADER;
use super::{bitmap::Bitmap, handle::Handles, kernel32, types::*};
use crate::machine::Machine;

const TRACE_CONTEXT: &'static str = "gdi32";

/// GDI Object, as identified by HANDLEs.
#[derive(Debug)]
pub enum Object {
    Brush(Brush),
    Bitmap(Bitmap),
    Pen(Pen),
}

pub type HGDIOBJ = HANDLE<Object>;
impl HGDIOBJ {
    /// Some Windows APIs use low values of GDI objects as known system constants,
    /// so start the handles higher.
    pub fn lowest_value() -> u32 {
        0x100
    }
}

pub struct State {
    pub dcs: Handles<HDC, DC>,
    pub desktop_dc: HDC,
    pub objects: Handles<HGDIOBJ, Object>,
}

impl Default for State {
    fn default() -> Self {
        let mut dcs: Handles<HDC, DC> = Default::default();
        let desktop_dc = dcs.reserve();
        State {
            dcs,
            desktop_dc,
            objects: Handles::new(HGDIOBJ::lowest_value()),
        }
    }
}

pub const CLR_INVALID: u32 = 0xFFFF_FFFF;

#[derive(Debug, win32_derive::TryFromEnum)]
pub enum GetStockObjectArg {
    WHITE_BRUSH = 0,
    LTGRAY_BRUSH = 1,
    GRAY_BRUSH = 2,
    DKGRAY_BRUSH = 3,
    BLACK_BRUSH = 4,
    OEM_FIXED_FONT = 10,
}

#[win32_derive::dllexport]
pub fn GetStockObject(machine: &mut Machine, i: Result<GetStockObjectArg, u32>) -> HGDIOBJ {
    match i.unwrap() {
        GetStockObjectArg::WHITE_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: COLORREF((0xff, 0xff, 0xff)),
        })),
        GetStockObjectArg::LTGRAY_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: COLORREF((0xc0, 0xc0, 0xc0)),
        })),
        GetStockObjectArg::BLACK_BRUSH => machine.state.gdi32.objects.add(Object::Brush(Brush {
            color: COLORREF((0x00, 0x00, 0x00)),
        })),
        GetStockObjectArg::OEM_FIXED_FONT => {
            log::error!("returning null stock object");
            HGDIOBJ::null()
        }
        _ => todo!(),
    }
}

#[win32_derive::dllexport]
pub fn SelectObject(machine: &mut Machine, hdc: HDC, hGdiObj: HGDIOBJ) -> HGDIOBJ {
    let dc = match machine.state.gdi32.dcs.get_mut(hdc) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(dc) => dc,
    };

    let obj = match machine.state.gdi32.objects.get(hGdiObj) {
        None => return HGDIOBJ::null(), // TODO: HGDI_ERROR
        Some(obj) => obj,
    };
    match obj {
        Object::Bitmap(_) => match dc.target {
            DCTarget::Memory(prev) => {
                dc.target = DCTarget::Memory(hGdiObj);
                prev
            }
            DCTarget::Window(_) => todo!(),
            DCTarget::DirectDrawSurface(_) => todo!(),
        },
        Object::Brush(_) => std::mem::replace(&mut dc.brush, hGdiObj),
        Object::Pen(_) => std::mem::replace(&mut dc.pen, hGdiObj),
    }
}

#[win32_derive::dllexport]
pub fn GetObjectA(machine: &mut Machine, handle: HGDIOBJ, bytes: u32, out: u32) -> u32 {
    let obj = match machine.state.gdi32.objects.get(handle) {
        None => return 0, // fail
        Some(obj) => obj,
    };

    match obj {
        Object::Brush(_) => todo!(),
        Object::Bitmap(bitmap) => {
            assert_eq!(bytes as usize, std::mem::size_of::<BITMAP>());
            let out = machine.mem().view_mut::<BITMAP>(out);
            *out = BITMAP {
                bmType: 0,
                bmWidth: bitmap.width,
                bmHeight: bitmap.height,
                bmWidthBytes: 0,
                bmPlanes: 0,
                bmBitsPixel: 0,
                bmBits: 0,
            };
            bytes
        }
        Object::Pen(_) => todo!(),
    }
}

#[win32_derive::dllexport]
pub fn DeleteObject(_machine: &mut Machine, handle: HGDIOBJ) -> bool {
    // TODO: leak
    true
}