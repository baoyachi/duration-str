#[cfg(test)]
#[macro_export]
macro_rules! catch_err {
    ($result:expr) => {
        format!("{}", $result.err().unwrap())
    };
}
