// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "Fuso",
    platforms: [.macOS(.v13)],
    targets: [
        .executableTarget(
            name: "Fuso",
            path: "Sources"
        )
    ]
)
