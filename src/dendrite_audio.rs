pub mod audio {
    use nannou_audio as audio;
    use nannou_audio::Buffer;

    pub struct Audio {
        sounds: Vec<audrey::read::BufFileReader>,
    }

    pub type StreamAudio = audio::Stream<Audio>;

    pub fn audio_set() -> audio::Stream<Audio>{
        // Initialise the audio host so we can spawn an audio stream.
        let audio_host = audio::Host::new();

        let sounds = vec![];
        let model = Audio { sounds };
        let stream = audio_host
            .new_output_stream(model)
            .render(audio)
            .build()
            .unwrap();

        stream.play().unwrap();

        //import file
        //let path = "./assets/music/Dendrite_demo.wav";
        let path = "./assets/music/demo.wav";
        let sound = audrey::open(path).expect("failed to load sound");

        stream
            .send(move |audio| {
                audio.sounds.push(sound);
            })
            .ok();

        stream
    }

    // A function that renders the given `Audio` to the given `Buffer`.
    // In this case we play the audio file.
    fn audio(audio: &mut Audio, buffer: &mut Buffer) {
        let mut have_ended = vec![];
        let len_frames = buffer.len_frames();

        // Sum all of the sounds onto the buffer.
        for (i, sound) in audio.sounds.iter_mut().enumerate() {
            let mut frame_count = 0;
            let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);
            for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
                for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
                    *sample += *file_sample;
                }
                frame_count += 1;
            }

            // If the sound yielded less samples than are in the buffer, it must have ended.
            if frame_count < len_frames {
                have_ended.push(i);
            }
        }

        // Remove all sounds that have ended.
        for i in have_ended.into_iter().rev() {
            audio.sounds.remove(i);
        }
    }
}




