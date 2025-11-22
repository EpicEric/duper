// swift-tools-version:5.3

import Foundation
import PackageDescription

var sources = ["src/parser.c"]
if FileManager.default.fileExists(atPath: "src/scanner.c") {
    sources.append("src/scanner.c")
}

let package = Package(
    name: "TreeSitterDuper",
    products: [
        .library(name: "TreeSitterDuper", targets: ["TreeSitterDuper"]),
    ],
    dependencies: [
        .package(name: "SwiftTreeSitter", url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.9.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterDuper",
            dependencies: [],
            path: ".",
            sources: sources,
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift",
            cSettings: [.headerSearchPath("src")]
        ),
        .testTarget(
            name: "TreeSitterDuperTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterDuper",
            ],
            path: "bindings/swift/TreeSitterDuperTests"
        )
    ],
    cLanguageStandard: .c11
)
