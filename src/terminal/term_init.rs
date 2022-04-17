use libc;

type DWORD = libc::c_ulong;
type LPDWORD = *mut DWORD;
type HANDLE = *mut libc::c_void;
type BOOL = libc::c_int;

const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
const STD_OUTPUT_HANDLE: DWORD = 0xFFFFFFF5;
const STD_INPUT_HANDLE: DWORD = 0xFFFFFFF6;
const FALSE: BOOL = 0;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
const DISABLE_NEWLINE_AUTO_RETURN: DWORD = 0x0008;
const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;
const ENABLE_PROCESSED_INPUT:DWORD = 0x0001;

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
#[allow(non_snake_case)]
extern "system" {
    pub fn GetStdHandle(console_type: DWORD) -> HANDLE;

    fn GetConsoleMode(
        handle: HANDLE,
        mode: LPDWORD, /*pointer to double word var that receives the mode*/
    ) -> BOOL;

    fn SetConsoleMode(handle: HANDLE, mode: DWORD) -> BOOL;
}

pub struct TermInit {
    hOut: HANDLE,
    hIn: HANDLE,
    originalOutputMode: DWORD,
    originalInputMode: DWORD,
}

impl TermInit {
    #[cfg(target_os = "windows")]
    pub fn init() -> TermInit {
        let hOut: HANDLE;
        let hIn: HANDLE;
        unsafe {
            hOut = GetStdHandle(STD_OUTPUT_HANDLE);
            hIn = GetStdHandle(STD_INPUT_HANDLE);
        }
        if hOut == INVALID_HANDLE_VALUE {
            panic!("Cannot GetStdHandle for stdout");
        }

        if hIn == INVALID_HANDLE_VALUE {
            panic!("Cannot GetStdHandle for stdin");
        }

        let mut originalOutputMode: DWORD = 0;
        let p_originalOutputMode: *mut DWORD = &mut originalOutputMode;

        let mut originalInputMode: DWORD = 0;
        let p_originalInputMode: *mut DWORD = &mut originalInputMode;

        unsafe {
            if GetConsoleMode(hOut, p_originalOutputMode) == FALSE {
                panic!("could not get the original stdout console mode");
            }

            if GetConsoleMode(hIn, p_originalInputMode) == FALSE {
                panic!("could not get the original stdin console mode")
            }
        }

        let requested_output_modes =
            originalOutputMode | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;

        let requested_input_modes = ENABLE_VIRTUAL_TERMINAL_INPUT | ENABLE_PROCESSED_INPUT;

        unsafe {
            if SetConsoleMode(hOut, requested_output_modes) == FALSE {
                panic!("could not set the requested stdout console mode");
            }

            if SetConsoleMode(hIn, requested_input_modes) == FALSE {
                panic!("could not set the requested stdin console mode");
            }
        }

        debug!("successfully set console mode");
        TermInit {
            hOut,
            hIn,
            originalOutputMode,
            originalInputMode,
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn init() -> TermInit {
        TermInit {
            hOut: INVALID_HANDLE_VALUE,
            hIn: INVALID_HANDLE_VALUE,
            originalOutputMode: 0,
            originalInputMode: 0,
        }
    }
}

impl Drop for TermInit {
    fn drop(&mut self) {
        if self.hOut != INVALID_HANDLE_VALUE {
            unsafe {
                if SetConsoleMode(self.hOut, self.originalOutputMode) == FALSE {
                    error!("could not reset the original stdout console mode");
                }
            }
        }

        if self.hIn != INVALID_HANDLE_VALUE {
            unsafe {
                if SetConsoleMode(self.hIn, self.originalInputMode) == FALSE {
                    error!("could not reset the original stdin console mode");
                }
            }
        }
    }
}
