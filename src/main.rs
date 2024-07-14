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
      arr2:
        - zaza
    "#;

    let mut docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &mut docs[0];

    println!("Before: {:?}", doc);

    walk(doc, "name");

    println!("After: {:?}", doc);

}