# Color Assets

Tired of creating Color Sets in Xcode? Too much clicking, not enough typing? Missing variables?

Turn this…

```
$white: #ffffff
$black: #000000
$classic: (light: $black, dark: $white)

$brightAccent: #5753CF
$mediumBright: rgba(25, 200, 255, 1)
$mediumBrightHighlight: #70D1FA

$grey1: $black

Text {
  Primary: (light: #151618, dark: #E7E8EA)
  Secondary: (light: $grey1, dark: #85868A)
}

LightContentSeparator: (light: #F1F2F2, dark: #222525)

NumericInput {
  NumericKey {
    Background: (light: $white, dark: #434343)
    Highlight: (light: #C4CCDA, dark: #666666)
    Shadow: (light: #848587, dark: $black)
    Text: $classic
  }

  DoneKey {
    Background: (light: $mediumBright, dark: $brightAccent)
    Highlight: (light: $mediumBrightHighlight, dark: rgba(103, 122, 219, 1))
    Shadow: (light: #6E7073, dark: $black)
    Text: $classic
  }

  Background: (light: #D6D9DE 30%, dark: #313131 40%)
}
```

into this…

![Xcode Screenshot](./.github/Xcode.png)

and also this…

```swift
// This file is automatically generated. Do not edit, your changes will be erased.

import UIKit

extension UIColor {
  enum Custom {
    static let LightContentSeparator = UIColor(named: "LightContentSeparator")!
    enum NumericInput {
      enum NumericKey {
        static let Background = UIColor(named: "NumericInputNumericKeyBackground")!
        static let Highlight = UIColor(named: "NumericInputNumericKeyHighlight")!
        static let Shadow = UIColor(named: "NumericInputNumericKeyShadow")!
        static let Text = UIColor(named: "NumericInputNumericKeyText")!
      }
      enum DoneKey {
        static let Background = UIColor(named: "NumericInputDoneKeyBackground")!
        static let Highlight = UIColor(named: "NumericInputDoneKeyHighlight")!
        static let Shadow = UIColor(named: "NumericInputDoneKeyShadow")!
        static let Text = UIColor(named: "NumericInputDoneKeyText")!
      }
      static let Background = UIColor(named: "NumericInputBackground")!
    }
    enum Text {
      static let Primary = UIColor(named: "TextPrimary")!
      static let Secondary = UIColor(named: "TextSecondary")!
    }
  }
}

```

Usage:

```
$ ./xcode-color-assets gen-assets colors.assetstyles -o Colors.xcassets
$ ./xcode-color-assets gen-swift colors.assetstyles -o UIColor+Custom.swift
```
