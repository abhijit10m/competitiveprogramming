pub fn expected_minutes_in_oven() -> i32 {
    let expected_time: i32 = 40;
    return expected_time
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let remaining_time :i32 = expected_minutes_in_oven() - actual_minutes_in_oven;
    return remaining_time
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let prep_time :i32 = 2 * number_of_layers;
    return prep_time
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let elapsed_time :i32 = preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;
    return elapsed_time
}
