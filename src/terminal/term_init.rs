use libc;

type DWORD = libc::c_ulong;
type LPDWORD = *mut DWORD;
type HANDLE = *mut libc::c_void;
type BOOL = libc::c_int;

const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
const STD_OUTPUT_HANDLE: DWORD = 0xFFFFFFF5;
const FALSE: BOOL = 0;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;
const DISABLE_NEWLINE_AUTO_RETURN: DWORD = 0x0008;

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
    originalMode: DWORD,
}

impl TermInit {
    #[cfg(target_os = "windows")]
    pub fn init() -> TermInit {
        let hOut: HANDLE;
        unsafe {
            hOut = GetStdHandle(STD_OUTPUT_HANDLE);
        }
        if hOut == INVALID_HANDLE_VALUE {
            panic!("Cannot GetStdHandle");
        }

        let mut originalMode: DWORD = 0;
        let p_originalMode: *mut DWORD = &mut originalMode;

        unsafe {
            if GetConsoleMode(hOut, p_originalMode) == FALSE {
                panic!("could not get the original console mode");
            }
        }

        let requested_modes =
            originalMode | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;

        unsafe {
            if SetConsoleMode(hOut, requested_modes) == FALSE {
                panic!("could not set the requested console mode");
            }
        }

        println!("successfully set console mode");
        TermInit { hOut, originalMode }
    }

    #[cfg(not(target_os = "windows"))]
    fn init() -> TermInit {
        TermInit {
            hOut: INVALID_HANDLE_VALUE,
            originalMode: 0,
        }
    }
}

impl Drop for TermInit {
    fn drop(&mut self) {
        if self.hOut == INVALID_HANDLE_VALUE {
            return;
        }

        // set console's original mode
        unsafe {
            if SetConsoleMode(self.hOut, self.originalMode) == FALSE {
                eprintln!("could not reset the original console mode");
            }
        }
    }
}
