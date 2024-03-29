use std::fs::File;
use std::io::prelude::Read;
use std::num::ParseIntError;
use std::path::Path;
use std::str;

use crate::error::Error;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{
  alphanumeric1, char, digit1, multispace0, newline, not_line_ending, space0, space1,
};
use nom::combinator::{all_consuming, cut, map, map_res, opt};
use nom::error::{
  context, convert_error, ContextError, FromExternalError, ParseError, VerboseError,
};
use nom::multi::{many0, separated_list0};
use nom::number::complete::float;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::Err;
use nom::IResult;

use super::ast::{
  Color, ColorSet, ColorSetValue, Declaration, Document, DocumentItem, RuleSet, RuleSetItem, Value,
  Variable,
};

fn parse_hex_value(input: &str) -> Result<u32, std::num::ParseIntError> {
  u32::from_str_radix(input, 16)
}

fn colorset_from_declarations(
  decl1: Declaration<ColorSetValue>,
  decl2: Declaration<ColorSetValue>,
) -> Result<ColorSet, Error> {
  match (decl1.identifier.as_str(), decl2.identifier.as_str()) {
    ("light", "dark") => Ok(ColorSet {
      light: decl1.value,
      dark: decl2.value,
    }),
    ("dark", "light") => Ok(ColorSet {
      light: decl2.value,
      dark: decl1.value,
    }),
    _ => Err(Error::InvalidColorSetDeclaration {
      message: format!(
        "Expected light & dark properties. Found {}, {}.",
        decl1.identifier, decl2.identifier
      ),
    }),
  }
}

pub fn parse_document(input: String) -> Result<Document, Error> {
  let mut modified_input = input;
  modified_input.push_str("\n");

  let result: IResult<&str, Document, VerboseError<&str>> = map(
    all_consuming(delimited(
      multiline_whitespace,
      separated_list0(
        line_delimiter,
        alt((
          map(variable, DocumentItem::Variable),
          map(ruleset, DocumentItem::RuleSet),
          map(declaration(value), DocumentItem::Declaration),
        )),
      ),
      multiline_whitespace,
    )),
    |res| Document { items: res },
  )(&modified_input);

  match result {
    Ok((_, doc)) => Ok(doc),
    Err(Err::Error(e)) | Err(Err::Failure(e)) => Err(Error::ParseError {
      message: convert_error(modified_input.as_ref(), e).replace("'\n'", "'\\n'"),
    }),
    _ => {
      eprintln!("An unknown error occurred.");
      std::process::exit(0x0100)
    }
  }
}

pub fn parse_document_from_file(filepath: impl AsRef<Path>) -> Result<Document, Error> {
  let mut f = File::open(filepath)?;
  let mut buffer = vec![];
  f.read_to_end(&mut buffer).unwrap();

  let contents = str::from_utf8(&buffer)?;
  parse_document(contents.to_string())
}

fn single_line_comment<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, &str, E> {
  context("Single Line Comment", preceded(tag("//"), not_line_ending))(input)
}

fn line_delimiter<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, char, E> {
  cut(preceded(
    space0,
    preceded(
      opt(single_line_comment),
      terminated(newline, multiline_whitespace),
    ),
  ))(input)
}

fn multiline_whitespace<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, Vec<&str>, E> {
  delimited(
    multispace0,
    separated_list0(terminated(newline, multispace0), single_line_comment),
    multispace0,
  )(input)
}

fn ruleset<
  'a,
  E: ParseError<&'a str>
    + ContextError<&'a str>
    + FromExternalError<&'a str, ParseIntError>
    + FromExternalError<&'a str, Error>,
>(
  input: &'a str,
) -> IResult<&'a str, RuleSet, E> {
  let body = |input| {
    delimited(
      terminated(char('{'), multiline_whitespace),
      cut(many0(terminated(
        alt((
          map(ruleset, RuleSetItem::RuleSet),
          map(declaration(value), RuleSetItem::Declaration),
        )),
        line_delimiter,
      ))),
      preceded(multiline_whitespace, char('}')),
    )(input)
  };

  context(
    "RuleSet",
    map(separated_pair(identifier, space0, body), |res| RuleSet {
      identifier: res.0.to_string(),
      items: res.1,
    }),
  )(input)
}

fn variable<
  'a,
  E: ParseError<&'a str>
    + ContextError<&'a str>
    + FromExternalError<&'a str, ParseIntError>
    + FromExternalError<&'a str, Error>,
>(
  input: &'a str,
) -> IResult<&'a str, Declaration<Value>, E> {
  context(
    "Variable",
    map(
      separated_pair(
        variable_identifier,
        cut(terminated(preceded(space0, char(':')), space0)),
        value,
      ),
      |res| {
        let (identifier, value) = res;
        Declaration { identifier, value }
      },
    ),
  )(input)
}

