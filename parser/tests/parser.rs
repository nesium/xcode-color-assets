use insta::{assert_debug_snapshot_matches, assert_snapshot_matches};
use parser::parse_document;

#[test]
fn empty_document() {
  test_parse_doc("empty_document", "");
}

#[test]
fn variable_only() {
  test_parse_doc("variable_only", "$var: #ff00ff");
}

#[test]
fn declaration_only() {
  test_parse_doc("declaration_only", "a: #00ff00");
}

#[test]
fn empty_ruleset() {
  test_parse_doc("empty_ruleset_1", "a {}");
  test_parse_doc(
    "empty_ruleset_2",
    r#"
    a { b {}
    }
  "#,
  );
}

#[test]
fn variable_declaration_ruleset_order() {
  test_parse_doc(
    "variable_declaration_ruleset_order_1",
    r#"
      a: $var
      a {}
      $var: #ff00ff
  "#,
  );
  test_parse_doc(
    "variable_declaration_ruleset_order_2",
    r#"
      $var: #ff00ff
      a {}
      a: $var
  "#,
  );
}

#[test]
fn garbage_doc() {
  test_parse_doc("garbage_doc_1", "something");
  test_parse_doc("garbage_doc_2", ".");
  test_parse_doc("garbage_doc_3", "{}");
  test_parse_doc("garbage_doc_4", "$");
}

#[test]
fn color_variables() {
  test_parse_doc("color_variables_1", "$myColor : #ff00ff 44%");
  test_parse_doc("color_variables_2", "$a :#4224be");
  test_parse_doc("color_variables_3", "$a1: #4B0FC6");
  test_parse_doc("color_variables_4", "$a1: $a0");
  test_parse_doc("color_variables_5", "$a: rgba(0, 127, 255, 0.5)");
  test_parse_doc("color_variables_6", "$a: rgba (0, 127, 255, 0.5)");
  test_parse_doc("color_variables_7", "$a: (light: #ff00ff, dark: #00ff00)");
}

#[test]
fn garbage_hex_value() {
  test_parse_doc("garbage_hex_value_1", "a: #ff0zff");
  test_parse_doc("garbage_hex_value_2", "a: #fff");
  test_parse_doc("garbage_hex_value_3", "a: #ffffff 33?");
  test_parse_doc("garbage_hex_value_4", "a: #fffffff");
}

#[test]
fn garbage_rgba_values() {
  test_parse_doc("garbage_rgba_value_1", "$a: rgba(0, 127; 255, 0.5)");
  test_parse_doc("garbage_rgba_value_2", "$a: rgba0, 127, 255, 0.5)");
  test_parse_doc("garbage_rgba_value_3", "$a: rgba(0, 300, 255, 0.5)")
}

#[test]
fn color_sets() {
  test_parse_doc(
    "color_sets",
    r#"
    a: (light: #ff00ff 30%, dark: #00ff00)
    b: (dark: #00ff00, light: #ff00ff 30%)
    c: (dark: $applicationBackgroundLight, light: #ff00ff 30%)
    d: (light: rgba(33, 199, 201, 1), dark: #ff00ff 30%)
  "#,
  )
}

#[test]
fn typos_in_color_sets() {
  test_parse_doc(
    "typos_in_color_sets_1",
    r#"
    a: (light = #ff00ff 30%, dark: #00ff00)
  "#,
  );
  test_parse_doc(
    "typos_in_color_sets_2",
    r#"
    a: (light: #ff00ff 30%; dark: #00ff00)
  "#,
  );
  test_parse_doc(
    "typos_in_color_sets_3",
    r#"
    a: (light: #ff00ff 30%, dark: variable)
  "#,
  );
  test_parse_doc(
    "typos_in_color_sets_4",
    r#"
    a: [light: #ff00ff 30%, dark: #00ff00)
  "#,
  );
  test_parse_doc(
    "typos_in_color_sets_5",
    r#"
    a: (light: #ff00ff 30%, dark: #00ff00]
  "#,
  );
  test_parse_doc(
    "typos_in_color_sets_6",
    r#"
    a: (light: #ff00ff 30%, dark: #00ff00)
    b: (dark: #00ff00, light: #ff00ff 30%)
    c: (dark: .applicationBackgroundLight, light: #ff00ff 30%)
  "#,
  )
}

#[test]
fn document() {
  test_parse_doc(
    "document",
    r#"
    $mediumBright: #aabbcc 33%
    $red: #ff0000
    $white: #ffffff

    ApplicationBackground: (light: $white, dark: #141517)

    Text {
      Primary: (light: #151618, dark: #E7E8EA)
      Secondary: (light: #75767A, dark: #85868A)
    }

    NumericInput {
      ActionKey {
        Background: (light: $red, dark: #ff00ff)
        Highlight: (light: #cccccc, dark: #000000)
      }
    }

    Background: (light: #D6D9DE, dark: #313131)
  "#,
  );
}

fn test_parse_doc(test_name: &str, contents: &str) {
  match parse_document(contents.to_string()) {
    Ok(doc) => assert_debug_snapshot_matches!(test_name, doc),
    Err(e) => assert_snapshot_matches!(test_name, format!("{}", e)),
  }
}
