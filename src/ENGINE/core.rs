use std::time;
use sdl2_sys as c;
use crate::{print_error, WINDOW_HEIGHT, WINDOW_WIDHT};


// ================ enums


#[derive(Debug)]
pub enum SDLErrs {
    DefaultErr,
    InitializationErr,
    LoadTextureErr,
    TTFQueryTextureErr,
    TTFFontErr,
    DisplayModeErr,
    WindowFullScreenErr,
    LoadSurfaceErr,
    CreateRGBSurfaceErr,
    BlitSurfaceErr,
    CreateTextureFromSurfaceErr,
    FillRectFaild,
    RenderErr,
}

#[derive(Clone, Copy)]
pub enum RendererFlip {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
}


pub enum FullscreenType {
    Off = 0,
    True = 0x00_00_00_01,
    Desktop = 0x00_00_10_01,
}

// ================ structs


pub struct Renderer {
    ekran: *mut c::SDL_Renderer,
    window: *mut c::SDL_Window,
}

pub struct FpsCapDeltaTime {
    frame_delay: u64,
    pub set_fps: f32,
    pub dt: f32,
    cap_frame_start: time::Instant,
    last_time: time::Instant,
}

pub struct Texture {
    raw: *mut c::SDL_Texture,
}

#[derive(Clone, Copy)]
pub struct Rect {
    raw: c::SDL_Rect,
}

#[derive(Clone, Copy)]
pub struct Point2D {
    raw: c::SDL_Point,
}

#[derive(Debug, Clone)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// ================ impls

impl RendererFlip{
    pub fn raw(self) -> c::SDL_RendererFlip{
        match self {
            RendererFlip::None => c::SDL_RendererFlip::SDL_FLIP_NONE,
            RendererFlip::Horizontal => c::SDL_RendererFlip::SDL_FLIP_HORIZONTAL,
            RendererFlip::Vertical => c::SDL_RendererFlip::SDL_FLIP_VERTICAL,
        }
    }
}

impl Renderer {
    pub fn new(title: &str) -> Result<Self, SDLErrs> {
        let ekran;
        let window;
        unsafe {
            if c::SDL_Init(c::SDL_INIT_VIDEO) < 0 {
                print_error!();
                return Err(SDLErrs::InitializationErr);
            }
            window = c::SDL_CreateWindow(
                // cstr.as_ptr(),
                title.as_ptr() as *const _,
                c::SDL_WINDOWPOS_CENTERED_MASK as i32,
                c::SDL_WINDOWPOS_CENTERED_MASK as i32,
                WINDOW_WIDHT,
                WINDOW_HEIGHT,
                0
            );
            if window.is_null() {
                print_error!();
                return Err(SDLErrs::InitializationErr);
            }

            ekran = c::SDL_CreateRenderer(window, -1, c::SDL_RendererFlags::SDL_RENDERER_ACCELERATED as u32);
            if ekran.is_null() {
                print_error!();
                return Err(SDLErrs::InitializationErr);
            }

            if c::SDL_RenderSetLogicalSize(ekran, WINDOW_WIDHT, WINDOW_HEIGHT) < 0 {
                print_error!();
                return Err(SDLErrs::InitializationErr);
            }

            let flags = c::image::IMG_InitFlags_IMG_INIT_PNG as i32;

            if c::image::IMG_Init(c::image::IMG_InitFlags_IMG_INIT_PNG as i32) & flags != flags {
                print_error!();
                return Err(SDLErrs::InitializationErr);
            }

            if c::ttf::TTF_Init() < 0 {
                print_error!();
                return Err(SDLErrs::InitializationErr);
            }
        }

        return Ok(Self {
            ekran,
            window,
        });
    }
    /*
    pub fn set_window_fullscreen(&self, fullscreen_type: FullscreenType) -> Result<(), SDLErrs> {
        unsafe {
            if c::SDL_SetWindowFullscreen(self.window, fullscreen_type as u32) < 0 {
                print_error!();
                return Err(SDLErrs::WindowFullScreenErr);
            }
        }
        return Ok(());
    }

    pub fn set_window_display_mode(&self, display_mode: &DisplayMode) -> Result<(), SDLErrs> {
        unsafe {
            if c::SDL_SetWindowDisplayMode(self.window, &display_mode.raw) < 0 {
                print_error!();
                return Err(SDLErrs::DisplayModeErr);
            }
        }
        return Ok(());
    }

    pub fn set_window_size(&self, width: i32, height: i32) {
        unsafe { c::SDL_SetWindowSize(self.window, width, height); }
    }

    pub fn set_window_bordered(&self, bordered: bool) {
        unsafe { c::SDL_SetWindowBordered(self.window, if bordered { c::SDL_bool::SDL_TRUE } else { c::SDL_bool::SDL_FALSE }); }
    }
    */

