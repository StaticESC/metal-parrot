pub mod ffmpeg;
pub mod youtube;

#[derive(Clone, Copy)]
pub struct RestartableOptions {
    pub lazy: bool,
    pub sponsorblock: bool,
}
