use parser::{ast::Document, parse_document};
use std::path::Path;
use swift_gen::{gen_swift, Error};
use tempdir::TempDir;

#[test]
fn generate_swift_file() {
  let tmp_dir = TempDir::new("generate_swift_file").expect("Create temp dir failed");

  gen_swift(
    &test_document(),
    &tmp_dir.path().join("UIColor+Custom.swift"),
    true,
  )
  .expect("Could not write Swift file");
  assert!(!dir_diff::is_different(&tmp_dir.path(), "tests/fixtures").unwrap());
}

#[test]
fn do_not_touch_identical_file() {
  let tmp_dir = TempDir::new("do_not_touch_identical_file").expect("Create temp dir failed");
  let fixture_path = "tests/fixtures/UIColor+Custom.swift";
  let tmp_path = tmp_dir.path().join("UIColor+Custom.swift");

  std::fs::copy(&fixture_path, &tmp_path).expect("Could not copy file");
  assert!(
    is_modification_date_equal(&fixture_path, &tmp_path),
    "Expected modification date to be equal after copy."
  );

  match gen_swift(&test_document(), &tmp_path, false) {
    Err(Error::FileIsIdentical(path)) => {
      assert_eq!(std::path::Path::new(&path), tmp_path);
      assert!(
        is_modification_date_equal(&fixture_path, &tmp_path),
        "Expected modification date to be equal after swift_gen"
      );
    }
    Err(Error::IO(msg)) => panic!("Unexpected error {}", msg),
    Ok(()) => panic!("Expected Err, got Ok"),
  }

  gen_swift(&test_document(), &tmp_path, true).expect("Could not write Swift file");
  assert!(
    !is_modification_date_equal(&fixture_path, &tmp_path),
    "Expected modification date to differ after swift_gen"
  );
}

fn test_document() -> Document {
  let contents = r#"
    $white: #ffffff
    $black: #000000
    $classic: (light: $black, dark: $white)

    $brightAccent: #5753CF
    $mediumBright: rgba(25, 200, 255, 1)
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
        Text: $classic
      }

      DoneKey {
        Background: (light: $mediumBright, dark: $brightAccent)
        Highlight: (light: $mediumBrightHighlight, dark: rgba(103, 122, 219, 1))
        Shadow: (light: #6E7073, dark: $black)
        Text: $classic
      }

      Background: (light: #D6D9DE 30%, dark: #313131 40%)
    }
  "#;

  parse_document(contents.to_string()).expect("Could not parse document")
}

fn is_modification_date_equal<P1: AsRef<Path>, P2: AsRef<Path>>(p1: P1, p2: P2) -> bool {
  let old_metadata = std::fs::metadata(p1).expect("Could not read metadata of file 1");
  let new_metadata = std::fs::metadata(p2).expect("Could not read metadata of file 2");

  old_metadata
    .modified()
    .expect("Could not retrieve modification date of file 1")
    == new_metadata
      .modified()
      .expect("Could not retrieve modification date of file 2")
}
