use yaml_sorter_rust::config::{Config, set_config};
use yaml_sorter_rust::processors::{process_yaml};
use yaml_rust2::{YamlLoader, Yaml, YamlEmitter};

pub fn init_test_config(mock_config: &str) -> Config {
    let config_yaml = YamlLoader::load_from_str(mock_config).unwrap();
    let config_doc = &config_yaml[0];
    set_config(
        config_doc["sortKey"].as_str().unwrap().to_string(),
        config_doc["preOrder"].as_vec().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect()
    )
}

#[test]
fn test_load_config() {
    let config = init_test_config(
        "
        preOrder:
            - b
            - a
        sortKey: test_key
        "
    );

    assert!(!config.pre_order.is_empty());
    assert_eq!(config.sort_key, "test_key")
}


#[test]
fn test_hash_sorter() {
    let config = init_test_config(
        "
        preOrder:
            - b
            - a
        sortKey: test_key
        "
    );


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
            (Yaml::String("b".to_string()), Yaml::Integer(2)),
            (Yaml::String("a".to_string()), Yaml::Integer(1)),
            (Yaml::String("c".to_string()), Yaml::Integer(3)),
        ].into_iter().collect()
    ));
}

#[test]
fn test_array_sorter() {
    let config = init_test_config(
        "
        preOrder:
            - b
            - a
        sortKey: name
        "
    );
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
    let config = init_test_config(
        "
        sortKey: name
        preOrder:
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
        "
    );

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
