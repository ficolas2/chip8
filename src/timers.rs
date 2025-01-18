use std::time::{Duration, Instant};

use rodio::{OutputStream, Source};

pub struct Timers {
    pub delay: u8,
    pub sound: u8,
    _stream: rodio::OutputStream,
    sink: rodio::Sink,
    last_decrement: Instant,
    playing: bool,
}

impl Timers {
    pub fn new() -> Timers {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        Timers {
            delay: 0,
            sound: 0,
            last_decrement: Instant::now(),
            _stream: stream,
            sink,
            playing: false,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_decrement);
        if elapsed.as_secs() < 1 {
            return;
        }
        self.last_decrement -= Duration::from_millis(1000 / 60);
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }

        if self.sound > 0 && (!self.playing || self.sink.empty()) {
            let source = rodio::source::SineWave::new(440.0)
                .amplify(0.2)
                .take_duration(std::time::Duration::from_millis(1000 / 60 * self.sound as u64));
            self.sink.append(source);
            self.playing = true;
        }
        if self.sound == 0 && self.playing {
            self.playing = false;
            self.sink.stop();
        }
    }
}
