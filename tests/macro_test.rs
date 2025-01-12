use crate::api::models::*;


// src/macros_test.rs (ou dans `#[cfg(test)]` dans le même fichier)
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct DFDRange {
        min: i32,
        max: i32,
    }

    #[derive(Debug, PartialEq)]
    struct NewRange {
        min: i32,
        max: i32,
    }

    impl_from!(DFDRange, NewRange, {
        min => min,
        max => max,
    });

    #[test]
    fn test_range_conversion() {
        let dfd_range = DFDRange { min: 10, max: 20 };
        let new_range: NewRange = (&dfd_range).into();
        assert_eq!(new_range, NewRange { min: 10, max: 20 });
    }
}
