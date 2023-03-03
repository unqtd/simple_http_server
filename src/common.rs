pub type Body = Vec<u8>;

pub mod headers {
    use std::collections::HashMap;

    pub type Name = String;
    pub type Value = String;

    pub type Headers = HashMap<Name, Value>;
}
