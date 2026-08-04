use std::{collections::HashMap, rc::Rc};

#[derive(Debug, Default)]
pub struct StringInterner {
    map: HashMap<Vec<u8>, Rc<String>>,
}

impl StringInterner {
    pub fn insert_or_get(&mut self, key: &[u8]) -> Rc<String> {
        if let Some(s) = self.map.get(key) {
            s.clone()
        } else {
            let val = Rc::new(String::from_utf8_lossy(key).into_owned());
            self.map.insert(key.to_vec(), val.clone());
            val
        }
    }
}
