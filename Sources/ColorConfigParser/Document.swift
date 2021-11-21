import Foundation

public struct Document {
  public var items: [DocumentItem]
}

public enum DocumentItem {
  case variable(Declaration<Value>)
  case ruleSet(RuleSet)
  case declaration(Declaration<Value>)
}

public struct RuleSet {
  public var identifier: String
  public var items: [RuleSetItem]
}

public enum RuleSetItem {
  case ruleSet(RuleSet)
  case declaration(Declaration<Value>)
}

public struct Declaration<T> {
  public var identifier: String
  public var value: T
}

public enum Value {
  case variable(Variable)
  case color(Color)
  case colorSet(ColorSet)
}

public struct ColorSet {
  public var light: ColorSetValue
  public var dark: ColorSetValue
}

public enum ColorSetValue {
  case variable(Variable)
  case color(Color)
}

public struct Variable {
  public var identifier: String
  public var opacity: Float
}

public struct Color {
  public var red: UInt8
  public var green: UInt8
  public var blue: UInt8
  public var alpha: Float
}
