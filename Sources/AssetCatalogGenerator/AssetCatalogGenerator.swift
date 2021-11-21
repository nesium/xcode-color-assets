import ColorConfigParser
import Foundation
import Stencil

public enum Colorspace: String {
  case displayP3 = "display-p3"
  case extendedRangeLinearSRGB = "extended-linear-srgb"
  case extendedRangeSRGB = "extended-srgb"
  case sRGB = "srgb"
}

public func generateAssetCatalog(
  from doc: Document,
  at url: URL,
  using colorSpace: Colorspace,
  context: VariableContext
) throws {
  if FileManager.default.fileExists(atPath: url.path) {
    try FileManager.default.removeItem(at: url)
  }

  try FileManager.default.createDirectory(at: url, withIntermediateDirectories: true)

  for item in doc.items {
    switch item {
    case let .declaration(declaration):
      try write(
        declaration: declaration,
        at: url,
        with: declaration.identifier,
        using: colorSpace,
        context: context
      )
    case let .ruleSet(ruleSet):
      try write(
        ruleSet: ruleSet,
        at: url,
        with: ruleSet.identifier,
        using: colorSpace,
        context: context
      )
    case .variable:
      break
    }
  }
}

private func write(
  ruleSet: RuleSet,
  at url: URL,
  with identifier: String,
  using colorSpace: Colorspace,
  context: VariableContext
) throws {
  let ruleSetURL = url.appendingPathComponent(ruleSet.identifier)

  try FileManager.default.createDirectory(at: ruleSetURL, withIntermediateDirectories: false)

  let json = """
  {
    "info": {
      "version": 1,
      "author": "xcode"
    }
  }
  """

  let jsonURL = ruleSetURL.appendingPathComponent("Contents.json")

  try json.write(toFile: jsonURL.path, atomically: true, encoding: .utf8)

  for item in ruleSet.items {
    switch item {
    case let .ruleSet(ruleSet):
      try write(
        ruleSet: ruleSet,
        at: ruleSetURL,
        with: "\(identifier)\(ruleSet.identifier)",
        using: colorSpace,
        context: context
      )
    case let .declaration(declaration):
      try write(
        declaration: declaration,
        at: ruleSetURL,
        with: "\(identifier)\(declaration.identifier)",
        using: colorSpace,
        context: context
      )
    }
  }
}

private func write(
  declaration: Declaration<Value>,
  at url: URL,
  with identifier: String,
  using colorSpace: Colorspace,
  context: VariableContext
) throws {
  let colorSetURL = url.appendingPathComponent(identifier).appendingPathExtension("colorset")

  try FileManager.default.createDirectory(at: colorSetURL, withIntermediateDirectories: false)

  let template = """
  {
    "colors": [
      {
        "color": {
          "color-space": "{{ colorSet.colorSpace }}",
          "components": {
            "alpha": "{{ colorSet.light.alpha }}",
            "blue": "{{ colorSet.light.blue }}",
            "green": "{{ colorSet.light.green }}",
            "red": "{{ colorSet.light.red }}"
          }
        },
        "idiom": "universal"
      }{% if colorSet.dark %},
      {
        "appearances": [
          {
            "appearance": "luminosity",
            "value": "dark"
          }
        ],
        "color": {
          "color-space": "{{ colorSet.colorSpace }}",
          "components": {
            "alpha": "{{ colorSet.dark.alpha }}",
            "blue": "{{ colorSet.dark.blue }}",
            "green": "{{ colorSet.dark.green }}",
            "red": "{{ colorSet.dark.red }}"
          }
        },
        "idiom": "universal"
      }{% endif %}
    ],
    "info": {
      "author": "xcode",
      "version": 1
    }
  }
  """

  struct TemplateData {
    struct Color {
      var red: String
      var green: String
      var blue: String
      var alpha: String

      init(color: ColorConfigParser.Color) {
        self.red = String(format: "0x%X", color.red)
        self.green = String(format: "0x%X", color.green)
        self.blue = String(format: "0x%X", color.blue)
        self.alpha = String(format: "%.02f", color.alpha)
      }
    }

    var colorSpace: String
    var light: Color
    var dark: Color?
  }

  let templateData: TemplateData

  switch declaration.value {
  case let .color(color):
    templateData = .init(colorSpace: colorSpace.rawValue, light: .init(color: color), dark: nil)
  case let .colorSet(colorSet):
    let resolvedColorSet = try context.resolve(colorSet: colorSet)
    templateData = .init(
      colorSpace: colorSpace.rawValue,
      light: .init(color: resolvedColorSet.light),
      dark: .init(color: resolvedColorSet.dark)
    )
  case let .variable(variable):
    let resolvedVariable = try context.resolve(variable: variable)
    switch resolvedVariable {
    case let .color(color):
      templateData = .init(colorSpace: colorSpace.rawValue, light: .init(color: color), dark: nil)
    case let .colorSet(resolvedColorSet):
      templateData = .init(
        colorSpace: colorSpace.rawValue,
        light: .init(color: resolvedColorSet.light),
        dark: .init(color: resolvedColorSet.dark)
      )
    }
  }

  let json = try Environment().renderTemplate(string: template, context: ["colorSet": templateData])
  let jsonURL = colorSetURL.appendingPathComponent("Contents.json")
  try json.write(toFile: jsonURL.path, atomically: true, encoding: .utf8)
}
