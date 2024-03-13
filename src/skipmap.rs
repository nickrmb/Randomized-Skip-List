use crate::*;
use std::rc::Rc;

// Entry struct, holds key and an optional value
pub struct Entry<K: Ord, V> {
    key: K,
    value: Option<Rc<V>>,
}

// Ordering Traits
impl<K: Ord, V> PartialEq for Entry<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}
impl<K: Ord, V> Eq for Entry<K, V> {}
impl<K: Ord, V> PartialOrd for Entry<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.key.cmp(&other.key))
    }
}
impl<K: Ord, V> Ord for Entry<K, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| std::cmp::Ordering::Greater)
    }
}

// Map Contruct based on a skiplist
pub struct SkipMap<K: Ord, V>
where
    K: Copy,
{
    skiplist: SkipList<Entry<K, V>>,
}

impl<K: Ord, V> SkipMap<K, V>
where
    K: Copy,
{
    // put value associated to some key, returns previous value associated to key if exists
    pub fn put<'a>(&mut self, key: &K, value: V) -> Option<Rc<V>> {
        // create new entry and insert it / replace it
        let r = self.skiplist.insert_or_replace(Entry {
            key: *key,
            value: Some(Rc::new(value)),
        });

        // check if we replaced a value
        if let Some(x) = r {
            if let Some(y) = &x.value {
                // should always go into this if
                return Some(Rc::clone(y));
            }
        }

        None
    }

    // get value associated to key if it exists
    pub fn get(&self, key: &K) -> Option<Rc<V>> {
        // create pseudo entry for comparison
        let a = Entry {
            key: *key,
            value: None,
        };
        
        // search for entry (key) in skiplist
        let a = self.skiplist.find(&a);

        // check if value was found
        if let Some(x) = a {
            if let Some(y) = &x.value {
                // return value inside found entry
                return Some(Rc::clone(y));
            }
        }

        None
    }

    // delete value associated to key, return it if it was found
    pub fn del(&mut self, key: &K) -> Option<Rc<V>> {
        // create pseudo key for comparison
        let a = Entry {
            key: *key,
            value: None,
        };

        // delete entry (key) from skiplist
        let a = self.skiplist.delete(&a);

        // if entry was deleted we get value inside
        if let Some(x) = a {
            if let Some(y) = &x.value {
                // return value in entry
                return Some(Rc::clone(y));
            }
        }

        None
    }

    // iterator over entries
    pub fn iter(&self) -> MapIterator<K, V> {
        MapIterator {
            iter: self.skiplist.iter(),
        }
    }

    // find node that contains entry associated with key (if exists)
    pub fn find(&self, key: &K) -> Option<Rc<Node<Entry<K, V>>>> {
        let a = Entry {
            key: *key,
            value: None,
        };
        return self.skiplist.find_node(&a);
    }
}

pub struct SEntry<K,V> {
    pub key: K,
    pub value: Rc<V>
}

pub struct MapIterator<K, V>
where
    K: Ord,
    K: Copy,
{
    iter: NodeIterator<Entry<K, V>>,
}

impl<K, V> Iterator for MapIterator<K, V>
where
    K: Ord,
    K: Copy,
{
    type Item = SEntry<K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iter.next();
        if let Some(x) = n {
            if let Some(y) = &x.value {
                return Some(SEntry { key: x.key, value: Rc::clone(y) });
            }
        }

        None
    }
}
