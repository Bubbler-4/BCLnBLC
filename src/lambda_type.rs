#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ULC {
    Var(usize),
    Lam(Box<ULC>),
    App(Box<(ULC, ULC)>),
}
