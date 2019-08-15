mod asset_catalog;
mod ast;
mod parser;

use crate::asset_catalog::write_asset_catalog;
use crate::parser::parse_document_from_file;
use clap::{App, Arg};

fn main() {
  let matches = App::new("color-assets")
    .version("1.0")
    .about("Create Xcode Asset Catalog with colors for light & dark mode.")
    .arg(
      Arg::with_name("output")
        .short("o")
        .help("Sets the output filename")
        .value_name("OUTPUT_FILE")
        .required(true),
    )
    .arg(
      Arg::with_name("input")
        .help("Sets the input file")
        .value_name("INPUT_FILE")
        .required(true)
        .index(1),
    )
    .get_matches();

  let input_file = matches.value_of("input").unwrap();
  let output_path = matches.value_of("output").unwrap();

  let doc = parse_document_from_file(&input_file).expect("Could not parse input file.");
  write_asset_catalog(&doc, &output_path).expect("Could not write asset catalog.");
}

#[cfg(test)]
mod tests {
  use crate::ast::{Color, ColorSet, ColorSetValue, Value};
  use crate::parser::{colorset, parse_document, ruleset, variable};
  use insta::assert_debug_snapshot_matches;

  #[test]
  fn test_document() {
    let doc = r#"
    mediumBright = #aabbcc 33%
    red = #ff0000
    white = #ffffff

    root {
      ApplicationBackground: (light: $white, dark: #141517)

      NumericInput {
        ActionKey {
          Background: (light: $red, dark: #ff00ff)
          Highlight: (light: #cccccc, dark: #000000)
        }
      }
    }
    "#;

    let (_, doc) = parse_document(&doc).unwrap();
    assert_debug_snapshot_matches!("test_document", doc);
  }

  #[test]
  fn test_ruleset() {
    let doc = r#"root {
      ApplicationBackground: (light: $white, dark: #141517)

      Key {
        Background: (light: $red, dark: #ff00ff)
      }

      AnotherKey {
        Highlight: (light: #cccccc, dark: #000000)
      }
    }"#;

    let (_, doc) = ruleset(&doc).unwrap();
    assert_debug_snapshot_matches!("test_ruleset", doc);
  }

  #[test]
  fn test_color_variable() {
    let (_, v1) = variable(&"myColor = #ff00ff 44%").unwrap();
    assert_eq!(v1.identifier, "myColor");
    assert_eq!(
      v1.value,
      Value::Color(Color {
        r: 255,
        g: 0,
        b: 255,
        a: 0.44
      })
    );

    let (_, v2) = variable(&"a = #4224be").unwrap();
    assert_eq!(v2.identifier, "a");
    assert_eq!(
      v2.value,
      Value::Color(Color {
        r: 66,
        g: 36,
        b: 190,
        a: 1.0
      })
    );

    let (_, v3) = variable(&"a1 = #4B0FC6").unwrap();
    assert_eq!(v3.identifier, "a1");
    assert_eq!(
      v3.value,
      Value::Color(Color {
        r: 75,
        g: 15,
        b: 198,
        a: 1.0
      })
    );
  }

  #[test]
  fn test_colorset() {
    let val1 = ColorSetValue::Color(Color {
      r: 255,
      g: 0,
      b: 255,
      a: 0.3,
    });
    let val2 = ColorSetValue::Color(Color {
      r: 0,
      g: 255,
      b: 0,
      a: 1.0,
    });

    let expected_result = ColorSet {
      light: val1.clone(),
      dark: val2,
    };

    let (_, v1) = colorset(&"(light: #ff00ff 30%, dark: #00ff00)").unwrap();
    assert_eq!(v1, expected_result);
    let (_, v2) = colorset(&"(dark: #00ff00, light: #ff00ff 30%)").unwrap();
    assert_eq!(v2, expected_result);
    let (_, v3) = colorset(&"(dark: $applicationBackgroundLight, light: #ff00ff 30%)").unwrap();
    assert_eq!(
      v3,
      ColorSet {
        light: val1,
        dark: ColorSetValue::Variable("applicationBackgroundLight".to_string()),
      }
    );
  }
}
