use std::{
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeMap},
};

type OneClass = [u8; 5];

pub enum CSSToken {
    Class,
    CustomProperty,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ClassContainer {
    class_name: ClassIter,
    container: [BTreeMap<String, String>; 2],
}

impl ClassContainer {
    pub fn add(&mut self, key: String, token: CSSToken) {
        let index = token as usize;
        if let Some(map) = self.container.get_mut(index) {
            if let Entry::Vacant(e) = map.entry(css_to_html(&key)) {
                if let Some(v) = self.class_name.next() {
                    e.insert(new_class(&v));
                }
            }
        }
    }

    pub fn get(&self, key: String, container: CSSToken) -> Option<String> {
        let index = container as usize;
        if let Some(map) = self.container.get(index) {
            if let Some(v) = map.get(&key) {
                return Some(v.to_string());
            }
        }
        None
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub struct ClassIter {
    array: OneClass,
    current_index: usize,
}

impl ClassIter {}

impl Iterator for ClassIter {
    type Item = OneClass;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.array;
        self.array[self.current_index] += 1;
        if self.array[self.current_index] == 27 {
            self.array[self.current_index] = 1;
            let mut add_new = true;
            for i in (0..self.current_index).rev() {
                let v = self.array[i];
                match v.cmp(&26) {
                    Ordering::Less => {
                        add_new = false;
                        self.array[i] += 1;
                        self.array[self.current_index] = 1;
                        break;
                    }
                    Ordering::Equal => {
                        self.array[i] = 1;
                    }
                    Ordering::Greater => (),
                }
            }
            if add_new {
                if self.current_index == 4 {
                    return None;
                }

                for i in (0..self.current_index).rev() {
                    self.array[i] = 1;
                }

                self.current_index += 1;
                self.array[self.current_index] = 1;
            }
        }
        Some(current)
    }
}

fn css_to_html(old: &str) -> String {
    if old.starts_with('.') {
        let mut chars = old.chars();
        chars.next();
        chars.as_str().replace('\\', "")
    } else {
        String::from(old)
    }
}

fn new_class(new_class: &OneClass) -> String {
    let mut s = String::new();
    for i in new_class {
        if i == &0 {
            break;
        }
        let c = 96_u8 + i;
        s.push(c as char);
    }
    s
}
