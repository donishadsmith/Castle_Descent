#[macro_export]
macro_rules! debug_print {
    ($(($statement:expr, $object:expr)),*) => {
        $(println!("{}: {:?}", $statement, $object);)*
    }
}
