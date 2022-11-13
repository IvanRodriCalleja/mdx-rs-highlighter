use mdx_lexers::constants::{DATA_LINE_NUMBERS_ATTR, CODE_CLASS_NAME};
use mdxjs::hast;
use mdx_lexers::highlight;

use crate::{config, language};

pub fn visit_code_mut(tree: &mut hast::Node, options: &config::HighlighterConfig) {
    for child in tree.children_mut().unwrap_or(&mut vec![]) {
        match child {
            hast::Node::Element(element) => match element.tag_name.as_str() {
                "code" => {
                    highlight_code(element, options);
                },
                "pre" => {
                    highlight_pre(element, options);
                },
                _ => {}
            },
            //Plugin just check for pre or code tag at root level
            element => visit_code_mut(element, options)
        }
    }
}

fn highlight_code(element: &mut hast::Element , options: &config::HighlighterConfig) {
    if let Some(node) = element.children.first() {
        if let hast::Node::Text(text) = node {
            let language = language::get_language(element.properties.clone());
            let input: Vec<char> = text.value.chars().collect();

            let code_rows = highlight(input, &language);

            let hast = hast::Node::Element(hast::Element {
                tag_name: "div".into(),
                properties: vec![(
                    "className".into(),
                    hast::PropertyValue::String(CODE_CLASS_NAME.into()),
                ), (
                    DATA_LINE_NUMBERS_ATTR.into(),
                    hast::PropertyValue::String(options.line_number.to_string()),
                )],
                children: code_rows,
                position: None,
            });

            element.children = vec![hast];
        }
    }
}

fn highlight_pre(element: &mut hast::Element, options: &config::HighlighterConfig) {
    if let Some(node) = element.children.first_mut() {
        if let hast::Node::Element(child) = node {
            if child.tag_name == "code" {
                highlight_code(child, options);
            }
        }
    }
}
