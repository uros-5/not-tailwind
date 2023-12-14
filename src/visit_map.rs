use crate::visit_selectors::ClassVisitor;

use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::codegen::{text_writer::JsWriter, Emitter},
    ecma::{
        parser::{lexer::Lexer, Parser, StringInput},
        visit::{as_folder, FoldWith, VisitMut},
    },
};

pub fn check_js(
    old_content: &String,
    class_visitor: &ClassVisitor,
) -> Option<Vec<u8>> {
    let map_visitor = MapVisitor::new(class_visitor);
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("test.js".into()),
        old_content.into(),
    );
    let lexer = Lexer::new(
        swc_core::ecma::parser::Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let s = parser.parse_program();
    let mut code = vec![];
    let mut srcmap = vec![];

    match s {
        Ok(program) => {
            let program = program.fold_with(&mut as_folder(map_visitor));

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
                let module = program.as_script()?;
                if emitter.emit_script(module).is_ok() {
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

impl<'a> VisitMut for MapVisitor<'a> {
    fn visit_mut_ident(&mut self, n: &mut swc_core::ecma::ast::Ident) {
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
                    let new_str = &self.class_visitor.get(s);
                    if let Some(new_str) = new_str {
                        let new_str = format!("'{}'", new_str);
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
