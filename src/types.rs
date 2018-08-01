use linked_hash_map::LinkedHashMap;
use regex::Regex;

/*
 * Table / internal data types.
 */
pub type TableRow<K, V> = LinkedHashMap<K, V>;
pub type Table<K, V> = Vec<TableRow<K, V>>;
pub type MultiTables<K, V> = Vec<Table<K, V>>;
pub type Headers = Vec<String>;
pub type NamedTable<K, V> = (String, Table<K, V>);
pub type ErroredTable = (String, String);

/*
 * Filtering
 */

#[derive(Clone)]
pub struct KVFilter {
    pub key: Regex,
    pub value: Regex,
}

impl KVFilter {
    pub fn new(key: String, value: String) -> KVFilter {
        let key_re = Regex::new(&key).unwrap();
        let value_re = Regex::new(&value).unwrap();

        KVFilter {
            key: key_re,
            value: value_re,
        }
    }
}
/**e
 * The API generally will support the RenderOptions
 */
#[derive(Default, Clone)]
pub struct RenderOptions {
    pub filters: Option<Vec<KVFilter>>,
    pub headings: Option<Headers>,
    pub sheets: Option<Vec<String>>,
}
