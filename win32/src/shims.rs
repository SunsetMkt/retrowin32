//! "Shims" are my word for the mechanism for x86 -> retrowin32 (and back) calls.

use crate::Machine;

struct Shim {
    name: String,
    handler: Option<fn(&mut Machine)>,
}

pub struct Shims {}
impl Default for Shims {
    fn default() -> Self {
        Self {}
    }
}
impl Shims {
    pub fn add(&mut self, name: String, handler: Option<fn(&mut Machine)>) -> u32 {
        1
    }
}

pub struct UnimplFuture {}
impl std::future::Future for UnimplFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

pub fn async_call(machine: &mut Machine, func: u32, args: Vec<u32>) -> UnimplFuture {
    todo!()
}
