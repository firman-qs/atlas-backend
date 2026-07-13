// pub fn validate_course_code(code: &str) -> garde::Result {
//     Ok(())
// }
//
// pub fn validate_course_title(title: &str) -> garde::Result {
//     Ok(())
// }

pub fn validate_course_description(value: &Option<String>, _: &()) -> garde::Result {
    println!("Validating course description: {:?}", value);
    Ok(())
}
