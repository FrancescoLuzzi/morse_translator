use super::{get_w, oscillator, Amplitude, MAX_AMPLITUDE, SAMPLE_RATE};

pub struct Note(pub f32);

impl Note {
    pub fn get_frequency(&self) -> f32 {
        self.0
    }

    pub fn combine<'a>(
        notes: &'a [Self],
        secs: f32,
        volume: &'a Amplitude,
    ) -> impl Iterator<Item = i16> + 'a {
        let nsamples = secs * SAMPLE_RATE as f32;
        (0..nsamples as u32)
            .map(move |t| {
                notes.iter().map(move |note| match volume {
                    Amplitude::Silent => 0_f32,
                    _ => oscillator(
                        get_w(note.get_frequency(), t as f32, SAMPLE_RATE as f32),
                        1_f32,
                    ),
                })
            })
            .map(|step| {
                // TODO: refactoring implementing Mean trait for iter
                // https://stackoverflow.com/questions/43921436/extend-iterator-with-a-mean-method#answer-43926007
                // You can use as base rust impl for Sum and their proc macro
                // https://github.com/rust-lang/rust/blob/master/library/core/src/iter/traits/accum.rs
                let (len, sum) = step.fold((0, 0_f32), |(len, sum), x| (len + 1, sum + x));
                f32::floor(MAX_AMPLITUDE * volume.scaling() * sum / len as f32) as i16
            })
    }

    pub fn audio_wave(&self, secs: f32, volume: &Amplitude) -> Vec<i16> {
        let nsamples = secs * SAMPLE_RATE as f32;
        (0..nsamples as u32)
            .map(|t| match *volume {
                Amplitude::Silent => 0_i16,
                _ => f32::floor(oscillator(
                    get_w(self.get_frequency(), t as f32, SAMPLE_RATE as f32),
                    MAX_AMPLITUDE * volume.scaling(),
                )) as i16,
            })
            .collect()
    }
}
