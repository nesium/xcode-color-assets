// swift-tools-version:5.5

import PackageDescription

let package = Package(
  name: "xcode-color-assets",
  products: [
    .library(
      name: "XcodeColorAssets",
      targets: ["XcodeColorAssets"]
    ),
  ],
  dependencies: [
    .package(
      name: "swift-parsing",
      url: "https://github.com/pointfreeco/swift-parsing",
      .upToNextMajor(from: "0.3.1")
    ),
  ],
  targets: [
    .target(
      name: "XcodeColorAssets",
      dependencies: [
        .product(name: "Parsing", package: "swift-parsing"),
      ]
    ),
  ]
)
