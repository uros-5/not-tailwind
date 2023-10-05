use std::string::FromUtf8Error;

use lol_html::{self, element, HtmlRewriter, Settings};

use crate::visit_selectors::ClassVisitor;

pub fn check_html(document: &str, msv: &ClassVisitor) -> Result<String, FromUtf8Error> {
    let mut output = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("[class]", |el| {
                let classes = el.get_attribute("class").unwrap();
                let classes = classes.split(' ');
                let classes = classes
                    .into_iter()
                    .map(|c| {
                        if let Some(class) = msv.get(c) {
                            return class;
                        }
                        c.to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                let _r = el.set_attribute("class", &classes);

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
