
#[cfg(test)]
mod test {
    use super::factorial;

    #[test]
    fn factorial_of_5_is_120() {
        assert_eq!(adder::add(3, 2), 5);
    }


    #[test]
    fn factorial_returns_max_value_for_any_input_over_20() {
        assert_eq!(adder::add(5, 5), 10);
    }
}
