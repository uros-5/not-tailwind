use crate::visit_selectors::ClassVisitor;

use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::{
        ast::Program,
        parser::{lexer::Lexer, Parser, StringInput},
        transforms,
        visit::{VisitAll, VisitAllWith, VisitMut},
    },
};

pub fn check_js<'a>(class_visitor: &'a ClassVisitor) {
    let mut map_visitor = MapVisitor::new(class_visitor);
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        FileName::Custom("test.js".into()),
        "let a = new Map(); a.set('text-2xl', 'text-2xl')".into(),
    );
    let lexer = Lexer::new(
        swc_core::ecma::parser::Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let s = parser.parse_program();
    match s {
        Ok(mut p) => {
            p.visit_all_children_with(&mut map_visitor);
            // dbg!(p);
        }
        Err(_) => todo!(),
    }

    // let parsed = Program::
    // let parsed = parse_options()
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

impl<'a> VisitAll for MapVisitor<'a> {
    fn visit_ident(&mut self, n: &swc_core::ecma::ast::Ident) {
        dbg!(n);
    }
}

impl<'a> VisitAllWith<MapVisitor<'a>> for MapVisitor<'a> {
    #[doc = r" Calls a visitor method (v.visit_xxx) with self."]
    fn visit_all_with(&self, v: &mut MapVisitor<'a>) {
        println!("what");
        todo!()
    }

    #[doc = r" Visit children nodes of self with `v`"]
    fn visit_all_children_with(&self, v: &mut MapVisitor<'a>) {
        todo!()
    }
    //
}
