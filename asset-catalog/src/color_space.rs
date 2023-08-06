use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, Clone, PartialEq)]
pub enum ColorSpace {
  DisplayP3,
  SRGB,
  ExtendedRangeSRGB,
  ExtendedRangeLinearSRGB,
}

impl FromStr for ColorSpace {
  type Err = ();

  fn from_str(s: &str) -> Result<ColorSpace, ()> {
    match s.to_lowercase().as_ref() {
      "display-p3" => Ok(ColorSpace::DisplayP3),
      "srgb" => Ok(ColorSpace::SRGB),
      "extended-srgb" => Ok(ColorSpace::ExtendedRangeSRGB),
      "extended-linear-srgb" => Ok(ColorSpace::ExtendedRangeLinearSRGB),
      _ => Err(()),
    }
  }
}

impl ToString for ColorSpace {
  fn to_string(&self) -> String {
    match self {
      Self::DisplayP3 => "display-p3".to_string(),
      Self::SRGB => "srgb".to_string(),
      Self::ExtendedRangeSRGB => "extended-srgb".to_string(),
      Self::ExtendedRangeLinearSRGB => "extended-linear-srgb".to_string(),
    }
  }
}
