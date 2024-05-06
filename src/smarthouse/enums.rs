    ///Состояния нашей розетки
    #[derive(Clone)]
    #[derive(Debug)]
    #[derive(PartialEq)]
    #[non_exhaustive]
    pub enum SocketState {
        IsOn,  //Включено
        IsOff, //Выключено
    }