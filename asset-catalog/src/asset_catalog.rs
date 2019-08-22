use super::ColorSpace;
use super::Error;
use parser::ast::{
  Color, ColorSet, ColorSetValue, Declaration, Document, DocumentItem, RuleSet, RuleSetItem, Value,
  Variable,
};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn path_to_str(path: &Path) -> String {
  path
    .file_name()
    .and_then(|s| s.to_str())
    .map(String::from)
    .unwrap()
}

enum ResolvedVariable<'a> {
  Color(Color),
  ColorSet(&'a ColorSet),
}

struct Config<'a> {
  color_space: ColorSpace,
  var_lookup: HashMap<String, &'a Value>,
}

impl<'a> Config<'a> {
  fn resolve_variable(&self, variable: &Variable) -> Result<ResolvedVariable<'a>, Error> {
    let value = match self.var_lookup.get(&variable.identifier) {
      Some(value) => value,
      None => return Err(Error::UnknownIdentifier(variable.identifier.to_string())),
    };
    match value {
      Value::Variable(identifier) => self.resolve_variable(identifier),
      Value::Color(color) => Ok(ResolvedVariable::Color(variable.resolve_against(color))),
      Value::ColorSet(colorset) => Ok(ResolvedVariable::ColorSet(colorset)),
    }
  }
}

pub fn write_asset_catalog(
  doc: &Document,
  path: &Path,
  color_space: ColorSpace,
  delete_directory_if_exists: bool,
) -> Result<(), Error> {
  let var_lookup: HashMap<String, &Value> = doc
    .items
    .iter()
    .filter_map(|item| match item {
      DocumentItem::Variable(v) => Some(v),
      _ => None,
    })
    .map(|variable| (variable.identifier.to_string(), &variable.value))
    .collect();

  let config = Config {
    color_space,
    var_lookup,
  };

  if path.exists() {
    if delete_directory_if_exists {
      fs::remove_dir_all(&path).map_err(|_| Error::CouldNotRemoveDirectory(path_to_str(&path)))?;
    } else {
      return Err(Error::CatalogExists(path_to_str(path)));
    }
  }
  fs::create_dir_all(&path).map_err(|_| Error::CouldNotCreateDirectory(path_to_str(&path)))?;

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
  path: &Path,
  identifier: &str,
  config: &Config,
) -> Result<(), Error> {
  let ruleset_path = path.join(&ruleset.identifier);
  fs::create_dir(&ruleset_path)
    .map_err(|_| Error::CouldNotCreateDirectory(path_to_str(&ruleset_path)))?;

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
  )
  .map_err(|_| Error::CouldNotCreateFile(path_to_str(&json_path)))?;

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
  path: &Path,
  identifier: &str,
  config: &Config,
) -> Result<(), Error> {
  let colorset_path = path.join(identifier.to_string()).with_extension("colorset");

  fs::create_dir(&colorset_path)
    .map_err(|_| Error::CouldNotCreateDirectory(path_to_str(&colorset_path)))?;

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
    |value: &mut serde_json::value::Value, colorset: &ColorSet| -> Result<(), Error> {
      match colorset.light {
        ColorSetValue::Variable(ref variable) => match config.resolve_variable(variable)? {
          ResolvedVariable::Color(color) => append_light_color(value, &color)?,
          ResolvedVariable::ColorSet(_) => {
            return Err(Error::AssignColorSetToLightProperty(
              variable.identifier.to_string(),
            ))
          }
        },
        ColorSetValue::Color(ref color) => append_light_color(value, color)?,
      }

      match colorset.dark {
        ColorSetValue::Variable(ref variable) => match config.resolve_variable(variable)? {
          ResolvedVariable::Color(color) => append_dark_color(value, &color)?,
          ResolvedVariable::ColorSet(_) => {
            return Err(Error::AssignColorSetToDarkProperty(
              variable.identifier.to_string(),
            ))
          }
        },
        ColorSetValue::Color(ref color) => append_dark_color(value, color)?,
      }

      Ok(())
    };

  match declaration.value {
    Value::Variable(ref identifier) => match config.resolve_variable(identifier)? {
      ResolvedVariable::Color(color) => append_light_color(&mut info, &color)?,
      ResolvedVariable::ColorSet(colorset) => append_colorset(&mut info, colorset)?,
    },
    Value::Color(ref color) => append_light_color(&mut info, color)?,
    Value::ColorSet(ref colorset) => append_colorset(&mut info, colorset)?,
  }

  let json_path = colorset_path.join("Contents.json");

  fs::write(
    &json_path,
    serde_json::to_string_pretty(&info).unwrap().as_bytes(),
  )
  .map_err(|_| Error::CouldNotCreateFile(path_to_str(&json_path)))
}
