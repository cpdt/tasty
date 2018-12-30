use crate::error::{Error, Result};
use crate::segment_tree::{Segment, SegmentTree};
use crate::context::{Context, DerivedContext};
use std::fmt;
use std::collections::HashMap;

fn err(fmt_res: fmt::Result) -> Result<()> {
    fmt_res.map_err(Error::Writer)
}

/// Returns true if the value is not empty and is not equal to "0"
fn is_truthy(val: &str) -> bool {
    !val.is_empty() && val != "0"
}

fn resolve_segment(
    f: &mut dyn fmt::Write,
    segment: &Segment,
    context: &dyn Context,
) -> Result<()> {
    match segment {
        Segment::Text(text) => resolve_text(f, text),
        Segment::Not(val_tree) => resolve_not(f, val_tree, context),
        Segment::Loop { count, contents } => resolve_loop(f, count, contents, context),
        Segment::If {
            condition,
            contents,
        } => resolve_if(f, condition, contents, context),
        Segment::Variable { name } => resolve_variable(f, name, context),
        Segment::With {
            assignments,
            contents,
        } => resolve_with(f, assignments, contents, context),
    }
}

fn resolve_text(f: &mut dyn fmt::Write, text: &str) -> Result<()> {
    err(f.write_str(text))
}

fn resolve_not(f: &mut dyn fmt::Write, val_tree: &SegmentTree, context: &dyn Context) -> Result<()> {
    let val_str = resolve_tree_str(val_tree, context)?;
    if is_truthy(val_str.trim()) {
        err(f.write_char('0'))
    } else {
        err(f.write_char('1'))
    }
}

fn resolve_loop(f: &mut dyn fmt::Write, count: &SegmentTree, contents: &SegmentTree, context: &dyn Context) -> Result<()> {
    let count_str = resolve_tree_str(count, context)?;
    let trimmed_count = count_str.trim();
    let count_int = trimmed_count.parse().map_err(|_| Error::ShouldBeInteger(trimmed_count.to_string()))?;

    for index in 0..count_int {
        let mut sub_context = HashMap::new();
        sub_context.insert("LOOP_INDEX", index.to_string());
        let derived_context = DerivedContext {
            parent: context,
            child: &sub_context
        };
        resolve_tree(f, &contents, &derived_context)?;
    }

    Ok(())
}

fn resolve_if(f: &mut dyn fmt::Write, condition: &SegmentTree, contents: &SegmentTree, context: &dyn Context) -> Result<()> {
    let condition_str = resolve_tree_str(condition, context)?;
    if is_truthy(condition_str.trim()) {
        resolve_tree(f, contents, context)?;
    }
    Ok(())
}

fn resolve_variable(f: &mut dyn fmt::Write, name: &SegmentTree, context: &dyn Context) -> Result<()> {
    let resolved_name = resolve_tree_str(name, context)?;
    let trimmed_name = resolved_name.trim();
    let value = match context.get_variable(trimmed_name) {
        Some(val) => val,
        None => return Err(Error::NoSuchVariable(trimmed_name.to_string()))
    };
    err(f.write_str(value))
}

fn resolve_with(f: &mut dyn fmt::Write, assignments: &[(SegmentTree, SegmentTree)], contents: &SegmentTree, context: &dyn Context) -> Result<()> {
    let mut new_context = HashMap::new();
    for (var_tree, value_tree) in assignments {
        let child_context = DerivedContext {
            parent: context,
            child: &new_context
        };
        let var_name = resolve_tree_str(var_tree, &child_context)?.trim().to_owned();
        let var_value = resolve_tree_str(value_tree, &child_context)?;
        new_context.insert(var_name, var_value);
    }
    resolve_tree(f, contents, &DerivedContext {
        parent: context,
        child: &new_context
    })
}

pub fn resolve_tree(
    f: &'_ mut dyn fmt::Write,
    tree: &SegmentTree,
    context: &'_ dyn Context,
) -> Result<()> {
    for segment in &tree.segments {
        resolve_segment(f, segment, context)?;
    }
    Ok(())
}

fn resolve_tree_str(tree: &SegmentTree, context: &dyn Context) -> Result<String> {
    let mut res = String::new();
    resolve_tree(&mut res, tree, context)?;
    Ok(res)
}
