use yaml_sorter_rust::config::{Config, init_test_config};
use yaml_sorter_rust::processors::{process_yaml};
use yaml_rust2::{YamlLoader, Yaml};
use yaml_sorter_rust::config;

#[test]
fn test_load_config() {
    let config = init_test_config(
        "
        preOrder:
            - b
            - a
        sortKey: test_key
        "
    );();

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

    let mut docs = YamlLoader::load_from_str(&test_str).unwrap();
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

    let mut docs = YamlLoader::load_from_str(&test_str).unwrap();
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

// #[test]
// fn test_array_sorter() {
//     let config_str = "
//     sortKey: name
//     ";
//     let docs = YamlLoader::load_from_str(config_str).unwrap();
//     CONFIG.set(docs[0].clone()).expect("Unable to set config");
//
//     let mut array = vec![
//         yaml_rust2::yaml::Hash::new(),
//         yaml_rust2::yaml::Hash::new(),
//         yaml_rust2::yaml::Hash::new(),
//     ];
//
//     array[0].insert(Yaml::String("name".to_string()), Yaml::String("Alice".to_string()));
//     array[1].insert(Yaml::String("name".to_string()), Yaml::String("Bob".to_string()));
//     array[2].insert(Yaml::String("name".to_string()), Yaml::String("Carol".to_string()));
//
//     array_sorter(&mut array);
//
//     let names: Vec<_> = array.iter().map(|h| h["name"].as_str().unwrap()).collect();
//     assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
// }
