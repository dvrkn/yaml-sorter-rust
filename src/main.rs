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
        _ => {}
    }
}

fn replacer(array: &mut Array, key: &str) {
    array.sort_by_key(|x| x[key].as_str().unwrap().to_string());
}

fn main() {
    let s = r#"
      - name: c
        value: 3
      - name: b
        value: 2
      - name: a
        value: 1
    "#;

    let mut docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &mut docs[0];

    println!("Before sorting: {}", doc[0]["name"].as_str().unwrap());

    walk(doc, "name");

    println!("After sorting: {}", doc[0]["name"].as_str().unwrap());
}