use crate::Gheedorah;

#[derive(Clone)]
pub struct Context {
    pub contract: Gheedorah,
}

impl Context {
    pub fn new(contract: Gheedorah) -> Self {
        Context { contract }
    }
}

impl juniper::Context for Context {}
