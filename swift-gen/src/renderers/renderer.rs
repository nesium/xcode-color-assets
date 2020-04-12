use super::super::AccessLevel;
use super::data::RuleSet;

pub struct RendererConfig {
  tab: String,
  pub access_level: AccessLevel,
}

impl RendererConfig {
  pub fn new(tab: &str, access_level: AccessLevel) -> Self {
    RendererConfig {
      tab: tab.to_owned(),
      access_level,
    }
  }

  pub fn indent(&self, depth: usize) -> String {
    self.tab.repeat(depth)
  }
}

pub trait Renderer {
  fn render_into(&self, ruleset: &RuleSet, destination: &mut String, config: &RendererConfig);
}
