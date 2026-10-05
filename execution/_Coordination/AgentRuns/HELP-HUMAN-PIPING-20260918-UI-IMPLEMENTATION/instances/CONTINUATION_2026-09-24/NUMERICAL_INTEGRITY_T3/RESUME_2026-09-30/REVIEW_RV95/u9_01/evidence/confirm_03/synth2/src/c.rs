pub fn f() { if true { } }
mod w { mod e; }
mod x { pub mod y { #[path = "deeper.rs"] mod d2; } }
