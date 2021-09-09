// This file is automatically generated. Do not edit, your changes will be erased.

import UIKit

extension UIColor {
  enum Custom {
    static let LightContentSeparator = UIColor(named: "LightContentSeparator", in: BundleToken.bundle, compatibleWith: nil)!
    enum NumericInput {
      static let Background = UIColor(named: "NumericInputBackground", in: BundleToken.bundle, compatibleWith: nil)!
      enum DoneKey {
        static let Background = UIColor(named: "NumericInputDoneKeyBackground", in: BundleToken.bundle, compatibleWith: nil)!
        static let Highlight = UIColor(named: "NumericInputDoneKeyHighlight", in: BundleToken.bundle, compatibleWith: nil)!
        static let Shadow = UIColor(named: "NumericInputDoneKeyShadow", in: BundleToken.bundle, compatibleWith: nil)!
        static let Text = UIColor(named: "NumericInputDoneKeyText", in: BundleToken.bundle, compatibleWith: nil)!
      }
      enum NumericKey {
        static let Background = UIColor(named: "NumericInputNumericKeyBackground", in: BundleToken.bundle, compatibleWith: nil)!
        static let Highlight = UIColor(named: "NumericInputNumericKeyHighlight", in: BundleToken.bundle, compatibleWith: nil)!
        static let Shadow = UIColor(named: "NumericInputNumericKeyShadow", in: BundleToken.bundle, compatibleWith: nil)!
        static let Text = UIColor(named: "NumericInputNumericKeyText", in: BundleToken.bundle, compatibleWith: nil)!
      }
    }
    enum Text {
      static let Primary = UIColor(named: "TextPrimary", in: BundleToken.bundle, compatibleWith: nil)!
      static let Secondary = UIColor(named: "TextSecondary", in: BundleToken.bundle, compatibleWith: nil)!
    }
  }
}

private final class BundleToken {
  static let bundle: Bundle = {
    #if SWIFT_PACKAGE
    return Bundle.module
    #else
    return Bundle(for: BundleToken.self)
    #endif
  }()
}
