import Foundation
import Parsing

public func parseDocument(input: String) throws -> Document? {
  try document.parse(input)
}

private typealias Input = String.SubSequence.UTF8View

private let document = Parse(Document.init(items:)) {
  Skip {
    Whitespace()
  }
  Many {
    OneOf {
      variable.map(DocumentItem.variable)
      ruleSet().map(DocumentItem.ruleSet)
      declaration(value).map(DocumentItem.declaration)
    }
  } separator: {
    lineDelimiter
  }
  Skip {
    Whitespace()
  }
  End()
}

private func ruleSet() -> AnyParser<Input, RuleSet> {
  var ruleSet: AnyParser<Input, RuleSet>!

  ruleSet = Parse(RuleSet.init(identifier:items:)) {
    identifier
    Skip {
      Whitespace()
      "{".utf8
      Whitespace()
    }
    Lazy {
      Many {
        OneOf {
          ruleSet.map(RuleSetItem.ruleSet)
          declaration(value).map(RuleSetItem.declaration)
        }
      } separator: {
        lineDelimiter
      }
    }
    Skip {
      Whitespace()
      "}".utf8
    }
  }.eraseToAnyParser()

  return ruleSet
}

private func declaration<P: Parser>(
  _ parser: P
) -> AnyParser<Input, Declaration<P.Output>> where P.Input == Input {
  Parse(Declaration.init(identifier:value:)) {
    identifier
    Skip {
      Whitespace()
      ":".utf8
      Whitespace()
    }
    parser
  }.eraseToAnyParser()
}

private let variable = Parse(Declaration.init(identifier:value:)) {
  variableIdentifier
  Skip {
    Whitespace()
    ":".utf8
    Whitespace()
  }
  value
}

private let value = OneOf {
  colorSetValue.map(Value.colorSetValue)
  colorSet.compactMap(Value.colorSet)
}

private extension Value {
  static func colorSetValue(_ value: ColorSetValue) -> Value {
    switch value {
    case let .color(color):
      return .color(color)
    case let .variable(variable):
      return .variable(variable)
    }
  }
}

private let colorSet = Parse {
  Skip {
    "(".utf8
    Whitespace()
  }
  declaration(colorSetValue)
  listSeparator
  declaration(colorSetValue)
  Skip {
    Whitespace()
    ")".utf8
  }
}.compactMap(ColorSet.init)

private extension ColorSet {
  init?(_ decl1: Declaration<ColorSetValue>, _ decl2: Declaration<ColorSetValue>) {
    switch (decl1.identifier, decl2.identifier) {
    case ("light", "dark"):
      self = ColorSet(light: decl1.value, dark: decl2.value)
    case ("dark", "light"):
      self = ColorSet(light: decl2.value, dark: decl1.value)
    default:
      return nil
    }
  }
}

private let colorSetValue = OneOf {
  hexColor.map(ColorSetValue.color)
  rgbaColor.map(ColorSetValue.color)
  variableValue.map(ColorSetValue.variable)
}

private let hexColor = Parse(Color.init(red:green:blue:alpha:)) {
  "#".utf8
  hexValue
  hexValue
  hexValue
  Optionally {
    " ".utf8
    Skip {
      Whitespace()
    }
    alphaValue
  }.map { $0 ?? Float(1) }
}

let rgbaColor = Parse {
  "rgba(".utf8
  Parse(Color.init(red:green:blue:alpha:)) {
    Parse {
      UInt8.parser()
      listSeparator
    }
    Parse {
      UInt8.parser()
      listSeparator
    }
    Parse {
      UInt8.parser()
      listSeparator
    }
    Float.parser()
  }
  Skip {
    Whitespace()
  }
  ")".utf8
}

private let lineDelimiter = Skip {
  "\n".utf8
  Whitespace()
}

private let listSeparator = Skip {
  Whitespace()
  ",".utf8
  Whitespace()
}

private let variableValue = Parse(Variable.init(identifier:opacity:)) {
  variableIdentifier
  Optionally {
    " ".utf8
    alphaValue
  }.map { $0 ?? Float(1) }
}

private let variableIdentifier = Parse {
  "$".utf8
  identifier
}

private let identifier = Prefix<Input>(1...) { c in
  (c >= UInt8(ascii: "0") && c <= UInt8(ascii: "9")) ||
    (c >= UInt8(ascii: "a") && c <= UInt8(ascii: "z")) ||
    (c >= UInt8(ascii: "A") && c <= UInt8(ascii: "Z"))
}.compactMap(String.init)

private let hexValue = Prefix<Input>(2)
  .compactMap { UInt8(String(decoding: $0, as: UTF8.self), radix: 16) }

private let alphaValue = Parse {
  Int.parser(of: Input.self, isSigned: false)
  "%".utf8
}.map { Float($0) / 100 }
