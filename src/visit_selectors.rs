use lightningcss::{
    selector::Selector,
    values::string::CowArcStr,
    visit_types,
    visitor::{VisitTypes, Visitor},
};
use parcel_selectors::parser::Component;

use crate::short_classes::{CSSToken, ClassContainer};
use lightningcss::stylesheet::StyleSheet;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ClassVisitor {
    container: ClassContainer,
}

impl ClassVisitor {
    pub fn show(&self, stylesheet: StyleSheet) {
        self.container.into_file(stylesheet);
    }

    pub fn get(&self, class: &str) -> Option<String> {
        self.container.get(class.to_owned(), CSSToken::Class)
    }

    pub fn get_custom_property(&self, class: &str) -> Option<String> {
        self.container
            .get(class.to_owned(), CSSToken::CustomProperty)
    }
}

impl<'i> Visitor<'i> for ClassVisitor {
    type Error = ();

    fn visit_types(&self) -> VisitTypes {
        visit_types!(SELECTORS | DASHED_IDENTS)
    }

    #[allow(clippy::collapsible_match)]
    fn visit_selector(
        &mut self,
        selector: &mut Selector<'i>,
    ) -> Result<(), Self::Error> {
        let iter = selector.iter_mut_raw_match_order();
        for i in iter {
            match i {
                Component::Class(c) => {
                    if let Some(n) =
                        self.container.add(c.0.to_string(), CSSToken::Class)
                    {
                        c.0 = CowArcStr::from(n);
                    }
                }
                Component::Negation(selectors)
                | Component::Is(selectors)
                | Component::Where(selectors)
                | Component::Has(selectors)
                | Component::Any(_, selectors) => {
                    for i in selectors.iter_mut() {
                        self.visit_selector(i)?;
                    }
                }
                Component::Slotted(selectors) => {
                    self.visit_selector(selectors)?;
                }
                Component::Host(selector) => {
                    if let Some(selector) = selector {
                        self.visit_selector(selector)?;
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn visit_dashed_ident(
        &mut self,
        ident: &mut lightningcss::values::ident::DashedIdent,
    ) -> Result<(), Self::Error> {
        if let Some(n) = self.container.add(
            ident.0.to_string(),
            crate::short_classes::CSSToken::CustomProperty,
        ) {
            let n = format!("--{}", n);
            ident.0 = CowArcStr::from(n);
        }

        Ok(())
    }
}
