mod error;

pub use error::{Error, Result};

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub user_id: i64
}

impl RequestContext {

    pub fn _new_for_testing() -> Self {
        Self {user_id: 0}
    }

    pub fn new(user_id: i64) -> Result<Self> {
        if user_id == 0 {
            return Err(Error::UserIdZeroIsReservedForTesting);   
        }

        Ok(Self {user_id})
    }

    // Property accessors

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}
 