    pub fn load_texture(&mut self, filename: &str) -> Result<Texture, SDLErrs> {
        let tex = unsafe { c::image::IMG_LoadTexture(self.ekran, filename.as_ptr() as *const _) };

        if tex.is_null() {
            print_error!();
            return Err(SDLErrs::LoadTextureErr);
        }


        return Ok(Texture { raw: tex });
    }

    pub fn renderer_copy_ref<'c, R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), SDLErrs>
        where
            R1: Into<Option<&'c Rect>>,
            R2: Into<Option<&'c Rect>>,
    {
        let ret = unsafe {
            c::SDL_RenderCopy(
                self.ekran,
                texture.raw,
                match src.into() {
                    Some(rect) => &rect.raw,
                    None => std::ptr::null(),
                },
                match dst.into() {
                    Some(rect) => &rect.raw,
                    None => std::ptr::null(),
                },
            )
        };

        if ret < 0 {
            print_error!();
            return Err(SDLErrs::RenderErr);
        } else {
            Ok(())
        }
    }
    pub fn renderer_copy_ref_ex<'c, R1, R2, C>(&mut self, texture: &Texture, src: R1, dst: R2, angle: f64, center: C, flip: RendererFlip) -> Result<(), SDLErrs>
        where
            R1: Into<Option<&'c Rect>>,
            R2: Into<Option<&'c Rect>>,
            C: Into<Option<&'c Point2D>>,
    {
        let ret = unsafe {
            c::SDL_RenderCopyEx(
                self.ekran,
                texture.raw,
                match src.into() {
                    Some(rect) => &rect.raw,
                    None => std::ptr::null(),
                },
                match dst.into() {
                    Some(rect) => &rect.raw,
                    None => std::ptr::null(),
                },
                angle,
                match center.into() {
                    Some(point) => &point.raw,
                    None => std::ptr::null(),
                },
                flip.raw() // copy
            )
        };

        if ret < 0 {
            print_error!();
            return Err(SDLErrs::RenderErr);
        } else {
            Ok(())
        }
    }

    pub fn renderer_copy<R1, R2>(&mut self, texture: &Texture, src: R1, dst: R2) -> Result<(), SDLErrs>
        where
            R1: Into<Option<Rect>>,
            R2: Into<Option<Rect>>,
    {
        let ret = unsafe {
            c::SDL_RenderCopy(
                self.ekran,
                texture.raw,
                match src.into() {
                    Some(ref rect) => &rect.raw,
                    None => std::ptr::null(),
                },
                match dst.into() {
                    Some(ref rect) => &rect.raw,
                    None => std::ptr::null(),
                },
            )
        };

        if ret < 0 {
            print_error!();
            return Err(SDLErrs::RenderErr);
        } else {
            Ok(())
        }
    }
    /*
        pub fn create_texture_from_surface(&self, surf: &Surface) -> Result<Texture, SDLErrs> {
            let tex = unsafe { c::SDL_CreateTextureFromSurface(self.ekran, surf.raw) };
            if tex.is_null() {
                print_error!();
                return Err(SDLErrs::CreateTextureFromSurfaceErr);
            }

            return Ok(Texture {
                raw: tex,
            })
        }

        pub fn warp_mouse_in_window(&self, x: i32, y: i32) {
            unsafe { c::SDL_WarpMouseInWindow(self.window, x, y); }
        }
    */
    pub fn set_draw_color(&self, color: impl Into<Color>) {
        let c = color.into();
        unsafe {
            if c::SDL_SetRenderDrawColor(self.ekran, c.r, c.g, c.b, c.a) < 0 {
                panic!();
            }
        }
    }

    pub fn draw_rect(&self, rect: &Rect) -> Result<(), SDLErrs> {
        unsafe {
            if c::SDL_RenderDrawRect(self.ekran, &rect.raw) < 0 {
                print_error!();
                return Err(SDLErrs::FillRectFaild);
            }
        }
        return Ok(());
    }

