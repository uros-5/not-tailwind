use std::{collections::BTreeMap, path::Path};

use minijinja::machinery::{self, Token, WhitespaceConfig};

use crate::{starter::MacroClassWalker, visit_selectors::ClassVisitor};

pub fn check_macro(
    old_content: String,
    class_visitor: &ClassVisitor,
    macros_visitor: &mut MacroClassWalker,
    path: &Path,
) -> Option<String> {
    let tokenizer =
        machinery::tokenize(&old_content, false, Default::default(), WhitespaceConfig::default());
    let mut visitor = MacroVisitor::default();
    for result in tokenizer.flatten() {
        match result.0 {
            Token::Ident(ident) => {
                let ident = ident.to_lowercase();
                if ident.ends_with("class") {
                    visitor.counter = 1;
                } else {
                    visitor.counter = 0;
                }
            }
            Token::Assign => {
                if visitor.counter == 1 {
                    visitor.counter = 2;
                } else {
                    visitor.counter = 0;
                }
            }
            Token::Str(class) => {
                if visitor.counter == 2 {
                    let mut new_classes = vec![];
                    let parts = class.split(' ');
                    for part in parts {
                        if let Some(c) = class_visitor.get(part) {
                            new_classes.push(c);
                        } else {
                            new_classes.push(part.to_string());
                        }
                    }
                    let new_class = new_classes.join(" ");
                    visitor.container.insert(class.to_string(), new_class);
                    visitor.counter = 0;
                }
            }
            _ => visitor.counter = 0,
        }
    }
    macros_visitor
        .insert(path.to_str().unwrap().to_string(), visitor.container);
    // for i in visitor.container {
    //     old_content = old_content.replace(&i.0, &i.1);
    // }
    Some(old_content)
}

#[derive(Default)]
pub struct MacroVisitor {
    pub counter: u8,
    pub container: BTreeMap<String, String>,
}
