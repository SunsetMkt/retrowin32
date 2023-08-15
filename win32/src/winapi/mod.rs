use memory::MemImpl;

mod alloc;
mod bass;
mod builtin;
pub mod ddraw;
pub mod dsound;
pub mod gdi32;
pub mod kernel32;
mod ole32;
mod oleaut32;
mod stack_args;
pub mod types;
pub mod user32;
mod winmm;

macro_rules! vtable_entry {
    ($shims:ident $module:ident $fn:ident todo) => {
        0u32
    };
    ($shims:ident $module:ident $fn:ident ok) => {
        $shims.add($module::$fn)
    };
    ($shims:ident $module:ident $fn:ident $shim:tt) => {
        $shims.add($shim)
    };
}
pub(crate) use vtable_entry;

macro_rules! vtable {
    ($iface:ident $module:ident $($fn:ident $impl:tt,)*) => {
        #[repr(C)]
        struct Vtable {
            $($fn: DWORD),*
        }
        unsafe impl memory::Pod for Vtable {}
        impl Vtable {
            fn new(shims: &mut crate::shims::Shims) -> Self {
                Vtable {
                    $($fn: $crate::winapi::vtable_entry!(shims $module $fn $impl).into()),*
                }
            }
        }

        pub fn vtable(state: &mut State, machine: &mut Machine) -> u32 {
            let addr = machine.state.kernel32.get_heap(machine.memory.mem(), state.hheap).unwrap().alloc(
                std::mem::size_of::<Vtable>() as u32,
            );
            let vtable = machine.memory.mem().view_mut::<Vtable>(addr);
            *vtable = Vtable::new(&mut machine.shims);
            addr
        }
    };
}
pub(crate) use vtable;

#[derive(Debug)]
pub enum ImportSymbol<'a> {
    Name(&'a str),
    Ordinal(u32),
}
impl<'a> std::fmt::Display for ImportSymbol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportSymbol::Name(name) => f.write_str(name),
            ImportSymbol::Ordinal(ord) => f.write_fmt(format_args!("{}", ord)),
        }
    }
}

pub const DLLS: [builtin::BuiltinDLL; 9] = [
    builtin::bass::DLL,
    builtin::ddraw::DLL,
    builtin::dsound::DLL,
    builtin::gdi32::DLL,
    builtin::kernel32::DLL,
    builtin::ole32::DLL,
    builtin::oleaut32::DLL,
    builtin::user32::DLL,
    builtin::winmm::DLL,
];

#[derive(serde::Serialize, serde::Deserialize)]
pub struct State {
    #[serde(skip)] // TODO
    pub ddraw: ddraw::State,
    #[serde(skip)] // TODO
    pub dsound: dsound::State,
    #[serde(skip)] // TODO
    pub gdi32: gdi32::State,
    pub kernel32: kernel32::State,
    #[serde(skip)] // TODO
    pub user32: user32::State,
}

impl State {
    pub fn new(memory: &mut MemImpl) -> Self {
        State {
            ddraw: ddraw::State::default(),
            dsound: dsound::State::default(),
            gdi32: gdi32::State::default(),
            kernel32: kernel32::State::new(memory),
            user32: user32::State::default(),
        }
    }
}
