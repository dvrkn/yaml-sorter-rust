use std::fs::File;
use std::io::Read;
use std::sync::OnceLock;
use yaml_rust2::{YamlEmitter, YamlLoader};
use yaml_rust2::yaml::{Array, Yaml};

static CONFIG: OnceLock<Yaml> = OnceLock::new();

fn load_config() -> Yaml {
    let mut file = File::open("config.yaml").expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read config file");
    let docs = YamlLoader::load_from_str(&contents).expect("Unable to parse config file");
    docs[0].clone()
}


fn walk(doc: &mut Yaml) {
    match doc {
        Yaml::Array(ref mut v) => {
            array_sorter(v);
            for x in v {
                walk(x);
            }
        }
        Yaml::Hash(ref mut h) => {
            hash_sorter(h);
            for (_, v) in h {
                walk(v);
            }
        }
        _ => {}
    }
}

fn hash_sorter(hash: &mut yaml_rust2::yaml::Hash) {
    let pre_order_array = CONFIG.get().unwrap()["preOrder"].as_vec().unwrap();
    let mut result = yaml_rust2::yaml::Hash::new();

    // Sort the hash by the pre_order_array
    for key in pre_order_array {
        if let Some((k, v)) = hash.remove_entry(key) {
            result.insert(k, v);
        }
    }

    // Sort the remaining hash
    let mut hash_keys: Vec<_> = hash.keys().cloned().collect();
    hash_keys.sort_by(|a, b| a.cmp(b));
    for key in hash_keys {
        if let Some((k, v)) = hash.remove_entry(&key) {
            result.insert(k, v);
        }
    }

    *hash = result;

}

fn array_sorter(array: &mut Array) {
    let key = CONFIG.get().unwrap()["sortKey"].as_str().unwrap();

    array.sort_by(|a, b| {
        match (a[key].as_str(), b[key].as_str()) {
            (Some(a_str), Some(b_str)) => a_str.cmp(b_str),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
}

fn main() {
    CONFIG.set(load_config()).expect("Unable to set config");

    let s = r#"
        test: yaml
        namespace: argocd
        name: test
        apiVersion: argoproj.io/v1alpha1
        arr:
          - test: without sort key
          - test: yaml
            namespace: argocd
            name: test
            apiVersion: argoproj.io/v1alpha1
            arr2:
              - namespace: argocd
                name: test
                apiVersion: argoproj.io/v1alpha1
                test: yaml
                enabled: false
          - enabled: false
            name: arr
        enabled: false
        spec:
          generators:
            - list:
                elements:
                  - name: c
                  - name: b
                  - name: a
                    ord:
                      - name: c
                      - name: b
                      - name: a

          name: cluster-resources

        # Comment
        anchor: &test
          - anchor
        anchor-test: *test
    "#;

    let mut docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &mut docs[0];

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("Before:\n{}\n\n", out_str);

    walk(doc);

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("After:\n{}", out_str);

}