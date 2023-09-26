use std::{cmp::Ordering, collections::BTreeMap, fs::File, io::Write};

pub struct ClassContainer {
    classes: BTreeMap<String, [u8; 5]>,
    class_name: ClassName,
}

impl Default for ClassContainer {
    fn default() -> Self {
        Self {
            classes: BTreeMap::default(),
            class_name: ClassName::default(),
        }
    }
}

impl ClassContainer {
    pub fn add(&mut self, key: String) {
        if !self.classes.contains_key(&key) {
            if let Some(v) = self.class_name.next() {
                self.classes.insert(key, v);
            }
        }
    }

    pub fn show(&self) {
        for i in &self.classes {
            println!("{}", i.0);
            println!("{}", ClassName::convert(*i.1));
        }
    }

    pub fn to_file(&self, mut content: String) {}
}

pub struct ClassName {
    array: [u8; 5],
    current_index: usize,
}

impl ClassName {
    fn convert(a: [u8; 5]) -> String {
        let mut s = String::new();
        for i in a {
            if i == 0 {
                break;
            }
            let c = 96_u8 + i;
            s.push(c as char);
        }
        s
    }
}

impl Default for ClassName {
    fn default() -> Self {
        ClassName {
            array: [1, 0, 0, 0, 0],
            current_index: 0,
        }
    }
}

impl Iterator for ClassName {
    type Item = [u8; 5];

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
                //
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
