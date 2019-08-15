use super::ast::{
  Color, ColorSet, ColorSetValue, Declaration, Document, DocumentItem, RuleSet, RuleSetItem, Value,
};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{alphanumeric1, char, digit1, multispace0, newline, space0, space1};
use nom::combinator::{map, map_res, opt};
use nom::error::{context, ParseError, VerboseError};
use nom::multi::{many0, separated_list};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::Err;
use nom::IResult;

use std::fs::File;
use std::io::prelude::Read;
use std::str;

#[allow(dead_code)]
fn parse_hex_value(input: &str) -> Result<u32, std::num::ParseIntError> {
  u32::from_str_radix(input, 16)
}

#[allow(dead_code)]
fn colorset_from_declarations(
  decl1: Declaration<ColorSetValue>,
  decl2: Declaration<ColorSetValue>,
) -> Result<ColorSet, String> {
  match (decl1.identifier.as_str(), decl2.identifier.as_str()) {
    ("light", "dark") => Ok(ColorSet {
      light: decl1.value,
      dark: decl2.value,
    }),
    ("dark", "light") => Ok(ColorSet {
      light: decl2.value,
      dark: decl1.value,
    }),
    _ => Err(format!(
      "Expected light & dark properties. Found {}, {}.",
      decl1.identifier, decl2.identifier
    )),
  }
}

#[allow(dead_code)]
fn line_delimiter(input: &str) -> IResult<&str, char> {
  terminated(preceded(space0, newline), multispace0)(input)
}

#[allow(dead_code)]
pub fn parse_document(input: &str) -> IResult<&str, Document> {
  map(
    preceded(
      multispace0,
      separated_list(
        line_delimiter,
        alt((
          map(variable, DocumentItem::Variable),
          map(ruleset, DocumentItem::RuleSet),
        )),
      ),
    ),
    |res| Document { items: res },
  )(input)
}

pub fn parse_document_from_file(filepath: &str) -> Result<Document, VerboseError<String>> {
  let mut f = File::open(filepath).unwrap();
  let mut buffer = vec![];
  f.read_to_end(&mut buffer).unwrap();

  let contents = match str::from_utf8(&buffer) {
    Ok(v) => v,
    Err(e) => panic!(
      "Could not read contents of file {}. Reason: {}",
      filepath, e
    ),
  };

  match parse_document(&contents) {
    Ok(res) => Ok(res.1),
    Err(Err::Error(e)) | Err(Err::Failure(e)) => {
      Err(VerboseError::from_error_kind(contents.to_string(), e.1))
    }
    _ => {
      eprintln!("Something went wrong");
      std::process::exit(0x0100)
    }
  }
}

#[allow(dead_code)]
pub fn ruleset(input: &str) -> IResult<&str, RuleSet> {
  let body = |input| {
    preceded(
      terminated(char('{'), multispace0),
      terminated(
        many0(alt((
          map(terminated(ruleset, line_delimiter), RuleSetItem::RuleSet),
          map(
            terminated(declaration(value), line_delimiter),
            RuleSetItem::Declaration,
          ),
        ))),
        preceded(multispace0, char('}')),
      ),
    )(input)
  };

  context(
    "ruleset",
    map(tuple((identifier, space0, body)), |res| RuleSet {
      identifier: res.0.to_string(),
      items: res.2,
    }),
  )(input)
}

pub fn variable(input: &str) -> IResult<&str, Declaration<Value>> {
  map(
    separated_pair(
      identifier,
      terminated(preceded(space0, char('=')), space0),
      value,
    ),
    |res| {
      let (identifier, value) = res;
      Declaration { identifier, value }
    },
  )(input)
}

fn value(input: &str) -> IResult<&str, Value> {
  alt((
    map(colorset_value, |res| res.into()),
    map(colorset, Value::ColorSet),
  ))(input)
}

fn colorset_value(input: &str) -> IResult<&str, ColorSetValue> {
  alt((
    map(hex_color, ColorSetValue::Color),
    map(preceded(char('$'), identifier), ColorSetValue::Variable),
  ))(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
  map(
    tuple((tag("#"), hex_value, opt(preceded(space1, alpha_value)))),
    |res| {
      let (_, rgb, alpha) = res;
      let r = ((rgb >> 16) & 0xff) as u8;
      let g = ((rgb >> 8) & 0xff) as u8;
      let b = (rgb & 0xff) as u8;
      let a = alpha.unwrap_or(1.0);
      Color { r, g, b, a }
    },
  )(input)
}

pub fn colorset(input: &str) -> IResult<&str, ColorSet> {
  map_res(
    tuple((
      tag("("),
      declaration(colorset_value),
      space0,
      tag(","),
      multispace0,
      declaration(colorset_value),
      tag(")"),
    )),
    |res| colorset_from_declarations(res.1, res.5),
  )(input)
}

pub fn identifier(input: &str) -> IResult<&str, String> {
  map(preceded(space0, alphanumeric1), |ident: &str| {
    ident.to_string()
  })(input)
}

pub fn declaration<F, V>(value: F) -> impl Fn(&str) -> IResult<&str, Declaration<V>>
where
  F: Fn(&str) -> IResult<&str, V>,
{
  move |input: &str| {
    map(
      separated_pair(
        identifier,
        terminated(preceded(space0, char(':')), space0),
        &value,
      ),
      |res| {
        let (identifier, value) = res;
        Declaration { identifier, value }
      },
    )(input)
  }
}

fn hex_value(input: &str) -> IResult<&str, u32> {
  map_res(take_while_m_n(6, 6, is_hex_digit), parse_hex_value)(input)
}

fn alpha_value(input: &str) -> IResult<&str, f32> {
  map_res(terminated(digit1, tag("%")), |input| {
    u32::from_str_radix(input, 10).map(|val| (val as f32) / 100.0)
  })(input)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}
