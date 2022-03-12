///
/// 

use std::ffi::{c_void};
use std::ptr::{null_mut};

// Win32 Types
type HANDLE = *mut c_void;
type BOOL = i32;
type DWORD = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct COORD {
    x: u16,
    y: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct SMALL_RECT {
    left: i16,
    top: i16,
    right: i16,
    bottom: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CONSOLE_SCREEN_BUFFER_INFO {
    size: COORD,
    cursor_position: COORD,
    attributes: u16,
    window: SMALL_RECT,
    maximum_window_size: COORD,
}

impl CONSOLE_SCREEN_BUFFER_INFO {
    pub fn init () -> CONSOLE_SCREEN_BUFFER_INFO {
        CONSOLE_SCREEN_BUFFER_INFO {
            size: COORD{x: 0, y: 0}, 
            cursor_position: COORD{x: 0, y: 0}, 
            attributes: 0,
            window: SMALL_RECT{left: 0, top: 0, right: 0, bottom: 0},
            maximum_window_size: COORD{x: 0, y: 0}
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union KEY_INPUT_CHAR {
    pub unicode_char: u16,
    pub ascii_char: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct KEY_EVENT_RECORD {
    pub key_down: i32,
    pub repeat_count: u16,
    pub virtual_key_code: u16,
    pub virtual_scan_code: u16,
    pub uchar: KEY_INPUT_CHAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MOUSE_EVENT_RECORD {
    pub mouse_position: COORD,
    pub button_state: u32,
    pub control_key_state: u32,
    pub event_flags: u32
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    pub size: COORD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MENU_EVENT_RECORD {
    pub command_id: u32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FOCUS_EVENT_RECORD {
    pub set_focus: BOOL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union INPUT_RECORD_EVENT {
    pub key_event: KEY_EVENT_RECORD,
    pub mouse_event: MOUSE_EVENT_RECORD,
    pub window_buffer_size_event: WINDOW_BUFFER_SIZE_RECORD,
    pub menu_event: MENU_EVENT_RECORD,
    pub focus_event: FOCUS_EVENT_RECORD,
}

pub const KEY_EVENT: u16 = 0x0001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INPUT_RECORD {
    pub event_type: u16,
    pub event: INPUT_RECORD_EVENT,
}

// STD Handle Values:
const STD_INPUT_HANDLE: u32 = 0xffff_fff6;
const STD_OUTPUT_HANDLE: u32 = 0xffff_fff5;

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

    fn ReadConsoleInputW (input_handle: HANDLE, buffer: *mut INPUT_RECORD, length: u32, number_of_events_read: *mut u32) -> BOOL;
}

pub struct Console {
    output_handle: HANDLE,
    input_handle: HANDLE,
    original_mode: u32,
}

pub enum Keys {

}

pub struct KeyInfo {
    key: u32,
    ch: char,
    modifiers: u32,
}

impl Console {
    pub fn init () -> Console {
        unsafe {
            // grab everything but hte error handle:
            let output_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let input_handle = GetStdHandle(STD_INPUT_HANDLE);

            let mut original_mode: DWORD = 0;
            
            if GetConsoleMode(output_handle, &mut original_mode as *mut DWORD) == 0 {
                panic!("Failed to retrieve console mode: {}", GetLastError());
            }

            let mode:DWORD = original_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;

            if SetConsoleMode(output_handle, mode) == 0 {
                panic!("Failed to set console mode: {}", GetLastError());
            }

            Console {
                output_handle,
                input_handle,
                original_mode
            }
        }
    }

    pub fn quit (&self) {
        unsafe {
            SetConsoleMode(self.output_handle, self.original_mode);
        }
    }

    pub fn set_text_position (&self, x: u16, y: u16) {
        unsafe {
            // SetConsoleCursorPosition(self.output_handle, COORD{x, y});
        }
    }

    pub fn set_text_color (&self, colour: u16) {
        unsafe {
            unimplemented!();
        }
    }

    pub fn clear (&self) {
        print!("\x1b[2J");
    }

    pub fn read_key (&self) -> KeyInfo {
        unsafe {
            /*/
            let mut inputs: INPUT_RECORD;
            let mut event_count: u32 = 0;

            ReadConsoleInputW(self.input_handle, &mut inputs as *mut INPUT_RECORD, 1, &mut event_count as *mut u32);

            if event_count > 0 {
                if inputs.event_type == KEY_EVENT {

                    inputs.event.key_event.key_down == 1
                }
            }
            */
        }

        KeyInfo {
            key: 0, ch: '\0', modifiers: 0
        }
    }
}