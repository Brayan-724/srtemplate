use crate::SrTemplateError;

use super::{advance, is_eof, make_error, TemplateNode};

pub fn string_literal<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> Result<TemplateNode<'a>, SrTemplateError> {
    advance(chars, position, line, column, start_line);
    let start = *position;
    let mut is_scapped = false;

    while !is_eof(chars, *position) {
        let token = chars[*position];
        if is_scapped {
            is_scapped = false;
        } else if token == b'\\' {
            is_scapped = true;
        } else if token == b'"' {
            advance(chars, position, line, column, start_line);
            return Ok(TemplateNode::String(&input[start..*position - 1]));
        }
        advance(chars, position, line, column, start_line);
    }

    Err(make_error(
        input,
        chars,
        *line,
        *column,
        *start_line,
        "Unterminated string literal",
        start,
    ))
}

pub fn number_literal<'a>(
    input: &'a str,
    chars: &[u8],
    position: &mut usize,
    line: &mut usize,
    column: &mut usize,
    start_line: &mut usize,
) -> Result<TemplateNode<'a>, SrTemplateError> {
    let mut is_float = false;
    let start = *position;

    while !is_eof(chars, *position)
        && (chars[*position].is_ascii_digit() || chars[*position] == b'.')
    {
        if chars[*position] == b'.' && is_float {
            return Err(make_error(
                input,
                chars,
                *line,
                *column,
                *start_line,
                "The float just need one '.'",
                *position,
            ));
        }
        if chars[*position] == b'.' {
            is_float = true;
        }
        advance(chars, position, line, column, start_line);
    }

    if let Some(&token) = chars.get(*position) {
        if !(token.is_ascii_digit() || token == b'.' || token == b',' || token == b')') {
            return Err(make_error(
                input,
                chars,
                *line,
                *column,
                *start_line,
                "Invalid character in Number literal",
                *position,
            ));
        }
    }

    if is_float {
        return Ok(TemplateNode::Float(&input[start..*position]));
    }

    Ok(TemplateNode::Number(&input[start..*position]))
}
