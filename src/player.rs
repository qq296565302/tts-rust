use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub struct AudioPlayer {
    _stream: Option<OutputStream>,
    _handle: Option<OutputStreamHandle>,
    sink: Option<Sink>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            _stream: None,
            _handle: None,
            sink: None,
        }
    }

    pub fn play(&mut self, path: &PathBuf) -> Result<()> {
        self.stop();
        
        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;
        
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;
        
        sink.append(source);
        sink.set_volume(1.0);
        
        self._stream = Some(stream);
        self._handle = Some(handle);
        self.sink = Some(sink);
        
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        self.sink = None;
        self._handle = None;
        self._stream = None;
    }
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self::new()
    }
}
