use crate::segment_tree::{Segment, SegmentTree};
use crate::error::{Result, Error};
use std::fmt;
use std::borrow::Cow;
use im::HashMap;

fn err(fmt_res: fmt::Result) -> Result<()> {
    fmt_res.map_err(|err| Error::Writer(err))
}

/// Returns true if the value is not empty and is not equal to "0"
fn is_truthy(val: &str) -> bool {
    !val.is_empty() && val != "0"
}

fn resolve_segment(f: &mut dyn fmt::Write, segment: &Segment, context: &HashMap<Cow<str>, Cow<str>>) -> Result<()> {
    match segment {
        Segment::Text(text) => {
            err(f.write_str(text))
        },
        Segment::Not(val_tree) => {
            let val_str = resolve_tree_str(val_tree, context)?;
            if is_truthy(val_str.trim()) {
                err(f.write_char('0'))
            } else {
                err(f.write_char('1'))
            }
        }
        Segment::Loop { count, contents } => {
            let count_str = resolve_tree_str(&count, context)?;
            let trimmed_count = count_str.trim();
            let count_int = trimmed_count.parse().map_err(|_| Error::ShouldBeInteger(trimmed_count.to_string()))?;

            for index in 0..count_int {
                let derived_context = context.update(Cow::Borrowed("LOOP_INDEX"), Cow::Owned(index.to_string()));
                resolve_tree(f, &contents, &derived_context)?;
            }

            Ok(())
        }
        Segment::If { condition, contents } => {
            let condition_str = resolve_tree_str(&condition, context)?;
            if is_truthy(condition_str.trim()) {
                resolve_tree(f, &contents, context)?;
            }
            Ok(())
        }
        Segment::Variable { name } => {
            let resolved_name = resolve_tree_str(&name, context)?;
            let trimmed_name = resolved_name.trim();
            let value = match context.get(trimmed_name) {
                Some(val) => val,
                None => return Err(Error::NoSuchVariable(trimmed_name.to_string()))
            };
            err(f.write_str(value))
        }
        Segment::With { assignments, contents } => {
            let mut new_context = context.clone();
            for (var_tree, value_tree) in assignments {
                let var_name = resolve_tree_str(var_tree, &new_context)?.trim().to_owned();
                let var_value = resolve_tree_str(value_tree, &new_context)?;
                new_context.insert(Cow::Owned(var_name), Cow::Owned(var_value));
            }
            resolve_tree(f, contents, &new_context)
        }
    }
}

pub fn resolve_tree(f: &mut dyn fmt::Write, tree: &SegmentTree, context: &HashMap<Cow<str>, Cow<str>>) -> Result<()> {
    for segment in &tree.segments {
        resolve_segment(f, segment, context)?;
    }
    Ok(())
}

fn resolve_tree_str(tree: &SegmentTree, context: &HashMap<Cow<str>, Cow<str>>) -> Result<String> {
    let mut res = String::new();
    resolve_tree(&mut res, tree, context)?;
    Ok(res)
}
