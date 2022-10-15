#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone, clap::ValueEnum)]
pub enum Scope {
    Global,
    Local,
}
impl Scope {
    pub(crate) fn is_global(&self) -> bool {
        &Self::Global == self
    }
}

impl Scope {}
