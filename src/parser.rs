use crate::error::{Error, Result};
use crate::segment_tree::{Segment, SegmentTree};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref START_REGEX: Regex = Regex::new(r"(\{\{)|(\{%)").unwrap();
    static ref BLOCK_NAME_END_REGEX: Regex = Regex::new(r" |%\}").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"[^\S\n]+$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terminal<'s> {
    Text(&'s str),
    Eof,
}

impl<'s> Terminal<'s> {
    fn len(self) -> usize {
        match self {
            Terminal::Text(text) => text.len(),
            Terminal::Eof => 0,
        }
    }

    fn find_in(self, val: &str) -> Option<usize> {
        match self {
            Terminal::Text(text) => val.find(text),
            Terminal::Eof => Some(val.len()),
        }
    }

    fn as_str(self) -> &'s str {
        match self {
            Terminal::Text(text) => text,
            Terminal::Eof => "EOF",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NewlineMode {
    None,
    TrimStart,
}

pub fn parse_template<'s>(input: &'s str) -> Result<SegmentTree<'s>> {
    Ok(parse_subexpr(input, Terminal::Eof, NewlineMode::None)?.1)
}

fn parse_subexpr<'s>(
    input: &'s str,
    terminal: Terminal,
    newline_mode: NewlineMode,
) -> Result<(&'s str, SegmentTree<'s>)> {
    let mut remaining_text = input;
    let mut segments = Vec::new();

    loop {
        // We only want to look for a start sequence that's before the next terminal
        let terminal_index = match terminal.find_in(remaining_text) {
            Some(index) => index,
            None => return Err(Error::MissingTerminal(terminal.as_str().to_string())),
        };
        let before_terminal_text = &remaining_text[..terminal_index];

        let found_start = match START_REGEX.captures(before_terminal_text) {
            Some(found) => found,
            None => {
                // No more variables or conditionals left, push the remaining text if any and exit
                // after removing whitespace when necessary (to remove whitespace due to indents
                // before blocks)
                let before_trimmed_text = if newline_mode == NewlineMode::TrimStart {
                    trim_whitespace_at_end(before_terminal_text)
                } else {
                    before_terminal_text
                };
                if !before_trimmed_text.is_empty() {
                    segments.push(Segment::Text(before_trimmed_text));
                }

                let after_terminal_text = &remaining_text[(terminal_index + terminal.len())..];
                // Remove newlines after the text if told to, to remove newlines from blocks
                let new_remaining_text = if newline_mode == NewlineMode::TrimStart
                    && after_terminal_text.starts_with('\n')
                {
                    &after_terminal_text[1..]
                } else {
                    after_terminal_text
                };
                return Ok((new_remaining_text, SegmentTree { segments }));
            }
        };

        let main_capture_group = found_start.get(0).unwrap();

        let before_start_text = &remaining_text[..main_capture_group.start()];
        let after_start_text = &remaining_text[main_capture_group.end()..];

        // Look at what the variable or block actually was
        let (new_remaining_text, new_segment, last_newline_mode) = if found_start.get(1).is_some() {
            // It's the start of a variable
            parse_variable(after_start_text)?
        } else if found_start.get(2).is_some() {
            // It's the start of a block
            let name_end_index = match BLOCK_NAME_END_REGEX.find(after_start_text) {
                Some(m) => m.start(),
                None => after_start_text.len(),
            };
            let block_name = &after_start_text[..name_end_index];
            let after_name_text = &after_start_text[(name_end_index + 1)..];
            match block_name {
                "LOOP" => parse_loop(after_name_text),
                "IF" => parse_if(after_name_text),
                "NOT" => parse_not(after_name_text),
                "WITH" => parse_with(after_name_text),
                _ => return Err(Error::UnknownBlock(block_name.to_string())),
            }?
        } else {
            panic!();
        };

        // Place everything before the start delimiter as a Text segment, if there _is_ anything.
        // Also trim non-newline whitespace here if the last processed section flags it, to remove
        // some leftover whitespace from around block statements.
        let before_trimmed_text = if last_newline_mode == NewlineMode::TrimStart {
            trim_whitespace_at_end(before_start_text)
        } else {
            before_start_text
        };
        if !before_trimmed_text.is_empty() {
            segments.push(Segment::Text(before_trimmed_text))
        }

        segments.push(new_segment);
        remaining_text = new_remaining_text;
    }
}

// Trims whitespace excluding newlines
fn trim_whitespace_at_end(val: &str) -> &str {
    let end_whitespace = WHITESPACE_REGEX.find(val);
    match end_whitespace {
        Some(m) => &val[..m.start()],
        None => val,
    }
}

