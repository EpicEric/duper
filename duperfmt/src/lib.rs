#![doc(html_logo_url = "https://duper.dev.br/logos/duper-100-100.png")]
//!
#![doc = include_str!("./duperfmt.md")]
//!

use std::io::Write;

use topiary_core::{Language, Operation, TopiaryQuery, formatter_tree};
use tree_sitter::Tree;

const DUPER_QUERY: &str = include_str!("./duper.scm");

/// Given a Duper [`Tree`] built from an input, formats said input into the output buffer.
pub fn format_duper(
    tree: Tree,
    input: &str,
    mut output: impl Write,
    indent: Option<String>,
    debug: bool,
) -> Result<(), topiary_core::FormatterError> {
    let language = Language {
        name: "duper".to_owned(),
        query: TopiaryQuery::new(&tree_sitter_duper::LANGUAGE.into(), DUPER_QUERY)?,
        grammar: tree_sitter_duper::LANGUAGE.into(),
        indent,
    };

    formatter_tree(
        tree.into(),
        input,
        &mut output,
        &language,
        Operation::Format {
            skip_idempotence: !debug,
            tolerate_parsing_errors: false,
        },
    )
}

#[cfg(test)]
mod format_duper_tests {
    use insta::assert_snapshot;

    use super::{Tree, format_duper};

    fn parse_duper(input: &'static str) -> Tree {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_duper::LANGUAGE.into())
            .unwrap();
        let tree = parser.parse(input, None).expect("parser was initialized");
        tree
    }

    #[test]
    fn single_line_object_no_indent() {
        let input = r#"
            {level:"WARN",timestamp:Instant('2025-12-04T13:29:33.947380870-03:00'),target:"simple",span:{count:10,span_id:1},spans:[{count:10,span_id:1}],fields:{message:"too few gifts... try again later"}}
        "#;
        let tree = parse_duper(input);
        let mut output = Vec::new();
        format_duper(tree, input, &mut output, None, true).unwrap();
        assert_snapshot!(String::from_utf8(output).unwrap());
    }

    #[test]
    fn multiline_object_no_indent() {
        let input = r##"
            Product({
                product_id: Uuid("1dd7b7aa-515e-405f-85a9-8ac812242609"),
                name: "Wireless Bluetooth Headphones",
                brand: "AudioTech", price: Decimal("129.99"), dimensions: (18.5, 15.2, 7.8), // In centimeters
                weight: Kilograms(0.285),

                in_stock: true,
                specifications: {     battery_life  : Duration("30h"),
                    noise_cancellation: true,
                    connectivity: ["Bluetooth 5.0","3.5mm Jack"],
                },
                image_thumbnail: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ="),
                tags:   
                   [ "electronics" , "audio"  , "wireless" ],
                release_date: PlainDate('2023-11-15'),
                /* Warranty is optional */
                "warranty period":null,
                customer_ratings: {
                    latest_review: r#"Absolutely ""astounding""!! 😎"#,
                    average: 4.5,
                    count: 127,
                },
                created_at: Instant('2023-11-17T21:50:43+00:00')
                })
        "##;
        let tree = parse_duper(input);
        let mut output = Vec::new();
        format_duper(tree, input, &mut output, None, true).unwrap();
        assert_snapshot!(String::from_utf8(output).unwrap());
    }

    #[test]
    fn single_line_object_four_space_indent() {
        let input = r#"
            {level:"WARN",timestamp:Instant('2025-12-04T13:29:33.947380870-03:00'),target:"simple",span:{count:10,span_id:1},spans:[{count:10,span_id:1}],fields:{message:"too few gifts... try again later"}}
        "#;
        let tree = parse_duper(input);
        let mut output = Vec::new();
        format_duper(tree, input, &mut output, Some("    ".into()), true).unwrap();
        assert_snapshot!(String::from_utf8(output).unwrap());
    }

    #[test]
    fn multiline_object_four_space_indent() {
        let input = r##"
            Product({
                product_id: Uuid("1dd7b7aa-515e-405f-85a9-8ac812242609"),
                name: "Wireless Bluetooth Headphones",
                brand: "AudioTech", price: Decimal("129.99"), dimensions: (18.5, 15.2, 7.8), // In centimeters
                weight: Kilograms(0.285),

                in_stock: true,
                specifications: {     battery_life  : Duration("30h"),
                    noise_cancellation: true,
                    connectivity: ["Bluetooth 5.0","3.5mm Jack"],
                },
                image_thumbnail: Png(b64"iVBORw0KGgoAAAANSUhEUgAAAGQ="),
                tags:   
                   [ "electronics" , "audio"  , "wireless" ],
                release_date: PlainDate('2023-11-15'),
                /* Warranty is optional */
                "warranty period":null,
                customer_ratings: {
                    latest_review: r#"Absolutely ""astounding""!! 😎"#,
                    average: 4.5,
                    count: 127,
                },
                created_at: Instant('2023-11-17T21:50:43+00:00')
                })
        "##;
        let tree = parse_duper(input);
        let mut output = Vec::new();
        format_duper(tree, input, &mut output, Some("    ".into()), true).unwrap();
        assert_snapshot!(String::from_utf8(output).unwrap());
    }

    #[test]
    fn invalid_object() {
        let input = r#"
            {"_":
        "#;
        let tree = parse_duper(input);
        let mut output = Vec::new();
        assert!(format_duper(tree, input, &mut output, None, true).is_err());
    }
}
