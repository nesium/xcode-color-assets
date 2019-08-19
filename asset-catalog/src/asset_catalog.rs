use super::Error;
use parser::ast::{
  Color, ColorSet, ColorSetValue, Declaration, Document, DocumentItem, RuleSet, RuleSetItem, Value,
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

pub fn write_asset_catalog(
  doc: &Document,
  path: &Path,
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
        write_ruleset(r, &path, &r.identifier, &var_lookup)?;
      }
      DocumentItem::Declaration(d) => {
        write_declaration(d, &path, &d.identifier, &var_lookup)?;
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
  var_lookup: &HashMap<String, &Value>,
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
        write_ruleset(r, &ruleset_path, &child_identifier, &var_lookup)?;
      }
      RuleSetItem::Declaration(d) => {
        let child_identifier = format!("{}{}", identifier, d.identifier);
        write_declaration(d, &ruleset_path, &child_identifier, &var_lookup)?;
      }
    }
  }

  Ok(())
}

fn write_declaration(
  declaration: &Declaration<Value>,
  path: &Path,
  identifier: &str,
  var_lookup: &HashMap<String, &Value>,
) -> Result<(), Error> {
  let colorset_path = path.join(identifier.to_string()).with_extension("colorset");

  fs::create_dir(&colorset_path)
    .map_err(|_| Error::CouldNotCreateDirectory(path_to_str(&colorset_path)))?;

  enum ResolvedVariable {
    Color(Color),
    ColorSet(ColorSet),
  }

  fn resolve_variable<F>(identifier: &str, var_lookup: &HashMap<String, &Value>, mut resolver: F)
  where
    F: FnMut(&ResolvedVariable) -> (),
  {
    let value = var_lookup
      .get(identifier)
      .expect(&format!("Referenced undefined variable {}", identifier));
    match value {
      Value::Variable(identifier) => resolve_variable(identifier, var_lookup, resolver),
      Value::Color(color) => resolver(&ResolvedVariable::Color(color.clone())),
      Value::ColorSet(colorset) => resolver(&ResolvedVariable::ColorSet(colorset.clone())),
    }
  };

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

  let append_light_color = |value: &mut serde_json::value::Value, color: &Color| {
    let arr = value["colors"].as_array_mut().unwrap();
    arr.push(json!({
      "idiom": "universal",
      "color": {
        "color-space" : "srgb",
        "components": components(color)
      }
    }));
  };

  let append_dark_color = |value: &mut serde_json::value::Value, color: &Color| {
    let arr = value["colors"].as_array_mut().unwrap();
    arr.push(json!({
      "idiom" : "universal",
      "appearances" : [{
        "appearance" : "luminosity",
        "value" : "dark"
      }],
      "color" : {
        "color-space" : "srgb",
        "components" : components(color)
      }
    }));
  };

  let append_colorset = |value: &mut serde_json::value::Value, colorset: &ColorSet| {
    match colorset.light {
      ColorSetValue::Variable(ref identifier) => {
        resolve_variable(identifier, var_lookup, |result| match result {
          ResolvedVariable::Color(color) => append_light_color(value, color),
          ResolvedVariable::ColorSet(_) => panic!(format!(
            "Attempt to assign a colorset to the light value of another colorset via variable {}",
            identifier
          )),
        });
      }
      ColorSetValue::Color(ref color) => append_light_color(value, color),
    }

    match colorset.dark {
      ColorSetValue::Variable(ref identifier) => {
        resolve_variable(identifier, var_lookup, |result| match result {
          ResolvedVariable::Color(color) => append_dark_color(value, color),
          ResolvedVariable::ColorSet(_) => panic!(format!(
            "Attempt to assign a colorset to the dark value of another colorset via variable {}",
            identifier
          )),
        });
      }
      ColorSetValue::Color(ref color) => append_dark_color(value, color),
    }
  };

  match declaration.value {
    Value::Variable(ref identifier) => {
      resolve_variable(identifier, var_lookup, |result| match result {
        ResolvedVariable::Color(color) => append_light_color(&mut info, color),
        ResolvedVariable::ColorSet(colorset) => append_colorset(&mut info, colorset),
      });
    }
    Value::Color(ref color) => append_light_color(&mut info, color),
    Value::ColorSet(ref colorset) => append_colorset(&mut info, colorset),
  }

  let json_path = colorset_path.join("Contents.json");

  fs::write(
    &json_path,
    serde_json::to_string_pretty(&info).unwrap().as_bytes(),
  )
  .map_err(|_| Error::CouldNotCreateFile(path_to_str(&json_path)))
}
