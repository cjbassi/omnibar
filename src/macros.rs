#[macro_export]
macro_rules! check_null {
    ($v:ident) => {
        if $v.is_null() {
            eprintln!("[{}:{}]: null pointer received", file!(), line!());
            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! check_error {
    ($v:ident) => {
        if $v == -1 {
            eprintln!("[{}:{}]: error value received", file!(), line!());
            std::process::exit(1);
        }
    };
}
