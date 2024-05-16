use std::fmt;

struct NoDeviceError {
    room_name: String,
    device_name: String,
}

struct NoRoomError {
    room_name: String,
}

impl fmt::Display for NoDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        error!(f, "{} в комнате {} не найдено", self.device_name, self.room_name);
    }
}