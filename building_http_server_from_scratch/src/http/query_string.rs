/// coding the query_string struct and its functionality
use std::collections::HashMap;

struct QueryString<'a> {
    data: HashMap<&'a str, &'a str>,
}

