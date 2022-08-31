use sdl2_sys as c;

pub fn poll_iter() -> Option<WEvent> {
    let mut event = std::mem::MaybeUninit::uninit();
    unsafe {
        if c::SDL_PollEvent(event.as_mut_ptr()) != 0 {
            return Some(WEvent { raw: event.assume_init() });
        }
    }

    return None;
}

pub const QUIT: u32 = c::SDL_EventType::SDL_QUIT as u32;
pub const MOUSE_BUTTON_UP: u32 = c::SDL_EventType::SDL_MOUSEBUTTONUP as u32;
pub const MOUSE_MOTION: u32 = c::SDL_EventType::SDL_MOUSEMOTION as u32;

pub mod ScanCode {
    use super::c;

    pub const W: u32 = c::SDL_Scancode::SDL_SCANCODE_W as u32;
    pub const S: u32 = c::SDL_Scancode::SDL_SCANCODE_S as u32;
    pub const A: u32 = c::SDL_Scancode::SDL_SCANCODE_A as u32;
    pub const D: u32 = c::SDL_Scancode::SDL_SCANCODE_D as u32;
}

pub struct WEvent {
    raw: c::SDL_Event,
}


impl WEvent {
    pub fn type_(&self) -> u32 {
        unsafe { self.raw.type_ }
    }

    pub fn get_mouse_pos(&self) -> (i32, i32) {
        unsafe {
            (self.raw.button.x, self.raw.button.y)
        }
    }
}

pub struct KeyboardState<'a> {
    keyboard_state: &'a [u8],
}

impl<'a> KeyboardState<'a>{
    pub fn is_scancode_pressed(&self, scancode: u32) -> bool{
        return self.keyboard_state[scancode as usize] != 0;
    }
}

pub fn get_keyboard_state<'a>() -> KeyboardState<'a> {

    let mut count = 0;
    unsafe {
        let ptr = c::SDL_GetKeyboardState(&mut count);
        return KeyboardState { keyboard_state: std::slice::from_raw_parts(ptr, count as usize) };
    }
}