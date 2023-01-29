pub struct Addr<'a> {
    pub(crate) source: &'a str,
    pub host: &'a str,
    pub port: u32,
}

impl<'a> From<&'a str> for Addr<'a> {
    fn from(value: &'a str) -> Self {
        let (host, port) = value
            .split_once(':')
            .expect("Хост и порт должен быть разделён ':'!");

        Self {
            source: value,
            host,
            port: port.parse().expect("Порт должен представляться числом!"),
        }
    }
}
