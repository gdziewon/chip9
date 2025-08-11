use rodio::{source::SineWave, OutputStream, OutputStreamBuilder, Sink, Source as _};

const SINEWAVE_FREQUENCY: f32 = 440.0; // A4

pub struct Audio {
    _stream_handle: OutputStream,
    audio: Sink
}

impl Audio {
    pub(super) fn new() -> Self {
        let mut _stream_handle = OutputStreamBuilder::open_default_stream().unwrap(); // todo: handle
        _stream_handle.log_on_drop(false); // disabling: Dropping OutputStream, audio playing through this stream will stop
        let audio = Sink::connect_new(&_stream_handle.mixer());
        let source = SineWave::new(SINEWAVE_FREQUENCY).repeat_infinite();
        audio.append(source);
        audio.pause();
        Audio { _stream_handle, audio}
    }

    pub(super) fn pause(&self) {
        self.audio.pause();
    }

    pub(super) fn play(&self) {
        self.audio.play();
    }

    pub(super) fn is_playing(&self) -> bool {
        !self.audio.is_paused()
    }
}