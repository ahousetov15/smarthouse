// use std::fmt::{self, Error};
use std::fmt::{self};



#[derive(Debug)]
pub struct NoRoomError {
    pub room_name: String,
}


impl fmt::Display for NoRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Комната '{}' не найдена", self.room_name)
    }
}



enum SmarthouseErrors {
    NoRoom(NoRoomError),
}


impl From<NoRoomError> for SmarthouseErrors {
    fn from(err: NoRoomError) -> Self {
        SmarthouseErrors::NoRoom(err)
    }
}