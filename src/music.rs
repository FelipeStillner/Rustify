use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Music {
    pub id: i64,
    name: String,
    path: String,
    duration: u64,
}

impl Music {
    pub fn new(id: i64, name: &str, filename: &str, duration: u64) -> Self {
        let path = String::from("musics/") + filename;
        Music {id: id, name: String::from(name), path: path, duration: duration}
    }
    pub fn play(&self) {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let file = std::fs::File::open(self.path.to_string()).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink.sleep_until_end();
    }
}