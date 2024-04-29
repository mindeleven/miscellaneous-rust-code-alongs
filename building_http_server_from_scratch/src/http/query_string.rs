/// coding the query_string struct and its functionality
use std::collections::HashMap;

pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

// for some values we need a string for others an array
// we're using a vector to store multiple values because 
// we need a heap allocated array that can have a varying number of values
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

// converting incoming string from request into a QueryString struct
// string structure example:
// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();

        // splitting up the string
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];

            }

            // is the key already in the hashmap?
            data.entry(key)
                // if yes 
                // and_modify accepts hashmap itself as parameter
                // it provides in-place mutable access to an occupied entry 
                // before any potential inserts into the map
                .and_modify(|existing: &mut Value| {
                    match existing {
                        Value::Single(prev_val) => {
                            // existing is a pointer that gets derefferenced with * so
                            // it can be overwritten with a value that points somewhere else
                            *existing = Value::Multiple(vec![prev_val, val]);
                        },
                        Value::Multiple(vec) => { vec.push(val)}
                    }
                })
                // if no, insert with or_insert()
                .or_insert(Value::Single(val));
                

        }

        QueryString { data };

        unimplemented!()
    }

}