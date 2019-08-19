use asset_catalog::{write_asset_catalog, ColorSpace};
use parser::parse_document;
use tempdir::TempDir;

#[test]
fn asset_catalog() {
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

  let document = parse_document(contents.to_string()).expect("Could not parse document");

  let tmp_dir_srgb = TempDir::new("asset_catalog_srgb").expect("Create temp dir failed");
  write_asset_catalog(&document, &tmp_dir_srgb.path(), ColorSpace::SRGB, true)
    .expect("Could not write asset catalog");
  assert!(!dir_diff::is_different(&tmp_dir_srgb.path(), "tests/fixtures/SRGB.xcassets").unwrap());

  let tmp_dir_display_p3 =
    TempDir::new("asset_catalog_display_p3").expect("Create temp dir failed");
  write_asset_catalog(
    &document,
    &tmp_dir_display_p3.path(),
    ColorSpace::DisplayP3,
    true,
  )
  .expect("Could not write asset catalog");
  assert!(!dir_diff::is_different(
    &tmp_dir_display_p3.path(),
    "tests/fixtures/DisplayP3.xcassets"
  )
  .unwrap());

  let tmp_dir_extended_linear_srgb =
    TempDir::new("asset_catalog_extended_linear_srgb").expect("Create temp dir failed");
  write_asset_catalog(
    &document,
    &tmp_dir_extended_linear_srgb.path(),
    ColorSpace::ExtendedRangeLinearSRGB,
    true,
  )
  .expect("Could not write asset catalog");
  assert!(!dir_diff::is_different(
    &tmp_dir_extended_linear_srgb.path(),
    "tests/fixtures/ExtendedRangeLinearSRGB.xcassets"
  )
  .unwrap());

  let tmp_dir_extended_srgb =
    TempDir::new("asset_catalog_extended_srgb").expect("Create temp dir failed");
  write_asset_catalog(
    &document,
    &tmp_dir_extended_srgb.path(),
    ColorSpace::ExtendedRangeSRGB,
    true,
  )
  .expect("Could not write asset catalog");
  assert!(!dir_diff::is_different(
    &tmp_dir_extended_srgb.path(),
    "tests/fixtures/ExtendedRangeSRGB.xcassets"
  )
  .unwrap());
}
