use mergeable_compatibility_layer::configuration::Configuration;
use serde_yaml::Value as YamlObject;

#[test]
fn verify_all_configurable_elements_can_be_correctly_parsed() {
    const TEST: &str = include_str!("all-configurable-elements-test.yaml");

    let config: Configuration = serde_yaml::from_str(TEST).unwrap();

    let raw: YamlObject = serde_yaml::from_str(TEST).unwrap();
    let p_config = serde_yaml::to_value(config).unwrap();

    assert_eq!(raw, p_config);

    let config: Configuration = serde_yaml::from_str(TEST).unwrap();
    println!("{config:#?}")
}
