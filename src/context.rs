use crate::Gheedorah;

pub struct Context {
    pub contract: Gheedorah,
}

impl Context {
    pub fn new(instance: Gheedorah) -> Self {
        Context { contract }
    }
}

impl juniper::Context for Context {}
