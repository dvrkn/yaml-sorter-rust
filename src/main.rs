use yaml_rust2::{YamlEmitter, YamlLoader};
use yaml_rust2::yaml::{Array, Yaml};

fn walk(doc: &mut Yaml, key: &str, pre_order_yaml: &str) {
    match doc {
        Yaml::Array(ref mut v) => {
            array_sorter(v, key);
            for x in v {
                walk(x, key, pre_order_yaml);
            }
        }
        Yaml::Hash(ref mut h) => {
            hash_sorter(h, pre_order_yaml);
            for (_, v) in h {
                walk(v, key, pre_order_yaml);
            }
        }
        _ => {}
    }
}

fn hash_sorter(hash: &mut yaml_rust2::yaml::Hash, pre_order_yaml: &str) {
    let pre_order_docs = YamlLoader::load_from_str(pre_order_yaml).unwrap();
    let pre_order_doc = &pre_order_docs[0];
    let pre_order_array = pre_order_doc.as_vec().unwrap();
    let mut result = yaml_rust2::yaml::Hash::new();

    // Sort the hash by the pre_order_array
    for key in pre_order_array {
        if hash.contains_key(key) {
            result.insert(key.clone(), hash.remove(key).unwrap());
        }
    }

    // Sort the rest of the hash
    let mut entries: Vec<_> = hash.into_iter().collect();
    entries.sort_by_key(|entry| entry.0);
    for (key, value) in entries {
        result.insert(key.clone(), value.clone());
    }

    *hash = result;

}

fn array_sorter(array: &mut Array, key: &str) {
    array.sort_by(|a, b| {
        let a_str = a[key].as_str();
        let b_str = b[key].as_str();
        if a_str.is_some() && b_str.is_some() {
            let a =a_str.unwrap();
            let b = b_str.unwrap();
            return a.cmp(b);
        }
        std::cmp::Ordering::Equal
    });
}

fn main() {
    let pre_order = r#"
      - enabled
      - apiVersion
      - kind
      - metadata
      - name
      - namespace
      - labels
      - annotations
      - goTemplate
      - generators
      - spec
      - containers
      - image
      - ports
      - protocol
      - resources
      - limits
      - requests
      - cpu
      - memory
      - volumeMount
      - destination
      - sources
    "#;

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

    walk(doc, "name", pre_order);

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("After:\n{}", out_str);

}