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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::string_interner::StringInterner;

    #[test]
    fn test_same_input_gets_same_output() {
        let mut si = StringInterner::default();
        let r1 = si.insert_or_get(b"hello");
        let r2 = si.insert_or_get(b"hello");

        assert!(Rc::ptr_eq(&r1, &r2));
        assert_eq!(Rc::strong_count(&r1), 3);
        assert_eq!(si.map.len(), 1);
    }

    #[test]
    fn test_different_input_gets_new_string() {
        let mut si = StringInterner::default();
        let r1 = si.insert_or_get(b"hell");
        let r2 = si.insert_or_get(b"hello");

        assert!(!Rc::ptr_eq(&r1, &r2));
        assert_eq!(Rc::strong_count(&r1), 2);
        assert_eq!(Rc::strong_count(&r2), 2);
        assert_eq!(si.map.len(), 2);
    }
}
