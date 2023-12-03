use crate::{
    data::Word,
    error::Result,
    image::Image,
    interpreter::{Interpreter, SCREEN_HEIGHT, SCREEN_WIDTH},
    platform::{Key, Platform, Point, Sprite},
};

use core::time::Duration;

////////////////////////////////////////////////////////////////////////////////

pub struct FrameBuffer([[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]);

impl Default for FrameBuffer {
    fn default() -> Self {
        Self([[false; SCREEN_WIDTH]; SCREEN_HEIGHT])
    }
}

impl FrameBuffer {
    pub fn iter_rows(&self) -> impl Iterator<Item = &[bool; SCREEN_WIDTH]> {
        self.0.iter()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait RandomNumberGenerator: FnMut() -> Word {}

impl<R: FnMut() -> Word> RandomNumberGenerator for R {}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct ManagedPlatform<R: RandomNumberGenerator> {
    rand: R,
    frame_buffer: FrameBuffer,
    delay_timer: Word,
    sound_timer: Word,
    // TODO: your code here.
}

impl<R: RandomNumberGenerator> Platform for ManagedPlatform<R> {
    fn draw_sprite(&mut self, pos: Point, sprite: Sprite) -> bool {
        // TODO: your code here.
        unimplemented!()
    }

    fn clear_screen(&mut self) {
        // TODO: your code here.
        unimplemented!()
    }

    fn get_delay_timer(&self) -> Word {
        // TODO: your code here.
        unimplemented!()
    }

    fn set_delay_timer(&mut self, value: Word) {
        // TODO: your code here.
        unimplemented!()
    }

    fn set_sound_timer(&mut self, value: Word) {
        // TODO: your code here.
        unimplemented!()
    }

    fn is_key_down(&self, key: Key) -> bool {
        // TODO: your code here.
        unimplemented!()
    }

    fn consume_key_press(&mut self) -> Option<Key> {
        // TODO: your code here.
        unimplemented!()
    }

    fn get_random_word(&mut self) -> Word {
        (self.rand)()
    }
}

impl<R: RandomNumberGenerator> ManagedPlatform<R> {
    fn new(rand: R) -> Self {
        Self {
            rand,
            frame_buffer: Default::default(),
            delay_timer: 0,
            sound_timer: 0,
            // TODO: your code here.
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct ManagedInterpreter<R: RandomNumberGenerator> {
    inner: Interpreter<ManagedPlatform<R>>,
    // TODO: your code here.
}

impl<R: RandomNumberGenerator> ManagedInterpreter<R> {
    pub const DEFAULT_OPERATION_DURATION: Duration = Duration::from_millis(2);
    pub const DEFAULT_DELAY_TICK_DURATION: Duration = Duration::from_nanos(16666667);
    pub const DEFAULT_SOUND_TICK_DURATION: Duration = Duration::from_nanos(16666667);

    pub fn new(image: impl Image, rand: R) -> Self {
        Self::new_with_durations(
            image,
            rand,
            Self::DEFAULT_OPERATION_DURATION,
            Self::DEFAULT_DELAY_TICK_DURATION,
            Self::DEFAULT_SOUND_TICK_DURATION,
        )
    }

    pub fn new_with_durations(
        image: impl Image,
        rand: R,
        operation_duration: Duration,
        delay_tick_duration: Duration,
        sound_tick_duration: Duration,
    ) -> Self {
        Self {
            inner: Interpreter::new(image, ManagedPlatform::new(rand)),
            // TODO: your code here.
        }
    }

    pub fn simulate_one_instruction(&mut self) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn simulate_duration(&mut self, duration: Duration) -> Result<()> {
        // TODO: your code here.
        unimplemented!()
    }

    pub fn frame_buffer(&self) -> &FrameBuffer {
        &self.inner.platform().frame_buffer
    }

    pub fn set_key_down(&mut self, key: Key, is_down: bool) {
        // TODO: your code here.
        unimplemented!()
    }
}
