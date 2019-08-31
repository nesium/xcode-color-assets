use super::data::RuleSet;

pub struct RendererConfig {
  tab: String
}

impl RendererConfig {
  pub fn new(tab: &str) -> Self {
    RendererConfig {
      tab: tab.to_owned()
    }
  }

  pub fn indent(&self, depth: usize) -> String {
    self.tab.repeat(depth)
  }
}

pub trait Renderer {
  fn render_into(&self, ruleset: &RuleSet, destination: &mut String, config: &RendererConfig);
}