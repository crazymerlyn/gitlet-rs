error_chain! {
    foreign_links {
        IO(::std::io::Error);
    }
}

//pub type Result<T> = ::std::result::Result<T, Error>;
