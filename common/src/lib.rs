#[derive(Debug)]
pub struct InputError {
    pub message: &'static str
}

impl From<std::num::ParseIntError> for InputError {
    fn from(_: std::num::ParseIntError) -> Self {
        InputError{  message: "ParseIntError" }
    }
}

impl From<std::io::Error> for InputError {
    fn from(_: std::io::Error) -> Self {
        InputError{  message: "io error"}
    }
}

impl From<&std::io::Error> for InputError {
    fn from(_: &std::io::Error) -> Self {
        InputError{ message: "io error" }
    }
}