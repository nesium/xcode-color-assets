import ArgumentParser
import AssetCatalogGenerator
import ColorConfigParser
import Foundation

struct XcodeColorAssets: ParsableCommand {
  public static let configuration =
    CommandConfiguration(
      abstract: "Create (dark mode compatible) color assets for Xcode programmatically from a CSS-like textfile"
    )

  @Argument(help: "Sets the input file")
  private var input: String

  @Argument(help: "Sets the output filename (e.g. Colors.xcassets)")
  private var output: String

  func validate() throws {
    guard FileManager.default.fileExists(atPath: self.input) else {
      throw ValidationError("No such file \(self.input)")
    }
  }

  func run() throws {
    let document = try parseDocument(input: String(contentsOfFile: self.input))!
    try generateAssetCatalog(
      from: document,
      at: URL(fileURLWithPath: self.output),
      using: .sRGB,
      context: VariableContext(document: document)
    )
  }
}

XcodeColorAssets.main()
