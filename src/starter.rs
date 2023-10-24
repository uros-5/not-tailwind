use std::{
    collections::{BTreeMap, HashMap},
    fs::{self, read_to_string},
    path::Path,
};

use crate::{
    config::{read_config, Config},
    visit_class::check_html,
    visit_macros::check_macro,
    visit_map::check_js,
    visit_selectors::ClassVisitor,
};
use lightningcss::{
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    visitor::Visit,
};

pub fn start_all() {
    let config = read_config();
    match config {
        Ok(config) => {
            let validate = config.validate();
            if config.validate().is_ok() {
                let mut css_walker = CSSWalker::new(&config.ignored_files);

                for dir in &config.css_dir {
                    css_walker.walk_tree(dir, &config);
                }

                let mut macro_class_walker =
                    MacroClassWalker::new(&css_walker.class_visitor);

                if let Some(macros) = &config.macro_classes {
                    for dir in macros {
                        macro_class_walker.walk_tree(dir, &config);
                    }
                }

                let mut html_walker = HTMLWalker::new(
                    &css_walker.class_visitor,
                    &macro_class_walker,
                );

                for dir in &config.html_dir {
                    html_walker.walk_tree(dir, &config);
                }

                let mut js_walker = JSWalker::new(&css_walker.class_visitor);
                if let Some(js) = &config.js_map {
                    for dir in js {
                        js_walker.walk_tree(dir, &config);
                    }
                }

                if let Some(assets_dir) = &config.assets_dir {
                    for dir in assets_dir {
                        html_walker.walk_tree(dir, &config);
                    }
                }
            } else {
                println!("{}", validate.err().unwrap());
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn handle_path(path: &Path) -> &Path {
    if path.starts_with("./") {
        if let Some(path2) = path.to_str() {
            let mut path2 = path2.chars();
            path2.next();
            path2.next();
            return Path::new(path2.as_str());
        }
    }
    path
}

trait TreeWalker {
    fn walk(&mut self, old_content: String, path: &Path) -> Option<String>;

    fn write(&self, path: &Path, new_content: Option<&str>, config: &Config) {
        let output_path = Path::new(&config.output_dir);
        let new_path = output_path.join(handle_path(path));
        if let Some(parent) = new_path.parent() {
            let _ = std::fs::create_dir_all(parent);
            if let Some(new_content) = new_content {
                let _ = std::fs::write(new_path, new_content);
            } else {
                let _ = fs::copy(path, new_path);
            }
        }
    }

    fn walk_tree(&mut self, dir: &String, config: &Config) {
        if let Ok(paths) = std::fs::read_dir(dir) {
            for path in paths.flatten() {
                if let Ok(meta) = path.metadata() {
                    if meta.is_file() {
                        if let Ok(old_content) = read_to_string(path.path()) {
                            if let Some(new_content) =
                                self.walk(old_content, &path.path())
                            {
                                self.write(
                                    &path.path(),
                                    Some(&new_content),
                                    config,
                                );
                            }
                        } else {
                            self.write(&path.path(), None, config)
                        }
                    } else if meta.is_dir() {
                        if let Some(path) = path.path().to_str() {
                            self.walk_tree(&path.to_string(), config);
                        }
                    }
                }
            }
        }
    }
}

struct CSSWalker<'a> {
    pub class_visitor: ClassVisitor,
    pub ignored: &'a Option<Vec<String>>,
}

impl<'a> CSSWalker<'a> {
    pub fn new(ignored: &'a Option<Vec<String>>) -> Self {
        Self {
            class_visitor: ClassVisitor::default(),
            ignored,
        }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        if let Some(ignored) = self.ignored {
            if let Some(path) = path.to_str() {
                return ignored.contains(&path.to_string());
            }
        }
        false
    }
}

impl<'a> TreeWalker for CSSWalker<'a> {
    fn walk(&mut self, old_content: String, path: &Path) -> Option<String> {
        if self.is_ignored(path) {
            return None;
        }
        let a = StyleSheet::parse(&old_content, ParserOptions::default());
        if let Ok(mut stylesheet) = a {
            let _ = stylesheet.visit(&mut self.class_visitor);
            let opt = PrinterOptions {
                minify: true,
                ..Default::default()
            };
            if let Ok(f) = stylesheet.to_css(opt) {
                return Some(f.code);
            }
        }
        None
    }
}

struct HTMLWalker<'a> {
    pub class_visitor: &'a ClassVisitor,
    pub macros_walker: &'a MacroClassWalker<'a>,
}

impl<'a> HTMLWalker<'a> {
    pub fn new(
        class_visitor: &'a ClassVisitor,
        macros_walker: &'a MacroClassWalker,
    ) -> Self {
        Self {
            class_visitor,
            macros_walker,
        }
    }
}

impl<'a> TreeWalker for HTMLWalker<'a> {
    fn walk(&mut self, mut old_content: String, path: &Path) -> Option<String> {
        if let Some(updated_macros) =
            self.macros_walker.macros.get(path.to_str().unwrap())
        {
            for m in updated_macros {
                old_content = old_content.replace(m.0, m.1);
            }
        }
        if let Ok(html) = check_html(&old_content, self.class_visitor) {
            return Some(html);
        }
        None
    }
}

struct JSWalker<'a> {
    pub class_visitor: &'a ClassVisitor,
}

impl<'a> JSWalker<'a> {
    pub fn new(class_visitor: &'a ClassVisitor) -> Self {
        Self { class_visitor }
    }
}

impl<'a> TreeWalker for JSWalker<'a> {
    fn walk(&mut self, old_content: String, _: &Path) -> Option<String> {
        if let Some(html) = check_js(&old_content, self.class_visitor) {
            if let Ok(html) = String::from_utf8(html) {
                return Some(html);
            }
        }
        None
    }
}

pub struct MacroClassWalker<'a> {
    pub class_visitor: &'a ClassVisitor,
    pub macros: HashMap<String, BTreeMap<String, String>>,
}

impl<'a> MacroClassWalker<'a> {
    pub fn new(class_visitor: &'a ClassVisitor) -> Self {
        Self {
            class_visitor,
            macros: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: String, map: BTreeMap<String, String>) {
        self.macros.insert(path, map);
    }
}

impl<'a> TreeWalker for MacroClassWalker<'a> {
    fn walk(&mut self, old_content: String, path: &Path) -> Option<String> {
        check_macro(old_content, self.class_visitor, self, path)
    }
}
