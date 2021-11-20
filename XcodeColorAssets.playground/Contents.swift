import Cocoa
import XcodeColorAssets

let input = """
$white: #ffffff
$black: #000000
$black50: #000000 50%
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
    Shadow: (light: #848587, dark: $black50 50%)
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
"""

dump(parseDocument(input: input))
