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
struct COORD {
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
union KEY_INPUT_CHAR {
    unicode_char: u16,
    ascii_char: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct KEY_EVENT_RECORD {
    key_down: i32,
    repeat_count: u16,
    virtual_key_code: u16,
    virtual_scan_code: u16,
    uchar: KEY_INPUT_CHAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MOUSE_EVENT_RECORD {
    mouse_position: COORD,
    button_state: u32,
    control_key_state: u32,
    event_flags: u32
}


#[repr(C)]
#[derive(Copy, Clone)]
struct WINDOW_BUFFER_SIZE_RECORD {
    size: COORD
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MENU_EVENT_RECORD {
    command_id: u32
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FOCUS_EVENT_RECORD {
    set_focus: BOOL
}

#[repr(C)]
#[derive(Copy, Clone)]
union INPUT_RECORD_EVENT {
    key_event: KEY_EVENT_RECORD,
    mouse_event: MOUSE_EVENT_RECORD,
    window_buffer_size_event: WINDOW_BUFFER_SIZE_RECORD,
    menu_event: MENU_EVENT_RECORD,
    focus_event: FOCUS_EVENT_RECORD,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct INPUT_RECORD {
    event_type: u16,
    event: INPUT_RECORD_EVENT,
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

    fn ReadConsoleInputW (input_handle: HANDLE, buffer: *mut INPUT_RECORD, length: i32, number_of_events_read: *mut i32) -> BOOL;
}

pub struct Console {
    output_handle: HANDLE,
    input_handle: HANDLE,
    original_mode: u32,
}

impl Console {
    pub fn init () -> Console {
        unsafe {
            // grab everything but hte error handle:
            let output_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let input_handle = GetStdHandle(STD_INPUT_HANDLE);

            let mut original_mode: DWORD = 0;
            let mode_ptr = &mut original_mode as *mut DWORD;
            
            if GetConsoleMode(output_handle, mode_ptr) == 0 {
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

    pub fn in_key (&self) -> u32 {
        unimplemented!()
    }
}