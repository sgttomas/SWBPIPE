mod inl {
    fn k() {
        let _s = "}{";
    }
    mod c;
    #[path = "q.rs"]
    mod q;
}
#[path = "side.rs"]
mod side;
