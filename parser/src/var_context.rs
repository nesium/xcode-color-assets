use super::ast::{Color, ColorSet, ColorSetValue, Document, DocumentItem, Value, Variable};
use super::error::Error;
use std::collections::HashMap;

pub struct VarContext<'a> {
  map: HashMap<String, &'a Value>,
}

pub enum ResolvedVariable {
  Color(Color),
  ColorSet(ResolvedColorSet),
}

pub struct ResolvedColorSet {
  pub light: Color,
  pub dark: Color,
}

impl<'a> VarContext<'a> {
  pub fn resolve(&self, variable: &Variable) -> Result<ResolvedVariable, Error> {
    let value = match self.map.get(&variable.identifier) {
      Some(value) => value,
      None => {
        return Err(Error::new(format!(
          "Could not find variable with identifier {}.",
          variable.identifier
        )))
      }
    };
    match value {
      Value::Variable(identifier) => self.resolve(identifier),
      Value::Color(color) => Ok(ResolvedVariable::Color(variable.resolve_against(color))),
      Value::ColorSet(colorset) => self
        .resolve_colorset(colorset)
        .map(ResolvedVariable::ColorSet),
    }
  }

  pub fn resolve_colorset(&self, colorset: &ColorSet) -> Result<ResolvedColorSet, Error> {
    let light: Color = match &colorset.light {
      ColorSetValue::Color(color) => Ok(color.clone()),
      ColorSetValue::Variable(ref light_variable) => match self.resolve(light_variable)? {
        ResolvedVariable::Color(color) => Ok(color),
        ResolvedVariable::ColorSet(_) => Err(Error::new(format!(
          "Attempt to assign a colorset to the light property of another colorset via variable {}.",
          light_variable.identifier
        ))),
      },
    }?;

    let dark = match &colorset.dark {
      ColorSetValue::Color(color) => Ok(color.clone()),
      ColorSetValue::Variable(ref dark_variable) => match self.resolve(dark_variable)? {
        ResolvedVariable::Color(color) => Ok(color),
        ResolvedVariable::ColorSet(_) => Err(Error::new(format!(
          "Attempt to assign a colorset to the dark property of another colorset via variable {}.",
          dark_variable.identifier
        ))),
      },
    }?;

    Ok(ResolvedColorSet { light, dark })
  }
}

impl<'a> VarContext<'a> {
  pub fn derive_from(doc: &'a Document) -> Self {
    let map: HashMap<String, &Value> = doc
      .items
      .iter()
      .filter_map(|item| match item {
        DocumentItem::Variable(v) => Some(v),
        _ => None,
      })
      .map(|variable| (variable.identifier.to_string(), &variable.value))
      .collect();
    VarContext { map }
  }
}
