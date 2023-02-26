use std::collections::HashMap;

use libactionkv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE: &str = "
Usage:
    akv_disk.exe FILE get KEY
    akv_disk.exe FILE delete KEY
    akv_disk.exe FILE insert KEY VALUE
    akv_disk.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_disk FILE get KEY
    akv_disk FILE delete KEY
    akv_disk FILE insert KEY VALUE
    akv_disk FILE update KEY VALUE
";

type ByteString = Vec<u8>;
type ByteStr = [u8];

fn store_index_on_disk(kv: &mut ActionKV, index_key: &ByteStr) {
    kv.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&kv.index).unwrap();
    kv.index = HashMap::new();
    kv.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    let args: Vec<String> = std::env::args().collect();

    let fname = args.get(1).expect(USAGE);
    let action = args.get(2).expect(USAGE).as_ref();
    let key = args.get(3).expect(USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(fname);
    let mut store = ActionKV::open(path).expect("unable to open the file");

    store.load().expect("unable to load data");

    match action {
        "get" => {
            let index_as_bytes = store.get(INDEX_KEY).unwrap().unwrap();
            let index_decoded: Result<HashMap<ByteString, u64>, Box<bincode::ErrorKind>> =
                bincode::deserialize(&index_as_bytes);
            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found key", key),
                Some(&index) => {
                    let kv = store.get_at(index).unwrap();
                    println!("{:?}", kv.value)
                }
            }
        }
        "delete" => store.delete(key).unwrap(),
        "insert" => {
            let value = maybe_value.expect(USAGE).as_ref();
            store.insert(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        "update" => {
            let value = maybe_value.expect(USAGE).as_ref();
            store.update(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        _ => eprintln!("{}", &USAGE),
    }
}
