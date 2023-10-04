use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use lightningcss::{
    stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
    visitor::Visit,
};

use crate::{
    config::{read_config, Config},
    visit_selectors::PreSelectorVisitor,
};

pub fn start_all() {
    let config = read_config();
    match config {
        Ok(config) => {
            let mut msv = PreSelectorVisitor::default();
            create_new_css(&config, &mut msv);
        }
        Err(e) => {}
    }
    // if let Ok(s) = read_to_string("chooser.txt") {}
}

pub fn create_new_css(config: &Config, msv: &mut PreSelectorVisitor) {
    for dir in &config.css_dir {
        walk_css(dir, config, msv);
    }
}

fn walk_css(dir: &String, config: &Config, msv: &mut PreSelectorVisitor) {
    if let Ok(paths) = std::fs::read_dir(dir) {
        for path in paths.flatten() {
            if let Ok(meta) = path.metadata() {
                if meta.is_file() {
                    if let Ok(s) = read_to_string(path.path()) {
                        let a = StyleSheet::parse(&s, ParserOptions::default());
                        if let Ok(mut stylesheet) = a {
                            let _ = stylesheet.visit(msv);
                            let opt = PrinterOptions {
                                minify: true,
                                ..Default::default()
                            };
                            if let Ok(f) = stylesheet.to_css(opt) {
                                let output_path = Path::new(&config.output_dir);
                                let p = output_path.join(handle_path(&path.path()));
                                if let Some(parent) = p.parent() {
                                    let _ = std::fs::create_dir_all(parent);
                                    let _ = std::fs::write(p, &f.code);
                                }
                            }
                        }
                    }
                } else if meta.is_dir() {
                    walk_css(dir, config, msv);
                }
            }
        }
    }
}

pub fn create_html(config: &Config) {
    //
}

pub fn create_js(config: &Config) {
    //
}

pub fn create_svg(config: &Config) {
    //
}

fn handle_path(path: &PathBuf) -> &Path {
    if path.starts_with(".") {
        if let Some(path2) = path.to_str() {
            let mut path2 = path2.chars();
            path2.next();
            path2.next();
            return Path::new(path2.as_str());
        }
    }
    path.as_path()
    // Path::from(path.to_str())
}
