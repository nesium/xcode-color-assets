#[derive(Debug, PartialEq)]
pub struct Document {
  pub items: Vec<DocumentItem>,
}

#[derive(Debug, PartialEq)]
pub enum DocumentItem {
  Variable(Declaration<Value>),
  RuleSet(RuleSet),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Variable(String),
  Color(Color),
  ColorSet(ColorSet),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorSetValue {
  Variable(String),
  Color(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorSet {
  pub light: ColorSetValue,
  pub dark: ColorSetValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration<T> {
  pub identifier: String,
  pub value: T,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuleSet {
  pub identifier: String,
  pub items: Vec<RuleSetItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleSetItem {
  RuleSet(RuleSet),
  Declaration(Declaration<Value>),
}

impl Into<Option<Color>> for Value {
  fn into(self) -> Option<Color> {
    match self {
      Value::Color(c) => Some(c),
      _ => None,
    }
  }
}

impl Into<Value> for ColorSetValue {
  fn into(self) -> Value {
    match self {
      ColorSetValue::Color(c) => Value::Color(c),
      ColorSetValue::Variable(v) => Value::Variable(v),
    }
  }
}
