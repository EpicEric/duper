import XCTest
import SwiftTreeSitter
import TreeSitterDuper

final class TreeSitterDuperTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_duper())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Duper grammar")
    }
}
