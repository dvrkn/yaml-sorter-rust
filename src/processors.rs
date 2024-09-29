// processors.rs
use yaml_rust2::yaml::{Array, Hash, Yaml};

pub fn process_yaml(doc: &mut Yaml, config: &Yaml) {
    match doc {
        Yaml::Array(v) => {
            array_sorter(v, &config["sortKey"].as_str().unwrap());
            for x in v {
                process_yaml(x, config);
            }
        }
        Yaml::Hash(h) => {
            hash_sorter(h, &config["preOrder"].as_vec().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect::<Vec<String>>());
            for (_, v) in h {
                process_yaml(v, config);
            }
        }
        _ => {}
    }
}

pub fn hash_sorter(hash: &mut Hash, pre_order: &[String]) {
    let mut result = Hash::new();

    // Sort the hash by the pre_order array
    for key in pre_order {
        if let Some((k, v)) = hash.remove_entry(&Yaml::String(key.clone())) {
            result.insert(k, v);
        }
    }

    // Sort the remaining hash
    let mut hash_keys: Vec<Yaml> = hash.keys().cloned().collect();
    hash_keys.sort_by(|a, b| a.cmp(b));
    for key in hash_keys {
        if let Some((k, v)) = hash.remove_entry(&key) {
            result.insert(k, v);
        }
    }

    *hash = result;
}

pub fn array_sorter(array: &mut Array, sort_key: &str) {
    array.sort_by(|a, b| {
        match (a[sort_key].as_str(), b[sort_key].as_str()) {
            (Some(a_str), Some(b_str)) => a_str.cmp(b_str),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
}