use std::collections::HashMap;
use std::borrow::Borrow;
use std::{cmp, hash};

pub trait Context {
    fn get_variable(&self, var_name: &str) -> Option<&str>;
}

impl<BA: Borrow<str> + cmp::Eq + hash::Hash, BB: Borrow<str> + cmp::Eq + hash::Hash> Context for HashMap<BA, BB> {
    fn get_variable(&self, var_name: &str) -> Option<&str> {
        self.get(var_name).map(BB::borrow)
    }
}

pub struct DerivedContext<'d> {
    pub parent: &'d dyn Context,
    pub child: &'d dyn Context,
}

impl<'d> Context for DerivedContext<'d> {
    fn get_variable(&self, var_name: &str) -> Option<&str> {
        self.child.get_variable(var_name).or_else(|| self.parent.get_variable(var_name))
    }
}
