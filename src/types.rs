use indexmap::IndexSet;
use linked_hash_map::LinkedHashMap;

pub type TableRow<K, V> = LinkedHashMap<K, V>;
pub type Table<K, V> = Vec<TableRow<K, V>>;
pub type MultiTables<K, V> = Vec<Table<K, V>>;
pub type Headers = IndexSet<String>;
pub type NamedTable<K, V> = (String, Table<K, V>);
pub type ErroredTable     = (String, String);
