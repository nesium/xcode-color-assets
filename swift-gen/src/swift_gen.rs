use super::renderers::{
  data::RuleSet as RendererRuleSet, ColorSetRenderer, Renderer, RendererConfig, DynamicColorRenderer
};
use super::Error;
use parser::ast::Document;
use std::fs;
use std::io::prelude::Read;
use std::path::Path;

pub enum RenderMode {
  ColorSet,
  DynamicColor,
}

pub fn gen_swift(
  doc: &Document,
  path: &Path,
  mode: RenderMode,
  force_overwrite: bool,
) -> Result<(), Error> {
  let root = RendererRuleSet::derive_from(doc)?;

  let mut contents = String::new();
  let config = RendererConfig::new("  ");

  let renderer: Box<dyn Renderer> = match mode {
    RenderMode::ColorSet => Box::new(ColorSetRenderer {}), 
    RenderMode::DynamicColor => Box::new(DynamicColorRenderer {})
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
