import Foundation
import Parsing

public func parseDocument(input: String) -> Document? {
  document.parse(input)
}

private typealias Input = String.SubSequence.UTF8View

private let document = Skip(Whitespace())
  .take(Many(documentItem, separator: lineDelimiter))
  .map(Document.init)

private let documentItem =
  variable.map(DocumentItem.declaration)
    .orElse(ruleSet().map(DocumentItem.ruleSet))
    .orElse(declaration(value).map(DocumentItem.declaration))

private func ruleSet() -> AnyParser<Input, RuleSet> {
  var ruleSetItem: AnyParser<Input, [RuleSetItem]>!
  let ruleSet = identifier
    .skip(Whitespace())
    .take(Lazy { ruleSetItem })
    .map(RuleSet.init)
    .eraseToAnyParser()

  ruleSetItem = Skip("{".utf8)
    .skip(Whitespace())
    .take(
      Many(
        ruleSet.map(RuleSetItem.ruleSet)
          .orElse(declaration(value).map(RuleSetItem.declaration)),
        separator: lineDelimiter
      )
    )
    .skip(Whitespace())
    .skip("}".utf8)
    .eraseToAnyParser()

  return ruleSet
}

private func declaration<P: Parser>(
  _ parser: P
) -> AnyParser<Input, Declaration<P.Output>> where P.Input == Input {
  identifier
    .skip(Whitespace())
    .skip(":".utf8)
    .skip(Whitespace())
    .take(parser)
    .map(Declaration.init)
    .eraseToAnyParser()
}

private let variable = variableIdentifier
  .skip(Whitespace())
  .skip(":".utf8)
  .skip(Whitespace())
  .take(value)
  .map(Declaration.init)

private let value = colorSetValue.map(Value.colorSetValue)
  .orElse(colorSet.compactMap(Value.colorSet))

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

private let colorSet = Skip("(".utf8)
  .skip(Whitespace())
  .take(declaration(colorSetValue))
  .skip(listSeparator)
  .take(declaration(colorSetValue))
  .skip(Whitespace())
  .skip(")".utf8)
  .compactMap(ColorSet.init)

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

private let colorSetValue = hexColor.map(ColorSetValue.color)
  .orElse(rgbaColor.map(ColorSetValue.color))
  .orElse(variableValue.map(ColorSetValue.variable))

private let hexColor = Skip("#".utf8)
  .take(hexValue)
  .take(hexValue)
  .take(hexValue)
  .take(Skip(" ".utf8).skip(Whitespace()).take(alphaValue).orElse(Always(1)))
  .map(Color.init)

private let rgbaColor = Skip("rgba".utf8)
  .skip("(".utf8)
  .take(
    UInt8.parser()
      .skip(listSeparator)
      .take(UInt8.parser())
      .skip(listSeparator)
      .take(UInt8.parser())
      .skip(listSeparator)
      .take(Float.parser())
  )
  .skip(Whitespace())
  .skip(")".utf8)
  .map(Color.init)

private let lineDelimiter = Skip("\n".utf8).skip(Whitespace())

private let listSeparator = Skip(Whitespace()).skip(",".utf8).skip(Whitespace())

private let variableValue = variableIdentifier
  .take(
    Skip(" ".utf8)
      .take(alphaValue)
      .orElse(Always(1))
  )
  .map(Variable.init)

private let variableIdentifier = Skip("$".utf8).take(identifier)

private let identifier = Prefix<Input>(1...) { c in
  (c >= UInt8(ascii: "0") && c <= UInt8(ascii: "9")) ||
    (c >= UInt8(ascii: "a") && c <= UInt8(ascii: "z")) ||
    (c >= UInt8(ascii: "A") && c <= UInt8(ascii: "Z"))
}.compactMap(String.init)

private let hexValue = Prefix<Input>(2)
  .compactMap { UInt8(String(decoding: $0, as: UTF8.self), radix: 16) }

private let alphaValue = Int.parser(of: Input.self, isSigned: false)
  .skip("%".utf8)
  .map { Float($0) / 100 }
