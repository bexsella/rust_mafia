///
/// 

use std::ffi::{c_void};
use std::ptr::{null_mut};

// Win32 Types
type HANDLE = *mut c_void;
type BOOL = i32;
type DWORD = u32;

#[repr(C)]
struct COORD {
    x: u16,
    y: u16,
}

#[repr(C)]
struct SMALL_RECT {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}

#[repr(C)]
struct CONSOLE_SCREEN_BUFFER_INFO {
    dwSize: COORD,
    dwCursorPosition: COORD,
    wAttributes: u16,
    srWindow: SMALL_RECT,
    dwMaximumWindowSize: COORD,
}

impl CONSOLE_SCREEN_BUFFER_INFO {
    pub fn init () -> CONSOLE_SCREEN_BUFFER_INFO {
        CONSOLE_SCREEN_BUFFER_INFO {
            dwSize: COORD{x: 0, y: 0}, 
            dwCursorPosition: COORD{x: 0, y: 0}, 
            wAttributes: 0,
            srWindow: SMALL_RECT{left: 0, top: 0, right: 0, bottom: 0},
            dwMaximumWindowSize: COORD{x: 0, y: 0}
        }
    }
}

#[repr(C)]
struct INPUT_RECORD {
    
}

// STD Handle Values:
const STD_INPUT_HANDLE: u32 = 0xffff_fff6;
const STD_OUTPUT_HANDLE: u32 = 0xffff_fff5;
const STD_ERROR_HANDLE: u32 = 0xffff_fff4;

// Foreground constants:
pub const FOREGROUND_BLUE: u16 =  0x0001;
pub const FOREGROUND_GREEN: u16 = 0x0002;
pub const FOREGROUND_RED: u16 = 0x0004;
pub const FOREGROUND_INTENSITY: u16 = 0x0008;

// Background constants:
pub const BACKGROUND_BLUE: u16 = 0x0010;
pub const BACKGROUND_GREEN: u16 = 0x0020;
pub const BACKGROUND_RED: u16 = 0x0040;
pub const BACKGROUND_INTENSITY: u16 = 0x0080;

// Common LVB constants:
pub const COMMON_LVB_UNDERSCORE: u16 = 0x8000;

// Virtual Terminal Constants:
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

#[link(name = "Kernel32")]
extern {
    // Win32 API
    fn GetLastError () -> DWORD;

    // Win32 Console API
    fn GetStdHandle (std_handle: u32) -> HANDLE;

    fn GetConsoleMode (console_handle: HANDLE, lp_mode: *mut u32) -> BOOL;
    fn SetConsoleMode (console_handle: HANDLE, mode: u32) -> BOOL;

    fn GetConsoleScreenBufferInfo (handle: HANDLE, console_screen_buffer_info: *mut CONSOLE_SCREEN_BUFFER_INFO);

    fn SetConsoleTextAttribute (handle: HANDLE, colour: u16) -> i32;
    fn SetConsoleCursorPosition (handle: HANDLE, position: COORD) -> i32;
}

pub struct Console {
    output_handle: HANDLE,
    input_handle: HANDLE,

    start_attributes: u16,
    original_mode: u32,
}

impl Console {
    pub fn init () -> Console {
        unsafe {
            let output_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let input_handle = GetStdHandle(STD_INPUT_HANDLE);

            let mut current_screen_buffer_info = CONSOLE_SCREEN_BUFFER_INFO::init();
            let screen_buffer_ptr = &mut current_screen_buffer_info as *mut CONSOLE_SCREEN_BUFFER_INFO;

            GetConsoleScreenBufferInfo(output_handle, screen_buffer_ptr);

            let mut original_mode: DWORD = 0;
            let mode_ptr = &mut original_mode as *mut DWORD;
            
            GetConsoleMode(output_handle, mode_ptr);

            let mode:DWORD = original_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            SetConsoleMode(output_handle, mode);

            Console {
                output_handle,
                input_handle,
                start_attributes: current_screen_buffer_info.wAttributes,
                original_mode
            }
        }
    }

    pub fn quit (&self) {
        unsafe {
            SetConsoleTextAttribute(self.output_handle, self.start_attributes);
            SetConsoleMode(self.output_handle, self.original_mode);
        }
    }

    pub fn set_text_position (&self, x: u16, y: u16) {
        unsafe {
            SetConsoleCursorPosition(self.output_handle, COORD{x, y});
        }
    }

    pub fn set_text_color (&self, colour: u16) {
        unsafe {
            SetConsoleTextAttribute(self.output_handle, colour);
        }
    }

    pub fn clear (&self) {
        unsafe {
                unimplemented!();
        }
    }

    pub fn in_key (&self) {
        unimplemented!();
    }
}