fn value<
  'a,
  E: ParseError<&'a str>
    + ContextError<&'a str>
    + FromExternalError<&'a str, ParseIntError>
    + FromExternalError<&'a str, Error>,
>(
  input: &'a str,
) -> IResult<&'a str, Value, E> {
  preceded(
    space0,
    alt((
      map(colorset_value, |res| res.into()),
      map(colorset, Value::ColorSet),
    )),
  )(input)
}

fn colorset_value<
  'a,
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
  input: &'a str,
) -> IResult<&'a str, ColorSetValue, E> {
  preceded(
    space0,
    alt((
      map(hex_color, ColorSetValue::Color),
      map(rgba_color, ColorSetValue::Color),
      map(variable_value, ColorSetValue::Variable),
    )),
  )(input)
}

fn rgba_color<
  'a,
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
  input: &'a str,
) -> IResult<&'a str, Color, E> {
  let u8_value = move |input: &'a str| {
    preceded(
      space0,
      cut(context(
        "Value between 0 - 255",
        map_res(digit1, |s: &str| s.parse::<u8>()),
      )),
    )(input)
  };
  let delimiter = move |input: &'a str| preceded(space0, cut(char(',')))(input);

  context(
    "RGBA Value",
    map(
      preceded(
        tag("rgba"),
        delimited(
          preceded(space0, char('(')),
          tuple((
            terminated(u8_value, delimiter),
            terminated(u8_value, delimiter),
            terminated(u8_value, delimiter),
            preceded(space0, float),
          )),
          preceded(space0, char(')')),
        ),
      ),
      |res| {
        let (r, g, b, a) = res;
        Color { r, g, b, a }
      },
    ),
  )(input)
}

fn hex_color<
  'a,
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
  input: &'a str,
) -> IResult<&'a str, Color, E> {
  map(
    tuple((
      char('#'),
      cut(hex_value),
      opt(preceded(space1, alpha_value)),
    )),
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

pub fn colorset<
  'a,
  E: ParseError<&'a str>
    + ContextError<&'a str>
    + FromExternalError<&'a str, ParseIntError>
    + FromExternalError<&'a str, Error>,
>(
  input: &'a str,
) -> IResult<&'a str, ColorSet, E> {
  context(
    "ColorSet",
    map_res(
      delimited(
        cut(char('(')),
        separated_pair(
          cut(declaration(colorset_value)),
          cut(preceded(space0, char(','))),
          cut(declaration(colorset_value)),
        ),
        cut(char(')')),
      ),
      |res| colorset_from_declarations(res.0, res.1),
    ),
  )(input)
}

pub fn identifier<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, String, E> {
  context(
    "Identifier",
    map(preceded(space0, alphanumeric1), |ident: &str| {
      ident.to_string()
    }),
  )(input)
}

pub fn variable_identifier<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
  input: &'a str,
) -> IResult<&'a str, String, E> {
  context(
    "Variable Identifier",
    map(
      preceded(space0, preceded(char('$'), alphanumeric1)),
      |ident: &str| ident.to_string(),
    ),
  )(input)
}

pub fn variable_value<
  'a,
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
  input: &'a str,
) -> IResult<&'a str, Variable, E> {
  map(
    tuple((variable_identifier, opt(preceded(space1, alpha_value)))),
    |res| {
      let (identifier, opacity) = res;
      Variable {
        identifier,
        opacity: opacity.unwrap_or(1.0),
      }
    },
  )(input)
}

pub fn declaration<'a, F, V, E: ParseError<&'a str> + ContextError<&'a str>>(
  value: F,
) -> impl Fn(&'a str) -> IResult<&'a str, Declaration<V>, E>
where
  F: Fn(&'a str) -> IResult<&'a str, V, E>,
{
  move |input: &'a str| {
    context(
      "Declaration",
      map(
        separated_pair(identifier, cut(preceded(space0, char(':'))), &value),
        |res| {
          let (identifier, value) = res;
          Declaration { identifier, value }
        },
      ),
    )(input)
  }
}

fn hex_value<
  'a,
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
  input: &'a str,
) -> IResult<&'a str, u32, E> {
  context(
    "Hex Value",
    map_res(take_while_m_n(6, 6, is_hex_digit), parse_hex_value),
  )(input)
}

fn alpha_value<
  'a,
  E: ParseError<&'a str> + ContextError<&'a str> + FromExternalError<&'a str, ParseIntError>,
>(
  input: &'a str,
) -> IResult<&'a str, f32, E> {
  context(
    "Alpha Value",
    map_res(terminated(digit1, tag("%")), |input| {
      u32::from_str_radix(input, 10).map(|val| (val as f32) / 100.0)
    }),
  )(input)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}
