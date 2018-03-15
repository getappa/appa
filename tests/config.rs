extern crate appa;

#[cfg(test)]
mod test {
    use appa::config::AppaConfig;

    #[test]
    fn should_map_correctly_vectors() {
        let set = AppaConfig::new("tests/mocks/config1.yml".to_string());
        assert_eq!("user".to_string(), set.entities[0].name);
        // assert_eq!("2".to_string(), set.tasks);
    }
}