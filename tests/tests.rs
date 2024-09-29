use yaml_sorter_rust::config::{load_config_from_file};
use yaml_sorter_rust::processors::{process_yaml};
use yaml_rust2::{YamlLoader, Yaml, YamlEmitter};

pub fn init_test_config(config_path: &str) -> Yaml {
    load_config_from_file(config_path)
}

#[test]
fn test_load_config() {
    let config = init_test_config("config.yaml");

    assert!(config["preOrder"].is_array());
    assert_eq!(config["sortKey"].as_str().unwrap(), "name");
}


#[test]
fn test_hash_sorter() {
    let config = init_test_config("config.yaml");

    let test_str = r#"
        c: 3
        b: 2
        a: 1
        "#;

    let mut docs = YamlLoader::load_from_str(test_str).unwrap();
    let doc = &mut docs[0];
    process_yaml(doc, &config);
    println!("{:?}", doc);
    assert_eq!(doc, &Yaml::Hash(
        vec![
            (Yaml::String("a".to_string()), Yaml::Integer(1)),
            (Yaml::String("b".to_string()), Yaml::Integer(2)),
            (Yaml::String("c".to_string()), Yaml::Integer(3)),
        ].into_iter().collect()
    ));
}

#[test]
fn test_array_sorter() {
    let config = init_test_config("config.yaml");
    let test_str = r#"
        - name: Bob
        - name: Alice
        - name: Carol
        "#;

    let mut docs = YamlLoader::load_from_str(test_str).unwrap();
    let doc = &mut docs[0];
    process_yaml(doc, &config);
    assert_eq!(doc, &Yaml::Array(
        vec![
            Yaml::Hash(
                vec![(Yaml::String("name".to_string()), Yaml::String("Alice".to_string()))].into_iter().collect()
            ),
            Yaml::Hash(
                vec![(Yaml::String("name".to_string()), Yaml::String("Bob".to_string()))].into_iter().collect()
            ),
            Yaml::Hash(
                vec![(Yaml::String("name".to_string()), Yaml::String("Carol".to_string()))].into_iter().collect()
            ),
        ]
    ));
}

#[test]
fn full_test() {
    let config = init_test_config("config-gitops.yaml");

    let test_str = r#"
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

    let result = r#"---
enabled: false
apiVersion: argoproj.io/v1alpha1
name: test
namespace: argocd
spec:
  name: cluster-resources
  generators:
    - list:
        elements:
          - name: a
            ord:
              - name: a
              - name: b
              - name: c
          - name: b
          - name: c
anchor:
  - anchor
anchor-test:
  - anchor
arr:
  - enabled: false
    name: arr
  - apiVersion: argoproj.io/v1alpha1
    name: test
    namespace: argocd
    arr2:
      - enabled: false
        apiVersion: argoproj.io/v1alpha1
        name: test
        namespace: argocd
        test: yaml
    test: yaml
  - test: without sort key
test: yaml"#;

    let mut docs = YamlLoader::load_from_str(test_str).unwrap();
    let doc = &mut docs[0];
    process_yaml(doc, &config);

    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }

    assert_eq!(out_str, result);
}
