pub mod config;
mod visitor;
mod language;

use mdxjs::hast;
use visitor::visit_code_mut;

pub fn mdx_plugin_highlighter(tree: &mut hast::Node, options: &config::HighlighterConfig) {
    visit_code_mut(tree, options);
}
