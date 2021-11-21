import Foundation

public enum ResolvedVariable {
  case color(Color)
  case colorSet(ResolvedColorSet)
}

public struct ResolvedColorSet {
  public var light: Color
  public var dark: Color
}

public struct ResolveVariableError: Error {
  public var localizedDescription: String
}

public struct VariableContext {
  var lookup: [String: Value]

  public init(document: Document) {
    let keysAndValues = document.items
      .compactMap { item -> (String, Value)? in
        guard case let .variable(declaration) = item else {
          return nil
        }
        return (declaration.identifier, declaration.value)
      }
    self.lookup = Dictionary(keysAndValues, uniquingKeysWith: { _, last in last })
  }
}

public extension VariableContext {
  func resolve(variable: Variable) throws -> ResolvedVariable {
    guard let value = self.lookup[variable.identifier] else {
      throw ResolveVariableError(
        localizedDescription: "Could not find variable with identifier \(variable.identifier)"
      )
    }
    switch value {
    case let .variable(variable):
      return try self.resolve(variable: variable)
    case let .colorSet(colorSet):
      return try .colorSet(self.resolve(colorSet: colorSet))
    case let .color(color):
      return .color(color)
    }
  }

  func resolve(colorSet: ColorSet) throws -> ResolvedColorSet {
    try ResolvedColorSet(
      light: self.resolve(colorSetValue: colorSet.light),
      dark: self.resolve(colorSetValue: colorSet.dark)
    )
  }

  private func resolve(colorSetValue: ColorSetValue) throws -> Color {
    switch colorSetValue {
    case let .variable(variable):
      let resolvedVariable = try self.resolve(variable: variable)
      switch resolvedVariable {
      case let .color(color):
        return color
      case .colorSet:
        throw ResolveVariableError(
          localizedDescription: """
          "Attempt to assign a colorset to a property of another colorset via \
          variable \(variable.identifier).
          """
        )
      }
    case let .color(color):
      return color
    }
  }
}
