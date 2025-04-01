use std::{
    fs::{read_to_string, File},
    io::Write,
};

use ignore::Walk;
use swc_core::{
    common::{sync::Lrc, FileName, SourceMap},
    ecma::{
        ast::CallExpr,
        codegen::{text_writer::JsWriter, Emitter},
        parser::{lexer::Lexer, Parser, StringInput},
        visit::{Visit, VisitWith},
    },
};
use tree_sitter::{Parser as TParser, Query, QueryCursor};

pub fn build_ts_map() {
    let mut vue = VueTs::default();
    let mut ts_content = vec![];
    let mut ntw_path = String::new();
    for result in Walk::new("./") {
        match result {
            Ok(entry) => {
                let path = entry.path().display().to_string();

                if path.ends_with("not-tailwind.ts") {
                    ntw_path = path.clone();
                }

                if path.ends_with(".vue") || path.ends_with(".ts") {
                    let content = read_to_string(&path).unwrap();
                    ts_content.extend(get_ts_content(&path, content, &mut vue));
                }
            }
            Err(err) => println!("ERROR: {}", err),
        }
    }
    if ntw_path == "" {
        println!("not-tailwind.ts not found");
        ntw_path = "not-tailwind.ts".to_string();
    }

    let start = "export const ntw = new Map();// as Map<NtwType, string>;\n"
        .to_string();

    let mut setters = "".to_string();
    let mut types = "type NtwType = ".to_string();
    let mut added_type = false;
    ts_content.dedup();
    for i in &ts_content {
        let content = format!("ntw.set('{}', '{}');\n", i, i);
        setters.push_str(&content);
        // types.push_str(&format!("| '{}' ", i));
        types.clear();
        added_type = true;
    }
    if !added_type {
        types.push_str(" '' ");
    }
    types.push_str(";");
    let ntw = format!("{types}\n{start}\n{setters}");
    let mut file = File::create(ntw_path).unwrap();
    file.write_all(ntw.as_bytes()).ok();
}

pub struct VueTs {
    pub parser: TParser,
    pub query: Query,
}

impl Clone for VueTs {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl Default for VueTs {
    fn default() -> VueTs {
        let mut parser = TParser::new();
        parser.set_language(tree_sitter_vue::language()).unwrap();

        let query = r#"

(
(script_element
	(raw_text) @text)
@script)
            
        "#;

        let query = Query::new(tree_sitter_vue::language(), query).unwrap();
        Self { parser, query }
    }
}

pub fn get_ts_content(
    file_name: &str,
    mut content: String,
    vue: &mut VueTs,
) -> Vec<String> {
    if file_name.ends_with(".vue") {
        let tree = vue.parser.parse(&content, None).unwrap();

        let mut cursor = QueryCursor::new();
        let capture_names = vue.query.capture_names();
        let node = tree.root_node();
        let c2 = content.clone();
        let captures = cursor.captures(&vue.query, node, c2.as_bytes());
        let mut new_content = String::new();
        let mut id = 0;
        for m in captures {
            for capture in m.0.captures {
                let name = &capture_names[capture.index as usize];
                if name == "text" {
                    if id == capture.node.id() {
                        continue;
                    }
                    id = capture.node.id();
                    if let Ok(content) =
                        capture.node.utf8_text(&content.as_bytes())
                    {
                        new_content.push_str(&content);
                    }
                }
            }
        }
        content = new_content;
    }
    check_script(&content)
}

#[derive(Default)]
pub struct ScriptContent {
    ntw_import: bool,
    is_import: bool,
    identifiers: Vec<String>,
}

impl ScriptContent {
    pub fn check_expr(&mut self, node: &CallExpr) -> Option<String> {
        let expr = node.clone().callee.expr()?;
        let member = expr.member()?;
        let object = member.obj.ident()?;
        if object.sym != "ntw" {
            return None;
        }
        let prop = member.prop.ident()?;
        if prop.sym != "get" {
            return None;
        }
        let first_arg = node.args.first()?;
        let lit = first_arg.clone().expr.lit()?;
        let value = lit.str()?.value;
        Some(value.to_string())
    }
}

impl Visit for ScriptContent {
    fn visit_import_decl(&mut self, node: &swc_core::ecma::ast::ImportDecl) {
        if self.ntw_import {
            return;
        }
        for specifier in &node.specifiers {
            if !specifier.is_named() {
                return;
            }
            if let Some(named) = &specifier.clone().named() {
                if named.local.sym == "ntw" {
                    self.is_import = true;
                    break;
                }
            }
        }
        if self.is_import {
            if node.src.value.ends_with("not-tailwind") {
                self.ntw_import = true;
            } else {
                self.ntw_import = false;
                self.ntw_import = false;
            }
        }
    }

    fn visit_call_expr(&mut self, node: &CallExpr) {
        if self.ntw_import == false {
            return;
        }
        let id = self.check_expr(node);
        if let Some(id) = id {
            if self.identifiers.contains(&id) {
                return;
            }
            self.identifiers.push(id);
        } else {
            <CallExpr as VisitWith<Self>>::visit_children_with(node, self);
        }
    }
}

pub fn check_script(old_content: &str) -> Vec<String> {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("test.js".into())),
        old_content.into(),
    );
    let lexer = Lexer::new(
        swc_core::ecma::parser::Syntax::Typescript(Default::default()),
        swc_core::ecma::ast::EsVersion::Es2023,
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);
    let s = parser.parse_module();
    let mut code = vec![];
    let mut srcmap = vec![];
    let mut map_visitor = ScriptContent::default();

    match s {
        Ok(module) => {
            module.visit_with(&mut map_visitor);

            {
                let mut _emmiter = Emitter {
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
                return map_visitor.identifiers;
                // if emitter.emit_module(&module).is_ok() {
                // }
            }
        }
        Err(e) => {
            println!("error 1, {:?}", e);
            return map_visitor.identifiers;
        }
    }
}
