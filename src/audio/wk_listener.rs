
use kira::{
	AudioManager, AudioManagerSettings, DefaultBackend, ResourceLimitReached,
    effect::filter::FilterBuilder, sound::static_sound::StaticSoundData,
    track::{
		TrackBuilder, TrackHandle
	},
};

pub struct ListenerKiraSound;
impl ListenerKiraSound
{
    fn sound_manager() -> Result<AudioManager, kira::backend::cpal::Error>
    {
        let manager = AudioManager::<DefaultBackend>::new(
            AudioManagerSettings::default()
        )?;

        Ok(manager)
    }

    //---------------------------------------------------------------
    /// Playing a sound with a low-pass filter applied (this makes the audio sound muffled)

    /// Cutoff = 1000.0 (Default)
    fn sound_track(cutoff: f64) -> Result<TrackHandle, ResourceLimitReached>
    {
        let track = Self::sound_manager().unwrap().add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(FilterBuilder::new().cutoff(cutoff));
            builder
        })?;

        Ok(track)
    }

    pub fn play_sound_with_effect_muffled(path_from_file: impl AsRef<std::path::Path>, cutoff: f64) -> Result<(), kira::sound::FromFileError>
    {
        //-------------------------------------------
        let mut manager = Self::sound_manager().unwrap();
        Self::sound_track(cutoff).unwrap();
        //-------------------------------------------

        let sound_data = StaticSoundData::from_file(path_from_file)?;
        manager.play(sound_data).expect("[FAIL] Manager.Play in play_sound_with_effect_muffled has been failure");

        Ok(())
    }

    //---------------------------------------------------------------
}