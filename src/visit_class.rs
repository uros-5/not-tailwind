use std::string::FromUtf8Error;

use lol_html::{self, element, HtmlRewriter, Settings};

use crate::visit_selectors::ClassVisitor;

pub fn check_html(
    document: &str,
    msv: &ClassVisitor,
) -> Result<String, FromUtf8Error> {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("[class]", |el| {
                let classes = el.get_attribute("class").unwrap();
                let mut in_expr = false;
                let classes = classes.split(' ');
                let classes = classes
                    .into_iter()
                    .map(|c| {
                        if c.ends_with("{{") {
                            in_expr = true;
                        } else if c.ends_with("}}") {
                            in_expr = false
                        }
                        let single = c.contains('\'');
                        let double = c.contains('"');
                        if in_expr && !single && !double {
                            return c.to_string();
                        }
                        let c2 = c.replace(['\"', '\''], "");
                        if let Some(class) = msv.get(&c2) {
                            return c.replace(&c2, &class);
                        }
                        c.to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                let _r = el.set_attribute("class", &classes);
                el.remove_attribute("hx-lsp");

                Ok(())
            })],
            ..Settings::default()
        },
        |c: &[u8]| output.extend_from_slice(c),
    );
    let _ = rewriter.write(document.as_bytes());
    let _ = rewriter.end();
    String::from_utf8(output)
}
