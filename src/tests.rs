#[cfg(test)]
mod test {
    use crate::col_to_vec;
    use crate::data_manipulation::data_manipulation::VecType;
    use crate ::epoch_to_date;

    #[test]
    fn test_col_to_vec() {
        let data: Vec<(i32, i32, i32, f64)> = vec![(1, 2, 1, 1.0), (2, 3, 2, 2.0), (3, 4, 3, 3.0)];
        assert_eq!(VecType::IntVec(vec![2, 3, 4]), col_to_vec(&data, 1), "col_to_vec function not working properly");
    }

    #[test]
    fn test_epoch_to_date() {
        assert_eq!((1, 1970), epoch_to_date(1.0), "epoch_to_date function not working properly");
        assert_eq!((2, 1971), epoch_to_date(31_540_000.0 + 3_000_000.0), "epoch_to_date function not working properly");
    }
}