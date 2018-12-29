use im::HashMap;
use std::fmt;
use std::borrow::Cow;

mod error;
mod parser;
mod resolver;
mod segment_tree;

use crate::error::Result;
use crate::parser::parse_template;
use crate::resolver::resolve_tree;

pub fn process_template<'s>(f: &mut impl fmt::Write, template_text: &str, context: impl Into<HashMap<Cow<'s, str>, Cow<'s, str>>>) -> Result<()> {
    let parsed_tree = parse_template(template_text)?;
    resolve_tree(f, &parsed_tree, &context.into())
}