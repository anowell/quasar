use ::std::io;


quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            from()
            description("io error")
            display("I/O error: {}", err)
            cause(err)
        }
        // Other(descr: &'static str) {
        //     description(descr)
        //     display("Error {}", descr)
        // }
        // IoAt { place: &'static str, err: io::Error } {
        //     cause(err)
        //     display(me) -> ("{} {}: {}", me.description(), place, err)
        //     description("io error at")
        //     from(s: String) -> {
        //         place: "some string",
        //         err: io::Error::new(io::ErrorKind::Other, s)
        //     }
        // }
        // Discard {
        //     from(&'static str)
        // }
    }
}