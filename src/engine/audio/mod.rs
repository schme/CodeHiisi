extern crate soloud;

use std::{
    fs, io,
    collections::HashMap,
    path::Path,
    fmt::Display,
};

use self::soloud::*;

pub use self::{
    systems::{AudioSystem},
};


pub type AudioCueId = u32;

#[derive(Default)]
pub struct AudioQueue {
    pub audio: Vec<AudioCueId>,
}

pub struct AudioCue {
    id: AudioCueId,
    wav_data: Wav,
}

pub struct AudioStorage {
    pub string_map: HashMap<String, AudioCueId>,
    pub storage_id: String,
    pub cue_data: Vec<AudioCue>,
}

impl AudioStorage {
    pub fn new<P: AsRef<Path>>(path: &P) -> Self {
        let mut storage = AudioStorage {
            string_map: HashMap::new(),
            storage_id: path.as_ref().to_str().unwrap().to_string(),
            cue_data: Vec::new(),
        };

        if let Err(msg) = storage.load_storage_from_path(&path) {
            println!("Could not load storage from path: {}", msg);
        }
        storage
    }

    pub fn load_storage_from_path<P: AsRef<Path>>(&mut self, path: &P) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let path = entry?.path();
            if path.is_dir() {
                if let Err(msg) = self.load_storage_from_path(&path) {
                    println!("Failed to load audio from {}: {}", path.to_str().unwrap(), msg);
                }
            }
            else {
                self.new_cue(&path);
            }
        }
        Ok(())
    }

    pub fn new_cue<P: AsRef<Path>>(&mut self, filename: &P) {
        let id = self.cue_data.len() as u32;
        let mut wav_data = Wav::default();
        wav_data.load(filename).expect("Unable to load audio file");

        self.cue_data.push( AudioCue {
            id, wav_data,
        });


        self.string_map.insert(
            filename.as_ref().file_name().unwrap().to_str().unwrap().to_string(),
            id);
        println!("Added: {}", filename.as_ref().to_str().unwrap());
    }

    pub fn get_cue(&self, id: AudioCueId) -> Option<&AudioCue> {
        self.cue_data.get(id as usize)
    }

    pub fn get_cue_mut(&mut self, id: AudioCueId) -> Option<&mut AudioCue> {
        self.cue_data.get_mut(id as usize)
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<&AudioCueId> {
        self.string_map.get(name)
    }
}

pub struct AudioEngine {
    pub core: soloud::Soloud,
    storage: AudioStorage,
}

impl AudioEngine {
    pub fn new<P: AsRef<Path>>(audio_path: P) -> Self {
        let storage = AudioStorage::new(&audio_path);
        let core = Soloud::default().unwrap();

        AudioEngine {
            core,
            storage,
        }
    }

    pub fn get_storage(&self) -> &AudioStorage {
        &self.storage
    }

    pub fn get_storage_mut(&mut self) -> &mut AudioStorage {
        &mut self.storage
    }

    // Audio API

    pub fn play_cue_by_id(&self, cue_id: AudioCueId) {
        let cue = self.storage.get_cue(cue_id).unwrap();
        self.core.play(&cue.wav_data);
    }
}

pub mod systems {

    use std::{
        path::Path,
    };

    use super::{AudioEngine, AudioStorage, AudioExt, components::*,};

    use engine::ecs::{System, SystemData};


    pub struct AudioSystem {
        engine: Box<AudioEngine>,
        music_playing: bool,
    }

    impl AudioSystem {
        pub fn new<P: AsRef<Path>>(audio_path: P) -> Self {
            AudioSystem {
                engine: Box::new(AudioEngine::new(audio_path)),
                music_playing: true, //MUTE MUSIC
            }
        }

        pub fn get_storage(&self) -> &AudioStorage {
            self.engine.get_storage()
        }
    }

    impl<'a> System<'a> for AudioSystem {

        type SystemData = ();

        fn run(&mut self, _: Self::SystemData) {
            if !self.music_playing {
                let storage = self.engine.get_storage();
                if let Some(id) = storage.get_id_by_name("BallGame.wav") {
                    self.engine.play_cue_by_id(*id);
                    self.music_playing = true;
                }
                else {
                    println!("Couldn't find audio/BallGame.wav!");
                }
            }
        }
    }

}

pub mod components {

    use ecs::{
        Component,
        storage::HashMapStorage,
    };

    use super::AudioCueId;

    struct Music(AudioCueId);
    impl Component for Music {
        type Storage = HashMapStorage<Self>;
    }
}
