use super::renderers::{
  data::RuleSet as RendererRuleSet, ColorSetRenderer, DynamicColorRenderer, Renderer,
  RendererConfig,
};
use super::Error;
use parser::ast::Document;
use std::fs;
use std::io::prelude::Read;
use std::path::Path;
use std::str::FromStr;

pub enum RenderMode {
  ColorSet,
  DynamicColor,
}

impl FromStr for RenderMode {
  type Err = ();

  fn from_str(s: &str) -> Result<RenderMode, ()> {
    match s.to_lowercase().as_ref() {
      "asset-catalog" => Ok(RenderMode::ColorSet),
      "dynamic-color" => Ok(RenderMode::DynamicColor),
      _ => Err(()),
    }
  }
}

pub enum AccessLevel {
  Internal,
  Public,
}

impl FromStr for AccessLevel {
  type Err = ();

  fn from_str(s: &str) -> Result<AccessLevel, ()> {
    match s.to_lowercase().as_ref() {
      "internal" => Ok(AccessLevel::Internal),
      "public" => Ok(AccessLevel::Public),
      _ => Err(()),
    }
  }
}

impl ToString for AccessLevel {
  fn to_string(&self) -> String {
    match self {
      AccessLevel::Internal => "".to_string(),
      AccessLevel::Public => "public ".to_string(),
    }
  }
}

pub fn gen_swift(
  doc: &Document,
  path: &Path,
  mode: RenderMode,
  force_overwrite: bool,
  access_level: AccessLevel,
) -> Result<(), Error> {
  let root = RendererRuleSet::derive_from(doc)?;

  let mut contents = String::new();
  let config = RendererConfig::new("  ", access_level);

  let renderer: Box<dyn Renderer> = match mode {
    RenderMode::ColorSet => Box::new(ColorSetRenderer {}),
    RenderMode::DynamicColor => Box::new(DynamicColorRenderer {}),
  };
  renderer.render_into(&root, &mut contents, &config);

  let data = contents.as_bytes();

  if !force_overwrite && path.exists() {
    let mut existing_data = Vec::new();
    let mut existing_file = fs::File::open(&path)?;
    existing_file.read_to_end(&mut existing_data)?;

    if existing_data == data {
      return Err(Error::FileIsIdentical(path.to_str().unwrap().to_string()));
    }
  }

  fs::write(path, data)?;

  Ok(())
}
