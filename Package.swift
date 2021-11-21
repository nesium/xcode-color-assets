// swift-tools-version:5.5

import PackageDescription

let package = Package(
  name: "xcode-color-assets",
  products: [
    .executable(name: "XcodeColorAssets", targets: ["XcodeColorAssets"]),
    .library(
      name: "ColorConfigParser",
      targets: ["ColorConfigParser"]
    ),
  ],
  dependencies: [
    .package(
      name: "swift-parsing",
      url: "https://github.com/pointfreeco/swift-parsing",
      .upToNextMajor(from: "0.3.1")
    ),
    .package(
      name: "swift-argument-parser",
      url: "https://github.com/apple/swift-argument-parser",
      .upToNextMajor(from: "1.0.2")
    ),
    .package(
      name: "Stencil",
      url: "https://github.com/stencilproject/Stencil",
      .upToNextMajor(from: "0.14.2")
    ),
  ],
  targets: [
    .executableTarget(
      name: "XcodeColorAssets",
      dependencies: [
        "ColorConfigParser",
        "AssetCatalogGenerator",
        .product(name: "ArgumentParser", package: "swift-argument-parser"),
      ]
    ),
    .target(
      name: "ColorConfigParser",
      dependencies: [
        .product(name: "Parsing", package: "swift-parsing"),
      ]
    ),
    .target(
      name: "AssetCatalogGenerator",
      dependencies: [
        "ColorConfigParser",
        "Stencil",
      ]
    ),
  ]
)
