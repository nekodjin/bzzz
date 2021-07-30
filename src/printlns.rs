macro_rules! printlns {
    ($($line:expr)*) => {
        $(println!($line);)*
    };
} pub(crate) use printlns;