fn parse_variable<'s>(remaining_text: &'s str) -> Result<(&'s str, Segment, NewlineMode)> {
    let (remaining_text, var_tree) =
        parse_subexpr(remaining_text, Terminal::Text("}}"), NewlineMode::None)?;
    Ok((
        remaining_text,
        Segment::Variable { name: var_tree },
        NewlineMode::None,
    ))
}

fn parse_loop<'s>(remaining_text: &'s str) -> Result<(&'s str, Segment, NewlineMode)> {
    let (remaining_text, count_tree) =
        parse_subexpr(remaining_text, Terminal::Text("%}"), NewlineMode::TrimStart)?;
    let (remaining_text, body_tree) = parse_subexpr(
        remaining_text,
        Terminal::Text("{%END%}"),
        NewlineMode::TrimStart,
    )?;
    Ok((
        remaining_text,
        Segment::Loop {
            count: count_tree,
            contents: body_tree,
        },
        NewlineMode::TrimStart,
    ))
}

fn parse_if<'s>(remaining_text: &'s str) -> Result<(&'s str, Segment, NewlineMode)> {
    let (remaining_text, condition_tree) =
        parse_subexpr(remaining_text, Terminal::Text("%}"), NewlineMode::TrimStart)?;
    let (remaining_text, body_tree) = parse_subexpr(
        remaining_text,
        Terminal::Text("{%END%}"),
        NewlineMode::TrimStart,
    )?;
    Ok((
        remaining_text,
        Segment::If {
            condition: condition_tree,
            contents: body_tree,
        },
        NewlineMode::TrimStart,
    ))
}

fn parse_with<'s>(remaining_text: &'s str) -> Result<(&'s str, Segment, NewlineMode)> {
    let (remaining_text, assignments_tree) =
        parse_subexpr(remaining_text, Terminal::Text("%}"), NewlineMode::TrimStart)?;

    // First put segments into groups, splitting on commas
    let mut assignment_groups = vec![Vec::new()];

    for segment in assignments_tree.segments.into_iter() {
        match segment {
            Segment::Text(text) => match text.find(',') {
                Some(comma_location) => {
                    let before_comma = &text[..comma_location];
                    let after_comma = &text[(comma_location + 1)..];

                    if !before_comma.is_empty() {
                        assignment_groups
                            .last_mut()
                            .unwrap()
                            .push(Segment::Text(before_comma))
                    }
                    if !after_comma.is_empty() {
                        assignment_groups.push(vec![Segment::Text(after_comma)])
                    } else {
                        assignment_groups.push(Vec::new());
                    }
                }
                None => assignment_groups.last_mut().unwrap().push(segment),
            },
            _ => assignment_groups.last_mut().unwrap().push(segment),
        }
    }

    // Now split each group into a variable name part and a value part
    let assignments = assignment_groups
        .into_iter()
        .map(|group| {
            let mut var_name_segments = Vec::new();
            let mut value_segments = Vec::new();
            let mut is_finding_var_name = true;

            for segment in group.into_iter() {
                match segment {
                    Segment::Text(text) => match text.find('=') {
                        Some(equal_location) => {
                            let before_equal = &text[..equal_location];
                            let after_equal = &text[(equal_location + 1)..];

                            if !is_finding_var_name {
                                return Err(Error::TooManyAssignmentsInWith);
                            }
                            is_finding_var_name = false;
                            if !before_equal.is_empty() {
                                var_name_segments.push(Segment::Text(before_equal))
                            }
                            if !after_equal.is_empty() {
                                value_segments.push(Segment::Text(after_equal))
                            }
                        }
                        None => {
                            if is_finding_var_name {
                                var_name_segments.push(segment);
                            } else {
                                value_segments.push(segment);
                            }
                        }
                    },
                    _ => {
                        if is_finding_var_name {
                            var_name_segments.push(segment);
                        } else {
                            value_segments.push(segment);
                        }
                    }
                }
            }

            // If we're still set to "finding var name", we haven't encountered an = yet and should
            // return an error.
            if is_finding_var_name {
                Err(Error::NoAssignmentInWith)
            } else {
                Ok((
                    SegmentTree {
                        segments: var_name_segments,
                    },
                    SegmentTree {
                        segments: value_segments,
                    },
                ))
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let (remaining_text, body_tree) = parse_subexpr(
        remaining_text,
        Terminal::Text("{%END%}"),
        NewlineMode::TrimStart,
    )?;

    Ok((
        remaining_text,
        Segment::With {
            assignments,
            contents: body_tree,
        },
        NewlineMode::TrimStart,
    ))
}

fn parse_not<'s>(remaining_text: &'s str) -> Result<(&'s str, Segment, NewlineMode)> {
    let (remaining_text, value_tree) =
        parse_subexpr(remaining_text, Terminal::Text("%}"), NewlineMode::None)?;
    Ok((remaining_text, Segment::Not(value_tree), NewlineMode::None))
}
