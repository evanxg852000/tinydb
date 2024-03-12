use crate::kv::ColumnFamily;


// Mutation is a single modification to the TinyKV's 
// underlying storage.
pub enum Mutation {
    Put{key: Vec<u8>, value: Vec<u8>, cf: ColumnFamily},
    Delete{key: Vec<u8>, cf: ColumnFamily},
}

impl Mutation {
    pub fn key(&self) -> Vec<u8> {
        match self {
            Mutation::Put { key, .. } => key.clone(),
            Mutation::Delete { key, .. } => key.clone(),
        }
    }

    pub fn value(&self) -> Option<Vec<u8>> {
        match self {
            Mutation::Put { value, .. } => Some(value.clone()),
            _ => None,
        }
    }

    pub fn column_family(&self) -> ColumnFamily {
        match self {
            Mutation::Put { cf, .. } => cf.clone(),
            Mutation::Delete { cf, .. } => cf.clone(),
        }
    }
}
