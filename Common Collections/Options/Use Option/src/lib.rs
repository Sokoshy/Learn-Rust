pub fn maybe_ice_cream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a value of 0
    // The Option output should gracefully handle cases where time_of_day > 23.
    // TODO: Return an Option!
    if time_of_day < 22 {
        Some(5)
    }else if time_of_day > 23 {
        None
    }else {
        Some(0)
    }
}

#[test]
fn raw_value() {

    assert_eq!(maybe_ice_cream(20).unwrap(), 5);}