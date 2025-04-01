use std::{fs::read_to_string, path::Path};

use crate::{
    args::NtwArgs, visit_class::check_html, visit_map::check_js,
    visit_selectors::ClassVisitor,
};
use ignore::Walk;
use lightningcss::{
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    visitor::Visit,
};

pub fn start_all2(arg: NtwArgs) {
    let mut css_walker = CSSWalker::new(arg.ignored);
    let output = arg.output.unwrap_or(String::from("not-tailwind"));
    css_walker.walk(&output);
    templates_walk(&output, arg.run, &css_walker.class_visitor);
    js_walk(&output, &css_walker.class_visitor);
}

fn js_walk(output: &str, class_visitor: &ClassVisitor) {
    for result in Walk::new("./") {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if !path.ends_with("not-tailwind.ts") {
                    continue;
                }

                let content = read_to_string(&entry.path()).unwrap_or_default();

                if let Some(new_content) = check_js(&content, class_visitor) {
                    if let Ok(js) = String::from_utf8(new_content) {
                        write_to_file(path, js, output);
                    }
                }
                break;
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
}

fn templates_walk(
    output: &str,
    file_types: Vec<String>,
    visitor: &ClassVisitor,
) {
    for result in Walk::new("./") {
        match result {
            Ok(entry) => {
                let ext = entry.path().extension().is_some_and(|v| {
                    file_types
                        .contains(&v.to_str().unwrap_or_default().to_string())
                });
                if ext == false {
                    continue;
                }
                let content = read_to_string(&entry.path()).unwrap_or_default();
                let new_content =
                    check_html(&content, visitor).unwrap_or_default();
                write_to_file(entry.path(), new_content, output);
            }
            Err(_) => {}
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

struct CSSWalker {
    pub class_visitor: ClassVisitor,
    pub ignored: Vec<String>,
}

impl CSSWalker {
    pub fn new(ignored: Option<Vec<String>>) -> Self {
        Self {
            class_visitor: ClassVisitor::default(),
            ignored: ignored.unwrap_or(vec![]),
        }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        if let Some(path) = path.to_str() {
            if !path.ends_with(".css") {
                return true;
            }
            return self.ignored.contains(&path.to_string());
        }
        false
    }

    pub fn walk(&mut self, output: &String) {
        for result in Walk::new("./") {
            match result {
                Ok(entry) => {
                    if self.is_ignored(entry.path()) {
                        continue;
                    }
                    let content =
                        read_to_string(&entry.path()).unwrap_or_default();
                    let a =
                        StyleSheet::parse(&content, ParserOptions::default());

                    if let Ok(mut stylesheet) = a {
                        let _ = stylesheet.visit(&mut self.class_visitor);
                        let opt = PrinterOptions {
                            minify: true,
                            ..Default::default()
                        };

                        if let Ok(f) = stylesheet.to_css(opt) {
                            write_to_file(&entry.path(), f.code, output);
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
}

fn write_to_file(path: &Path, code: String, output: &str) {
    let output_path = Path::new(output);
    let new_path = output_path.join(handle_path(path));
    if let Some(parent) = new_path.parent() {
        let _ = std::fs::create_dir_all(parent);
        let _ = std::fs::write(new_path, code);
    }
}
