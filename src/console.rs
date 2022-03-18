use std::ffi::{c_void};

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
    pub fn new () -> CONSOLE_SCREEN_BUFFER_INFO {
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

pub const FOCUS_EVENT: u16 = 0x0010;
pub const KEY_EVENT: u16 = 0x0001;
pub const MENU_EVENT: u16 = 0x0008;
pub const WINDOW_BUFFER_SIZE_EVENT: u16 = 0x0004;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INPUT_RECORD {
    pub event_type: u16,
    pub event: INPUT_RECORD_EVENT,
}

// STD Handle Values:
const STD_INPUT_HANDLE: u32 = 0xffff_fff6;
const STD_OUTPUT_HANDLE: u32 = 0xffff_fff5;

pub enum Colours {
    Black,
    DarkBLue,
    DarkGreen,
    DarkRed,
    DarkCyan,
    DarkPurple,
    DarkGrey,
    DarkWhite,
    Grey,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Yellow,
    White,
}

const FOREGROUND_BLUE: u16 = 0x0001;
const FOREGORUND_GREEN: u16 = 0x0002;
const FOREGROUND_RED: u16 = 0x0004;
const FOREGROUND_INTENSITY: u16 = 0x0008;
const BACKGROUND_BLUE: u16 = 0x0010;
const BACKGROUND_GREEN: u16 = 0x0020;
const BACKGROUND_RED: u16 = 0x0040;
const BACKGROUND_INTENSITY: u16 = 0x0080;

#[link(name = "Kernel32")]
extern {
    // Win32 API
    fn GetLastError () -> DWORD;

    // Win32 Console API
    fn GetStdHandle (std_handle: u32) -> HANDLE;

    fn GetConsoleScreenBufferInfo(console_handle: HANDLE, screne_buffer_info: *mut CONSOLE_SCREEN_BUFFER_INFO) -> BOOL;

    fn SetConsoleTextAttribute(console_handle: HANDLE, attributtes: u16) -> BOOL;
    fn FillConsoleOutputCharacterW(console_handle: HANDLE, character: u16, length: DWORD, write_coord: COORD, chars_written_count: *mut DWORD) -> BOOL;
    fn FillConsoleOutputAttribute(console_handle: HANDLE, attributes: u16, coordinates: COORD, chars_written: *mut DWORD) -> BOOL;
    fn SetConsoleCursorPosition(console_handle: HANDLE, position: COORD) -> BOOL;

    fn FlushConsoleInputBuffer(console_handle: HANDLE) -> BOOL;
    fn ReadConsoleInputW (input_handle: HANDLE, buffer: *mut INPUT_RECORD, length: u32, number_of_events_read: *mut u32) -> BOOL;
}

pub struct Console {
    output_handle: HANDLE,
    input_handle: HANDLE,
    original_mode: CONSOLE_SCREEN_BUFFER_INFO,
}

impl Console {
    pub fn new () -> Console {
        unsafe {
            // grab everything but hte error handle:
            let output_handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let input_handle = GetStdHandle(STD_INPUT_HANDLE);

            let mut original_mode = CONSOLE_SCREEN_BUFFER_INFO::new();
            
            if GetConsoleScreenBufferInfo(output_handle, &mut original_mode as *mut CONSOLE_SCREEN_BUFFER_INFO) == 0 {
                panic!("Failed to retrieve console mode: {}", GetLastError());
            }

            FlushConsoleInputBuffer(input_handle); // we want to ignore the enter up that occurs when we launch.

            Console {
                output_handle,
                input_handle,
                original_mode
            }
        }
    }

    pub fn quit (&self) {
        unsafe {
            SetConsoleTextAttribute(self.output_handle, self.original_mode.attributes);
        }
    }

    pub fn set_text_position (&self, x: u16, y: u16) {
        unsafe {
            SetConsoleCursorPosition(self.output_handle, COORD{x, y});
        }
    }

    pub fn set_text_color (&self, fore: Colours, back: Colours) {
        let mut text_attributes: u16 = 0;

        unsafe {
            SetConsoleTextAttribute(self.output_handle, text_attributes);
        }
    }

    pub fn clear (&self) {
        unsafe {
            let mut csbi = CONSOLE_SCREEN_BUFFER_INFO::new();
            GetConsoleScreenBufferInfo(self.output_handle, &mut csbi as *mut CONSOLE_SCREEN_BUFFER_INFO);

            let mut chars_written: u32 = 0;
            FillConsoleOutputCharacterW(self.output_handle, 0x20, csbi.size.x as u32 * csbi.size.y as u32, COORD{x: 0, y: 0}, &mut chars_written as *mut u32);
            GetConsoleScreenBufferInfo(self.output_handle, &mut csbi as *mut CONSOLE_SCREEN_BUFFER_INFO);
            FillConsoleOutputAttribute(self.output_handle, csbi.attributes, COORD{x: 0, y: 0}, &mut chars_written as *mut u32);
        }
    }

    pub fn read_key (&self) -> (bool, u16) {
        // Fill with nonsense, so at very least rustc can determine it's not empty.
        let mut input = INPUT_RECORD {
            event_type: 0,
            event: INPUT_RECORD_EVENT {
                focus_event: FOCUS_EVENT_RECORD {
                    set_focus: 0
                }
            }
        };

        unsafe {
            let mut event_count: u32 = 0;

            // TODO(sbell): Do we want to handle other events?
            if ReadConsoleInputW(self.input_handle, &mut input as *mut INPUT_RECORD, 1, &mut event_count as *mut u32) == 1 {
                if event_count > 0 {
                    if input.event_type == KEY_EVENT {
                        let key = input.event.key_event;
                        return (key.key_down == 1, key.virtual_key_code);
                    }
                }
            } else {
                return (false, 0);
            }
        }

        return (false, 0)
    }
}