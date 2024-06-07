pub trait ErrorLogging {
    type ResultType;

    /// uuid, file, line
    fn elog(self, ctx: (Option<&str>, &str, u32)) -> Self::ResultType;
}

impl<T, E> ErrorLogging for Result<T, E>
where
    E: std::fmt::Debug,
{
    type ResultType = Result<T, E>;

    fn elog(self, ctx: (Option<&str>, &str, u32)) -> Self::ResultType {
        self.map_err(|err| {

            let (uid, file, line) = ctx;

            if let Some(uid) = uid {
                log::error!("[ <{uid}> | <{file}> | <{line}> ] - [{err:?}]");
            } else {
                log::error!("[ <{file}> | <{line}> ] - [{err:?}]");
            }

            err
        })
    }
}

#[macro_export]
macro_rules! ctx {
    ($uid:expr) => {
        (
            Some($uid),     // uid
            file!(),        // cur file
            line!(),        // cur line
        )
    };
    () => {
        (
            None,    // no uid
            file!(), // cur file
            line!(), // cur line
        )
    };
}

