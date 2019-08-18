use parser::parse_document;
use swift_gen::gen_swift;
use tempdir::TempDir;

#[test]
fn asset_catalog() {
  let contents = r#"
    $white: #ffffff
    $black: #000000

    $brightAccent: #5753CF

    $mediumBright: #19C8FF
    $mediumBrightHighlight: #70D1FA

    $grey1: $black

    Text {
      Primary: (light: #151618, dark: #E7E8EA)
      Secondary: (light: $grey1, dark: #85868A)
    }

    LightContentSeparator: (light: #F1F2F2, dark: #222525)

    NumericInput {
      NumericKey {
        Background: (light: $white, dark: #434343)
        Highlight: (light: #C4CCDA, dark: #666666)
        Shadow: (light: #848587, dark: $black)
        Text: (light: $black, dark: $white)
      }

      DoneKey {
        Background: (light: $mediumBright, dark: $brightAccent)
        Highlight: (light: $mediumBrightHighlight, dark: #677ADB)
        Shadow: (light: #6E7073, dark: $black)
        Text: (light: $black, dark: $white)
      }

      Background: (light: #D6D9DE 30%, dark: #313131 40%)
    }
  "#;

  let tmp_dir = TempDir::new("swift_gen").expect("Create temp dir failed");
  let document = parse_document(contents.to_string()).expect("Could not parse document");

  gen_swift(&document, &tmp_dir.path().join("UIColor+Custom.swift"))
    .expect("Could not write Swift file");
  assert!(!dir_diff::is_different(&tmp_dir.path(), "tests/fixtures").unwrap());
}
