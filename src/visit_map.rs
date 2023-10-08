use crate::visit_selectors::ClassVisitor;

use swc_core::ecma::{
    ast::Program,
    atoms::JsWordStaticSet,
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{
    plugin_transform, proxies::TransformPluginProgramMetadata,
};

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
