#[macro_export]
macro_rules! log {
    () => {
        eprintln!(
            "{} {}:{}",
            chrono::Local::now().time().format("%H:%M:%S%.6f"),
            file!(),
            line!()
        )
    };
    ($($arg:tt)*) => {
        eprintln!(
            "{} {}:{}\t{}",
            chrono::Local::now().time().format("%H:%M:%S%.6f"),
            file!(),
            line!(),
            format_args!($($arg)*)
        )
    };
}
