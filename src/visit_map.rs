use crate::visit_selectors::ClassVisitor;

use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::{
        codegen::{text_writer::JsWriter, Emitter},
        parser::{lexer::Lexer, Parser, StringInput},
        visit::{Fold, VisitMut, VisitMutWith},
    },
};

pub fn check_js(
    old_content: &String,
    class_visitor: &ClassVisitor,
) -> Option<Vec<u8>> {
    let mut map_visitor = MapVisitor::new(class_visitor);
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("test.js".into())),
        old_content.into(),
    );
    let lexer = Lexer::new(
        swc_core::ecma::parser::Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let s = parser.parse_module();
    let mut code = vec![];
    let mut srcmap = vec![];

    match s {
        Ok(mut module) => {
            module.visit_mut_with(&mut map_visitor);

            {
                let mut emitter = Emitter {
                    cfg: Default::default(),
                    cm: cm.clone(),
                    comments: None,
                    wr: JsWriter::new(
                        cm.clone(),
                        "\n",
                        &mut code,
                        Some(&mut srcmap),
                    ),
                };
                if emitter.emit_module(&module).is_ok() {
                    return Some(code);
                }
                None
            }
        }
        Err(_) => None,
    }
}

pub struct MapVisitor<'a> {
    pub is_set: bool,
    pub first_string: Option<String>,
    pub class_visitor: &'a ClassVisitor,
}

impl<'a> MapVisitor<'a> {
    fn new(class_visitor: &'a ClassVisitor) -> Self {
        Self {
            is_set: false,
            first_string: None,
            class_visitor,
        }
    }
}

impl<'a> Fold for MapVisitor<'a> {}

impl<'a> VisitMut for MapVisitor<'a> {
    fn visit_mut_ident_name(&mut self, n: &mut swc_core::ecma::ast::IdentName) {
        let s = n.to_string();
        if s.starts_with("set") {
            self.is_set = true;
        } else {
            self.is_set = false;
            self.first_string = None;
        }
    }

    fn visit_mut_str(&mut self, n: &mut swc_core::ecma::ast::Str) {
        if self.is_set {
            match &self.first_string {
                Some(s) => {
                    let mut custom_variable = "";
                    let new_str = &self.class_visitor.get(s).or_else(|| {
                        custom_variable = "--";
                        self.class_visitor.get_custom_property(s)
                    });
                    if let Some(new_str) = new_str {
                        let new_str =
                            format!("'{}{}'", custom_variable, new_str);
                        n.raw = Some(new_str.into());
                    }
                    self.is_set = false;
                    self.first_string = None;
                }
                None => {
                    self.is_set = true;
                    self.first_string = Some(n.value.to_string());
                }
            }
        }
    }
}
