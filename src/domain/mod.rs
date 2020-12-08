pub mod delimiter;
pub mod lang_builtin;
pub mod lang_default;
pub mod type_alias;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Point {
        x: f64,
        y: f64,
    }

    #[test]
    fn should_convert_yaml() {
        let point = Point { x: 1.0, y: 2.0 };

        let s = serde_yaml::to_string(&point).unwrap();
        assert_eq!(s, "---\nx: 1.0\ny: 2.0");

        let deserialized_point: Point = serde_yaml::from_str(&s).unwrap();
        assert_eq!(point, deserialized_point);
    }
}
