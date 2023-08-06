use super::super::Error;
use parser::ast::{
  Color as ASTColor, Document as ASTDocument, DocumentItem as ASTDocumentItem,
  RuleSetItem as ASTRuleSetItem, Value as ASTValue,
};
use parser::{ResolvedColorSet, ResolvedVariable, VarContext};
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Identifier {
  pub short: String,
  pub full: String,
  pub depth: usize,
}

#[derive(Debug)]
pub enum RuleSetItem {
  Declaration(Declaration),
  RuleSet(RuleSet),
}

#[derive(Debug)]
pub struct RuleSet {
  pub identifier: Identifier,
  pub items: Vec<RuleSetItem>,
}

#[derive(Debug)]
pub enum DeclarationValue {
  Color(Color),
  ColorSet(ColorSet),
}

#[derive(Debug)]
pub struct Declaration {
  pub identifier: Identifier,
  pub value: DeclarationValue,
}

#[derive(Debug)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: f32,
}

#[derive(Debug)]
pub struct ColorSet {
  pub light: Color,
  pub dark: Color,
}

impl Hash for Color {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.r.hash(state);
    self.g.hash(state);
    self.b.hash(state);
    self.comparable_alpha().hash(state);
  }
}

impl PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.r == other.r
      && self.g == other.g
      && self.b == other.b
      && self.comparable_alpha() == other.comparable_alpha()
  }
}

impl Eq for Color {}

impl Color {
  fn comparable_alpha(&self) -> i32 {
    (self.a * 1000.0) as i32
  }
}

impl RuleSetItem {
  fn identifier(&self) -> &Identifier {
    match self {
      Self::Declaration(decl) => &decl.identifier,
      Self::RuleSet(ruleset) => &ruleset.identifier,
    }
  }
}

impl RuleSet {
  pub fn derive_from(doc: &ASTDocument) -> Result<Self, Error> {
    let ctx = VarContext::derive_from(doc);
    let items: Vec<ASTRuleSetItem> = doc
      .items
      .iter()
      .filter_map(|item| match item {
        ASTDocumentItem::Variable(_) => None,
        ASTDocumentItem::Declaration(decl) => Some(ASTRuleSetItem::Declaration(decl.clone())),
        ASTDocumentItem::RuleSet(ruleset) => Some(ASTRuleSetItem::RuleSet(ruleset.clone())),
      })
      .collect();
    RuleSet::new(Identifier::new(), &items, &ctx)
  }
}

impl From<&ASTColor> for Color {
  fn from(color: &ASTColor) -> Self {
    Color {
      r: color.r,
      g: color.g,
      b: color.b,
      a: color.a,
    }
  }
}

impl From<&ResolvedColorSet> for ColorSet {
  fn from(colorset: &ResolvedColorSet) -> Self {
    ColorSet {
      light: Color::from(&colorset.light),
      dark: Color::from(&colorset.dark),
    }
  }
}

impl Identifier {
  fn new() -> Self {
    Identifier {
      short: "Custom".to_string(),
      full: "".to_string(),
      depth: 1,
    }
  }

  fn appending(&self, identifier: &str) -> Self {
    Identifier {
      short: identifier.to_string(),
      full: format!("{}{}", self.full, identifier),
      depth: self.depth + 1,
    }
  }
}

impl RuleSet {
  fn new<'a>(
    identifier: Identifier,
    items: &[ASTRuleSetItem],
    ctx: &VarContext<'a>,
  ) -> Result<Self, Error> {
    let mut resolved_items: Vec<RuleSetItem> = vec![];

    for item in items {
      match item {
        ASTRuleSetItem::Declaration(decl) => match &decl.value {
          ASTValue::Color(color) => resolved_items.push(RuleSetItem::Declaration(Declaration {
            identifier: identifier.appending(&decl.identifier),
            value: DeclarationValue::Color(color.into()),
          })),
          ASTValue::ColorSet(colorset) => {
            resolved_items.push(RuleSetItem::Declaration(Declaration {
              identifier: identifier.appending(&decl.identifier),
              value: DeclarationValue::ColorSet(
                ctx
                  .resolve_colorset(colorset)
                  .map(|cs| ColorSet::from(&cs))?,
              ),
            }))
          }
          ASTValue::Variable(variable) => match ctx.resolve(&variable)? {
            ResolvedVariable::Color(color) => {
              resolved_items.push(RuleSetItem::Declaration(Declaration {
                identifier: identifier.appending(&decl.identifier),
                value: DeclarationValue::Color(Color::from(&color)),
              }))
            }
            ResolvedVariable::ColorSet(colorset) => {
              resolved_items.push(RuleSetItem::Declaration(Declaration {
                identifier: identifier.appending(&decl.identifier),
                value: DeclarationValue::ColorSet(ColorSet::from(&colorset)),
              }))
            }
          },
        },
        ASTRuleSetItem::RuleSet(ruleset) => {
          resolved_items.push(RuleSetItem::RuleSet(RuleSet::new(
            identifier.appending(&ruleset.identifier),
            &ruleset.items,
            ctx,
          )?))
        }
      }
    }

    resolved_items.sort_by(|a, b| {
      a.identifier()
        .short
        .partial_cmp(&b.identifier().short)
        .unwrap()
    });

    Ok(RuleSet {
      identifier,
      items: resolved_items,
    })
  }
}
