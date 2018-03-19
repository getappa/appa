use rocksdb::DB;

pub struct RocksDbStorage {
    pub conn: DB
}

impl Storage {
    pub fn new(path: String) -> RocksDbStorage {
        RocksDbStorage{
            conn: DB::open_default(path).unwrap()
        }
    }

    pub fn add(&self, key: String, value: String) {
        
    }
}