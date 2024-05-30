pub trait ErrorLogging {
    type ResultType;

    /// uuid, file, line
    fn elog(self, ctx: (&str, &str, u32)) -> Self::ResultType;
}

impl<T, E> ErrorLogging for Result<T, E>
where
    E: std::fmt::Debug,
{
    type ResultType = Result<T, E>;

    fn elog(self, ctx: (&str, &str, u32)) -> Self::ResultType {
        self.map_err(|err| {
            let (uid, file, line) = ctx;
            log::error!("[- uid -|- file -|- line -] [ <{uid}> | <{file}> | <{line}> ] - err [{err:?}]");
            err
        })
    }
}

#[macro_export]
macro_rules! ctx {
    () => {
        (
            &get(),   // uid
            file!(), // cur file
            line!(), // cur line
        )
    };
}

#[cfg(test)]
mod test {
    use std::env;
    use crate::error_logging::ErrorLogging;
    use crate::uid::get;

    #[test]
    fn test_ep() {

        env::set_var("RUST_LOG", "debug");

        env_logger::init();

        let r: Result<u64, &str> = Err("test ep print");
        r.elog(ctx!()).unwrap_or(0);
    }
}