use mdxjs::hast;

use crate::constants::{LINE_CLASS_NAME, DATA_LINE_NUMBER_ATTR};

pub struct HighlighterTokenizer {
    rows: Vec<hast::Node>,
    tokens: Vec<hast::Node>,
}

impl HighlighterTokenizer {
    pub fn new() -> Self {
        Self {
            rows: vec![],
            tokens: vec![],
        }
    }

    pub fn get_highlighted_code(self) -> Vec<hast::Node> {
        self.rows
    }

    pub fn end_of_line(&mut self) {
        let row = hast::Node::Element(hast::Element {
            tag_name: "div".into(),
            properties: vec![
                (
                    "className".into(),
                    hast::PropertyValue::String(LINE_CLASS_NAME.into()),
                ),
                (
                    DATA_LINE_NUMBER_ATTR.into(),
                    hast::PropertyValue::String((self.rows.len() + 1).to_string()),
                ),
            ],
            children: self.tokens.clone(),
            position: None,
        });
        self.rows.push(row);

        self.tokens = vec![];
    }

    pub fn add_token(&mut self, text: String, class_name: String) {
        self.tokens.push(hast::Node::Element(hast::Element {
            tag_name: "span".into(),
            properties: vec![("className".into(), hast::PropertyValue::String(class_name))],
            children: vec![
                (hast::Node::Text(hast::Text {
                    value: text,
                    position: None,
                })),
            ],
            position: None,
        }));
    }

    pub fn add_text(&mut self, value: String) {
        if let Some(node) = self.tokens.last_mut() {
            if let hast::Node::Text(text) = node {
                text.value.push_str(value.as_str());
            } else {
                self.create_text_token(value);
            }
        } else {
            self.create_text_token(value);
        }
    }

    fn create_text_token(&mut self, value: String) {
        self.tokens.push(hast::Node::Text(hast::Text {
            value,
            position: None,
        }));
    }
}