    pub fn clear(&self) {
        unsafe {
            if c::SDL_RenderClear(self.ekran) < 0 {
                panic!();
            }
        }
    }

    pub fn present(&mut self) {
        unsafe { c::SDL_RenderPresent(self.ekran); }
    }
}

impl FpsCapDeltaTime {
    pub fn new(fps: u64) -> Self {
        Self {
            frame_delay: (1000 / fps),
            set_fps: fps as f32,
            dt: 0.0,
            last_time: time::Instant::now(),
            cap_frame_start: time::Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.cap_frame_start = time::Instant::now();
        self.dt = self.last_time.elapsed().as_secs_f32();
        // println!("FPS: {} | set fps {} | dt {} ", 1.0 / self.dt, self.set_fps, self.dt);
        // println!("{}", (self.dt * self.set_fps));
        self.last_time = time::Instant::now();
    }

    pub fn end(&mut self) {
        let cap_frame_end = self.cap_frame_start.elapsed().as_millis() as u64;
        if cap_frame_end < self.frame_delay {
            std::thread::sleep(time::Duration::from_millis(self.frame_delay - cap_frame_end));
        }
    }
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { raw: c::SDL_Rect { x, y, w, h } }
    }

    pub fn has_intersection(&self, other: &Rect) -> bool {
        unsafe { c::SDL_HasIntersection(&self.raw, &other.raw) != c::SDL_bool::SDL_FALSE }
    }

    pub const fn contains_point(&self, point: Point2D) -> bool {
        let inside_x = point.x() >= self.left() && point.x() < self.right();
        return inside_x && (point.y() >= self.top() && point.y() < self.bottom());
    }

    pub fn set_x(&mut self, x:i32) {
        self.raw.x = x;
    }

    pub fn set_y(&mut self, y:i32) {
        self.raw.y = y;
    }

    pub const fn width( &self) -> i32 {
        self.raw.w
    }

    pub const fn height( &self) -> i32 {
        self.raw.h
    }

    pub const fn left(&self) -> i32 {
        self.raw.x
    }

    pub const fn right(&self) -> i32 {
        self.raw.x + self.raw.w
    }

    pub const fn top(&self) -> i32 {
        self.raw.y
    }

    pub const fn bottom(&self) -> i32 {
        self.raw.y + self.raw.h
    }
}

impl Point2D {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            raw: c::SDL_Point { x, y }
        }
    }

    pub const fn def() -> Self {
        Self {
            raw: c::SDL_Point { x: 0, y: 0 }
        }
    }

    pub fn set_from_typle(&mut self, tpl: (i32, i32)) {
        self.raw.x = tpl.0;
        self.raw.y = tpl.1;
    }

    pub fn set_x(&mut self, x: i32) {
        self.raw.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.raw.y = y;
    }

    pub const fn x(&self) -> i32 {
        self.raw.x
    }
    pub const fn y(&self) -> i32 {
        self.raw.y
    }
}

impl Vector2D {
    pub fn def() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Color {
    pub fn as_raw(&self) -> c::SDL_Color {
        c::SDL_Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
        }
    }
}


impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            c::SDL_DestroyWindow(self.window);
            c::SDL_DestroyRenderer(self.ekran);
            c::ttf::TTF_Quit();
            c::image::IMG_Quit();
            c::SDL_Quit();
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { c::SDL_DestroyTexture(self.raw); }
    }
}



impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color { r, g, b, a: 0xff }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color { r, g, b, a }
    }
}



impl std::fmt::Debug for Rect {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return write!(
            fmt,
            "Rect {{ x: {}, y: {}, w: {}, h: {} }}",
            self.raw.x, self.raw.y, self.raw.w, self.raw.h
        );
    }
}

impl std::fmt::Debug for Point2D {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        return write!(
            fmt,
            "Point2D {{ x: {}, y: {} }}",
            self.raw.x, self.raw.y
        );
    }
}


#[macro_export]
macro_rules! print_error {
    () =>  {
        #[allow(unused_unsafe)]
        unsafe { dbg!(std::ffi::CStr::from_ptr(c::SDL_GetError())) };
    }
}

#[macro_export]
macro_rules! not {
    ($x:expr) => {!$x};
}