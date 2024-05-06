
pub mod rooms {
    use std::collections::HashSet;

    /// Команата
    #[derive(Debug, Clone)]
    pub struct Room {
        pub name: String,
        pub devices: HashSet<String>,
    }

    impl Room {
        pub fn new(params: Room) -> Self {
            Self {
                name: params.name,
                devices: params.devices,
            }
        }
    }
}