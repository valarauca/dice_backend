#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VariableReference<'a> {
    pub name: &'a str,
}
