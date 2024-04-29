/// coding the query_string struct and its functionality
use std::collections::HashMap;

struct QueryString<'a> {
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