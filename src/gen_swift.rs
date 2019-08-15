use super::ast::{Document, DocumentItem, RuleSet, RuleSetItem};
use std::fs;

pub fn gen_swift(doc: &Document, path: &str) -> std::io::Result<()> {
  let mut identifiers: Vec<String> = vec![];

  for item in doc.items.iter() {
    match item {
      DocumentItem::RuleSet(r) => {
        append_identifiers_from_ruleset(r, "", &mut identifiers);
      }
      _ => {}
    }
  }

  let prefix = r#"
import UIKit

extension UIColor {
  enum Custom {}
}

extension UIColor.Custom {
"#;

  let suffix = r#"
}
"#;

  let mapped_idents: Vec<String> = identifiers
    .iter()
    .map(|ident| {
      format!(
        "  static let {} = UIColor(named: \"{}\")!",
        lowercase_first_letter(ident),
        ident
      )
    })
    .collect();

  let result = format!("{}{}{}", prefix, mapped_idents.join("\n"), suffix);

  fs::write(path, result.as_bytes())
}

fn append_identifiers_from_ruleset(
  ruleset: &RuleSet,
  identifier: &str,
  identifiers_bucket: &mut Vec<String>,
) {
  for item in ruleset.items.iter() {
    match item {
      RuleSetItem::RuleSet(r) => {
        let identifier = format!("{}{}", identifier, r.identifier);
        append_identifiers_from_ruleset(r, &identifier, identifiers_bucket);
      }
      RuleSetItem::Declaration(d) => {
        identifiers_bucket.push(format!("{}{}", identifier, d.identifier));
      }
    }
  }
}

fn lowercase_first_letter(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
  }
}
