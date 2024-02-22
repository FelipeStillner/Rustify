use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Music {
    pub id: i64,
    pub name: String,
    pub filename: String,
    pub duration: i64,
}

impl Music {
    pub fn new(id: i64, name: &str, filename: &str, duration: i64) -> Self {
        
        Music {
            id: id,
            name: String::from(name),
            filename: filename.to_string(),
            duration: duration,
        }
    }
    pub fn play(&self) {
        let path = String::from("musics/") + self.filename.as_str();
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();
        let file = std::fs::File::open(path.to_string()).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink.sleep_until_end();
    }
}
