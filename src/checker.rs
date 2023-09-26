use std::{fmt::Debug, rc::Rc};

use serde::Serialize;
use swc_common::{BytePos, FileName, SourceFile, Span};
use swc_css_ast::{Ident, Stylesheet};
use swc_css_parser::parser::ParserConfig;

use crate::short_tailwind::ClassContainer;

struct ParserResult {
    ast: Option<Stylesheet>,
    errors: Vec<Error>,
}

struct Error {
    span: Span,
    message: String,
}

pub fn parse(
    code: String,
    allow_wrong_line_comments: bool,
    css_modules: bool,
    legacy_nesting: bool,
) {
    let code2 = code.clone();

    let source_file = SourceFile::new(
        FileName::Custom("input_css".into()),
        false,
        FileName::Custom("input.css".into()),
        code,
        BytePos(1),
    );

    let mut class_container = ClassContainer::default();

    let mut errors = vec![];

    let mut result = match swc_css_parser::parse_file(
        &source_file,
        ParserConfig {
            allow_wrong_line_comments,
            css_modules,
            legacy_nesting,
            legacy_ie: false,
        },
        &mut errors,
    ) {
        Ok(stylesheet) => ParserResult {
            ast: Some(stylesheet),
            errors: errors.into_iter().map(convert_recoverable_error).collect(),
        },
        Err(error) => {
            let message = error.message().to_string();
            ParserResult {
                ast: None,
                errors: vec![Error {
                    span: error.into_inner().0,
                    message,
                }],
            }
        }
    };

    if let Some(s) = result.ast {
        for rule in s.rules.into_iter() {
            if let Some(i) = rule.qualified_rule() {
                if let Some(l) = i.prelude.selector_list() {
                    for complex_children in l.children {
                        for children in complex_children.children {
                            if let Some(c) = children.compound_selector() {
                                for mut s in c.subclass_selectors {
                                    if s.is_class() {
                                        if let Some(r) = s.class() {
                                            if let Some(r) = r.text.raw {
                                                class_container.add(r.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // while let Some(i) = rule.qualified_rule() {
            //     println!("asfjaksfl");
        }
    }

    class_container.to_file(code2);
}

fn convert_recoverable_error(error: swc_css_parser::error::Error) -> Error {
    let message = format!("{} (Recoverable)", error.message());
    Error {
        span: error.into_inner().0,
        message,
    }
}

fn abc() {
    // let a = swc_css_parser::
    // swc_css_parser::parse_file(, , )
}
