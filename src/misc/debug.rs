#[macro_export]
macro_rules! debug_single {
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprint!(" {} = {:?}", stringify!($val), &tmp);
                tmp
            }
        }
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! de {
    () => {
        eprintln!("#{}", line!())
    };
    ($($val:expr),+ $(,)?) => {
        eprint!("#{}", line!());
        ($(debug_single!($val)),+,);
        eprintln!();
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! de {
    ($($val:expr),+ $(,)?) => { };
}
