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
            log::error!("[ <{uid}> | <{file}> | <{line}> ] - [{err:?}]");
            err
        })
    }
}

#[macro_export]
macro_rules! ctx {
    () => {
        (
            &sync_get(), // uid
            file!(),     // cur file
            line!(),     // cur line
        )
    };
}

#[macro_export]
macro_rules! actx {
    () => {
        (
            &async_get(), // uid
            file!(),      // cur file
            line!(),      // cur line
        )
    };
}

#[cfg(test)]
mod test {
    use crate::error_logging::ErrorLogging;
    use crate::sync_uid::sync_get;
    use std::env;

    #[test]
    fn test_ep() {
        env::set_var("RUST_LOG", "debug");

        env_logger::init();

        let r: Result<u64, &str> = Err("test ep print");
        r.elog(ctx!()).unwrap_or(0);
    }
}
