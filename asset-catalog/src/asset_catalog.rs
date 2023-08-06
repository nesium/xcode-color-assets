use super::ColorSpace;
use super::Error;
use parser::ast::{
  Color, ColorSet, Declaration, Document, DocumentItem, RuleSet, RuleSetItem, Value, Variable,
};
use parser::{ResolvedColorSet, ResolvedVariable, VarContext};
use serde_json::json;
use std::fs;
use std::path::Path;

struct Config<'a> {
  color_space: ColorSpace,
  var_lookup: VarContext<'a>,
}

impl<'a> Config<'a> {
  fn resolve_variable(&self, variable: &Variable) -> Result<ResolvedVariable, Error> {
    self.var_lookup.resolve(variable).map_err(|err| err.into())
  }

  fn resolve_colorset(&self, colorset: &ColorSet) -> Result<ResolvedColorSet, Error> {
    self
      .var_lookup
      .resolve_colorset(colorset)
      .map_err(|err| err.into())
  }
}

pub fn write_asset_catalog(
  doc: &Document,
  path: impl AsRef<Path>,
  color_space: ColorSpace,
  delete_directory_if_exists: bool,
) -> Result<(), Error> {
  let path = path.as_ref();

  let config = Config {
    color_space,
    var_lookup: VarContext::derive_from(doc),
  };

  if path.exists() {
    if delete_directory_if_exists {
      fs::remove_dir_all(&path)?;
    } else {
      return Err(Error::CatalogExists { path: path.into() });
    }
  }
  fs::create_dir_all(&path)?;

  for item in doc.items.iter() {
    match item {
      DocumentItem::RuleSet(r) => {
        write_ruleset(r, &path, &r.identifier, &config)?;
      }
      DocumentItem::Declaration(d) => {
        write_declaration(d, &path, &d.identifier, &config)?;
      }
      DocumentItem::Variable(_) => {}
    }
  }

  Ok(())
}

fn write_ruleset(
  ruleset: &RuleSet,
  path: impl AsRef<Path>,
  identifier: &str,
  config: &Config,
) -> Result<(), Error> {
  let ruleset_path = path.as_ref().join(&ruleset.identifier);
  fs::create_dir(&ruleset_path)?;

  let info = json!({
    "info": {
      "version": 1,
      "author": "xcode"
    }
  });

  let json_path = ruleset_path.join("Contents.json");

  fs::write(
    &json_path,
    serde_json::to_string_pretty(&info).unwrap().as_bytes(),
  )?;

  for item in ruleset.items.iter() {
    match item {
      RuleSetItem::RuleSet(r) => {
        let child_identifier = format!("{}{}", identifier, r.identifier);
        write_ruleset(r, &ruleset_path, &child_identifier, &config)?;
      }
      RuleSetItem::Declaration(d) => {
        let child_identifier = format!("{}{}", identifier, d.identifier);
        write_declaration(d, &ruleset_path, &child_identifier, &config)?;
      }
    }
  }

  Ok(())
}

fn write_declaration(
  declaration: &Declaration<Value>,
  path: impl AsRef<Path>,
  identifier: &str,
  config: &Config,
) -> Result<(), Error> {
  let colorset_path = path
    .as_ref()
    .join(identifier.to_string())
    .with_extension("colorset");

  fs::create_dir(&colorset_path)?;

  let mut info: serde_json::value::Value = json!({
    "info": {
      "version": 1,
      "author": "xcode"
    },
    "colors": []
  });

  let components = |color: &Color| {
    json!({
      "red": format!("0x{:02X}", color.r),
      "green": format!("0x{:02X}", color.g),
      "blue": format!("0x{:02X}", color.b),
      "alpha": format!("{:.3}", color.a)
    })
  };

  let append_light_color =
    |value: &mut serde_json::value::Value, color: &Color| -> Result<(), Error> {
      let arr = value["colors"].as_array_mut().unwrap();
      arr.push(json!({
        "idiom": "universal",
        "color": {
          "color-space" : config.color_space.to_string(),
          "components": components(color)
        }
      }));
      Ok(())
    };

  let append_dark_color =
    |value: &mut serde_json::value::Value, color: &Color| -> Result<(), Error> {
      let arr = value["colors"].as_array_mut().unwrap();
      arr.push(json!({
        "idiom" : "universal",
        "appearances" : [{
          "appearance" : "luminosity",
          "value" : "dark"
        }],
        "color" : {
          "color-space" : config.color_space.to_string(),
          "components" : components(color)
        }
      }));
      Ok(())
    };

  let append_colorset =
    |value: &mut serde_json::value::Value, colorset: &ResolvedColorSet| -> Result<(), Error> {
      append_light_color(value, &colorset.light)?;
      append_dark_color(value, &colorset.dark)?;
      Ok(())
    };

  match declaration.value {
    Value::Variable(ref identifier) => match config.resolve_variable(identifier)? {
      ResolvedVariable::Color(color) => append_light_color(&mut info, &color)?,
      ResolvedVariable::ColorSet(colorset) => append_colorset(&mut info, &colorset)?,
    },
    Value::Color(ref color) => append_light_color(&mut info, color)?,
    Value::ColorSet(ref colorset) => {
      append_colorset(&mut info, &config.resolve_colorset(colorset)?)?
    }
  }

  let json_path = colorset_path.join("Contents.json");

  fs::write(
    &json_path,
    serde_json::to_string_pretty(&info).unwrap().as_bytes(),
  )?;

  Ok(())
}
