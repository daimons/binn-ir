// License: see LICENSE file at root directory of `master` branch

//! # Container functions

use {
    crate::{Blob, List, Map, MapKey, Object, ObjectKey, Value},
};

/// # Makes new blob
pub fn blob() -> Value {
    Value::Blob(Blob::new())
}

/// # Makes new blob with capacity
pub fn blob_with_capacity(capacity: usize) -> Value {
    Value::Blob(Blob::with_capacity(capacity))
}

/// # Makes new list
pub fn list() -> Value {
    Value::List(List::new())
}

/// # Makes new list with capacity
pub fn list_with_capacity(capacity: usize) -> Value {
    Value::List(List::with_capacity(capacity))
}

/// # Pushes new value into a list
pub fn push<T>(list: &mut List, value: T) where T: Into<Value> {
    list.push(value.into());
}

/// # Makes new map
pub fn map() -> Value {
    Value::Map(Map::new())
}

/// # Makes new map from one pair of key/value
pub fn map_from<K, V>(key: K, value: V) -> Value where K: Into<MapKey>, V: Into<Value> {
    let mut map = Map::new();
    map_insert(&mut map, key, value);
    map.into()
}

/// # Inserts new item into a map
///
/// Returns previous value (if it existed).
pub fn map_insert<K, V>(map: &mut Map, key: K, value: V) -> Option<Value> where K: Into<MapKey>, V: Into<Value> {
    map.insert(key.into(), value.into())
}

/// # Makes new object
pub fn object() -> Value {
    Value::Object(Object::new())
}

/// # Makes new object from one pair of key/value
pub fn object_from<K, V>(key: K, value: V) -> Value where K: Into<ObjectKey>, V: Into<Value> {
    let mut object = Object::new();
    object_insert(&mut object, key, value);
    object.into()
}

/// # Inserts new item into an object
///
/// Returns previous value (if it existed).
pub fn object_insert<K, V>(object: &mut Object, key: K, value: V) -> Option<Value> where K: Into<ObjectKey>, V: Into<Value> {
    object.insert(key.into(), value.into())
}
