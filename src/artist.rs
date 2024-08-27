use crate::*;

struct Artist {
    id: i64,
    name: String,
}

impl Artist {
    fn new(id: i64, name: String) -> Self {
        Artist { id: (id), name: (name) }
    }
    
}