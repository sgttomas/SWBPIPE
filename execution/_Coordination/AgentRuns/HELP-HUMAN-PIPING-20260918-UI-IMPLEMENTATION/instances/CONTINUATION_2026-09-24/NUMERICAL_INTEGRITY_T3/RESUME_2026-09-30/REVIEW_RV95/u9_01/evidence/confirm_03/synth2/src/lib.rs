#![allow(dead_code)]
mod a;
mod b;
mod c;
#[cfg(test)]
pub(crate) mod t { pub fn h() { let _ = '{'; } mod g; #[path = "pp.rs"] mod p; }
mod outer { pub mod inner { mod deep; } }
