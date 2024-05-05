#[cfg(test)]
mod test {
    use crate::col_to_vec;
    use crate::data_manipulation::data_manipulation::VecType;
    use crate::epoch_to_date;
    use crate::strong_ratings_only;
    use crate::components_and_sizes;

    #[test]
    fn test_col_to_vec() {
        let data: Vec<(i32, i32, i32, f64)> = vec![(1, 2, 1, 1.0), (2, 3, 2, 2.0), (3, 4, 3, 3.0)];
        assert_eq!(VecType::IntVec(vec![2, 3, 4]), col_to_vec(&data, 1), "col_to_vec function not working properly.");
    }

    #[test]
    fn test_epoch_to_date() {
        assert_eq!((1, 1970), epoch_to_date(1.0), "epoch_to_date function not working properly.");
        assert_eq!((2, 1971), epoch_to_date(31_540_000.0 + 3_000_000.0), "epoch_to_date function not working properly.");
    }

    #[test]
    fn test_strong_ratings_only() {
        let data: Vec<(i32, i32, i32, f64)> = vec![(1, 2, 1, 1.0), (2, 3, 2, 2.0), (3, 4, 7, 3.0), (4, 5, 8, 4.0), (5, 6, 10, 5.0)];
        assert_eq!(vec![(4, 5, 8, 4.0), (5, 6, 10, 5.0)], strong_ratings_only(&data), "strong_ratings_only function not working properly.");
    }

    #[test]
    fn test_components_and_sizes() {
        let data: Vec<(i32, i32, i32, f64)> = vec![(1, 2, 1, 1.0), (2, 3, 2, 2.0), (3, 4, 7, 3.0), (5, 6, 10, 5.0), (6, 7, 10, 6.0), (8, 9, 5, 7.0)];
        assert_eq!((3, vec![4, 3, 2]), components_and_sizes(&data), "components_and_sizes function not working properly");
    }
}