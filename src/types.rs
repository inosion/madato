use linked_hash_map::LinkedHashMap;
use regex::Regex;
use thiserror::Error;

/*
 * Table / internal data types.
 */
pub type TableRow<K, V> = LinkedHashMap<K, V>;
pub type Table<K, V> = Vec<TableRow<K, V>>;
pub type MultiTables<K, V> = Vec<Table<K, V>>;
pub type Headers = Vec<String>;
pub type NamedTable<K, V> = (String, Table<K, V>);
pub type ErroredTable = (String, String);

#[derive(Error, Debug)]
pub enum MadatoError {
    #[error("Problem reading file: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Problem parsing YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Problem parsing JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Problem parsing CSV: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Problem with conversion: {0}")]
    DataError(#[from] std::string::FromUtf8Error),

    #[error("Problem parsing XLSX, ODS: {0}")]
    CalError(String),
}

/*
 * Filtering
 */

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KVFilter {
    #[serde(with = "build_regex")]
    pub key_re: Regex,

    #[serde(with = "build_regex")]
    pub value_re: Regex,
}

mod build_regex {
    use serde::{self, Deserialize, Deserializer, Serializer};

    use regex::Regex;

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(re: &Regex, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", re.to_string());
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Regex, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Regex::new(&s).map_err(serde::de::Error::custom)
    }
}

impl KVFilter {
    pub fn new(key: String, value: String) -> KVFilter {
        let key_re = Regex::new(&key).unwrap();
        let value_re = Regex::new(&value).unwrap();

        KVFilter {
            key_re: key_re,
            value_re: value_re,
        }
    }
}
/**e
 * The API generally will support the RenderOptions
 */
#[derive(Default, Clone)]
pub struct RenderOptions {
    /// Filters to apply to the data
    pub filters: Option<Vec<KVFilter>>,

    /// Column headings to use
    pub headings: Option<Headers>,

    /// When XLSX, ODS, the sheet name to use
    pub sheet_name: Option<String>,
}
