use yaml_rust2::{YamlLoader};
use yaml_rust2::yaml::{Array, Yaml};

fn walk(doc: &mut Yaml, key: &str) {
    match doc {
        Yaml::Array(ref mut v) => {
            replacer(v, key);
            for x in v {
                walk(x, key);
            }
        }
        Yaml::Hash(ref mut h) => {
            for (_, v) in h {
                walk(v, key);
            }
        }
        _ => {}
    }
}

fn replacer(array: &mut Array, key: &str) {
    array.sort_by_key(|x| x[key].as_str().unwrap().to_string());
}

fn main() {
    let s = r#"
      arr:
      - name: c
        value: 3
      - name: b
        value: 2
      - name: a
        value: 1
        arr:
        - name: z
          value: 99
        - name: y
          value: 88
        - name: x
          value: 77
    "#;

    let mut docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &mut docs[0];

    // pretty print all the documents in the YAML file
    println!("Before: {:?}", doc);

    walk(doc, "name");

    println!("After: {:?}", doc);

}