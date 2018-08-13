#[allow(unused_macros)]
#[macro_export]
macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! treemap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::BTreeMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! linkedhashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::linked_hash_map::LinkedHashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! indexset {
    ($( $key: expr ),*) => {{
         let mut map = ::indexmap::IndexSet::new();
         $( map.insert($key); )*
         map
    }}
}

/*
 * https://gist.github.com/kardeiz/26c303957fc298212c3623c01a26f38c
 */
pub trait StripMargin {
    fn strip_margin(self) -> String;
}

impl StripMargin for &'static str {
    fn strip_margin(self) -> String {
        let mut out = Vec::new();
        for l in self.lines().filter(|x| !x.is_empty()) {
            if let Some(s) = l.splitn(2, '|').nth(1) {
                out.push(s);
            }
        }
        out.join("\n")
    }
}
