use std::{
    fs::{self, read_to_string},
    path::Path,
};

use lightningcss::{
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    visitor::Visit,
};

use crate::{
    config::{read_config, Config},
    visit_class::check_html,
    visit_selectors::ClassVisitor,
};

pub fn start_all() {
    let config = read_config();
    match config {
        Ok(config) => {
            let validate = config.validate();
            if config.validate().is_ok() {
                let mut css_walker = CSSWalker::default();

                for dir in &config.css_dir {
                    css_walker.walk_tree(dir, &config);
                }

                let mut html_walker =
                    HTMLWalker::new(&css_walker.class_visitor);

                for dir in &config.html_dir {
                    html_walker.walk_tree(dir, &config);
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
    fn walk(&mut self, old_content: String) -> Option<String>;

    fn write(&self, path: &Path, new_content: Option<&str>, config: &Config) {
        let output_path = Path::new(&config.output_dir);
        let new_path = output_path.join(handle_path(path));
        if let Some(parent) = new_path.parent() {
            dbg!(path);
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
                            if let Some(new_content) = self.walk(old_content) {
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
                        self.walk_tree(dir, config);
                    }
                }
            }
        }
    }
}

#[derive(Default)]
struct CSSWalker {
    pub class_visitor: ClassVisitor,
}

impl TreeWalker for CSSWalker {
    fn walk(&mut self, old_content: String) -> Option<String> {
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
}

impl<'a> HTMLWalker<'a> {
    pub fn new(class_visitor: &'a ClassVisitor) -> Self {
        Self { class_visitor }
    }
}

impl<'a> TreeWalker for HTMLWalker<'a> {
    fn walk(&mut self, old_content: String) -> Option<String> {
        if let Ok(html) = check_html(&old_content, self.class_visitor) {
            return Some(html);
        }
        None
    }
}
