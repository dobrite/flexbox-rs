use sdl2::EventPump;

pub struct Events {
    pump: EventPump,

    pub quit: bool,
    pub key_escape: bool,
    pub key_j: Option<bool>,
    pub key_k: Option<bool>,
    pub key_h: Option<bool>,
    pub key_l: Option<bool>,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
            key_j: None,
            key_k: None,
            key_h: None,
            key_l: None,
        }
    }

    /// Update the events record.
    pub fn pump(&mut self) {
        // If the SDL context is dropped, then poll_iter() will simply stop
        // yielding any input.
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } => self.quit = true,

                KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Escape) => self.key_escape = true,
                        Some(J) => self.key_j = Some(true),
                        Some(K) => self.key_k = Some(true),
                        Some(H) => self.key_h = Some(true),
                        Some(L) => self.key_l = Some(true),
                        _ => {}
                    }
                }

                KeyUp { keycode, .. } => {
                    match keycode {
                        Some(Escape) => self.key_escape = false,
                        Some(J) => self.key_j = Some(false),
                        Some(K) => self.key_k = Some(false),
                        Some(H) => self.key_h = Some(false),
                        Some(L) => self.key_l = Some(false),
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
}
