use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Debug, Default)]
pub struct StringInterner {
    map: RwLock<HashMap<Vec<u8>, Arc<String>>>,
}

const PL: &str = "Poisoned lock";

impl StringInterner {
    pub fn insert_or_get<T: AsRef<[u8]>>(&self, key: T) -> Arc<String> {
        if let Some(s) = self.map.read().expect(PL).get(key.as_ref()) {
            s.clone()
        } else {
            let val = Arc::new(String::from_utf8_lossy(key.as_ref()).into_owned());
            self.map
                .write()
                .expect(PL)
                .insert(key.as_ref().to_vec(), val.clone());
            val
        }
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.map.read().expect(PL).len()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::string_interner::StringInterner;

    #[test]
    fn test_same_input_gets_same_output() {
        let si = StringInterner::default();
        let r1 = si.insert_or_get(b"hello");
        let r2 = si.insert_or_get(b"hello");

        assert!(Arc::ptr_eq(&r1, &r2));
        assert_eq!(Arc::strong_count(&r1), 3);
        assert_eq!(si.len(), 1);
    }

    #[test]
    fn test_different_input_gets_new_string() {
        let si = StringInterner::default();
        let r1 = si.insert_or_get(b"hell");
        let r2 = si.insert_or_get(b"hello");

        assert!(!Arc::ptr_eq(&r1, &r2));
        assert_eq!(Arc::strong_count(&r1), 2);
        assert_eq!(Arc::strong_count(&r2), 2);
        assert_eq!(si.len(), 2);
    }
}
