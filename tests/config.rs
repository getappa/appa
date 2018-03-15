extern crate appa;

#[cfg(test)]
mod test {
    use appa::config::AppaConfig;

    #[test]
    fn should_map_correctly_vectors() {
        let set = AppaConfig::new("tests/mocks/config1.yml".to_string());

        assert_eq!(4, set.tasks.len());
        assert_eq!(2, set.entities.len());
    }
}