use knife_util::{AnyError, ERR_DB};

pub fn from_rbatis(err: rbatis::Error) -> AnyError {
    ERR_DB.cause(anyhow::Error::new(err))
}
