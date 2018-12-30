use std::fmt;

mod error;
mod parser;
mod resolver;
mod segment_tree;
mod context;

pub use crate::error::{Error, Result};
pub use crate::context::Context;
use crate::parser::parse_template;
use crate::resolver::resolve_tree;

pub fn process_template<'s>(
    f: &mut dyn fmt::Write,
    template_text: &str,
    context: &dyn Context,
) -> Result<()> {
    let parsed_tree = parse_template(template_text)?;
    resolve_tree(f, &parsed_tree, context)
}
