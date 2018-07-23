use indexmap::IndexSet;
use std::collections::BTreeMap;

pub type TableRow<K, V> = BTreeMap<K, V>;
pub type Table<K, V> = Vec<TableRow<K, V>>;
pub type MultiTables<K, V> = Vec<Table<K, V>>;
pub type Headers = IndexSet<String>;
