use std::{mem::size_of, process, ptr};

use winapi::{
    ctypes::{c_int, c_uint},
    shared::{
        minwindef::{BOOL, DWORD, FALSE, LPARAM, TRUE, WORD},
        windef::HWND__,
    },
    um::winuser::{
        AttachThreadInput, EnumWindows, GetForegroundWindow, GetWindowThreadProcessId, INPUT_u,
        IsWindowVisible, SendInput, SetForegroundWindow, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
        KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, VK_LCONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT,
    },
};

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Key {
    Control = VK_LCONTROL,
    S = 0x53,
    Alt = VK_MENU,
    Shift = VK_SHIFT,
    LeftWindows = VK_LWIN,
    RightWindows = VK_RWIN,
}

#[derive(Clone, Copy)]
#[repr(u32)]
enum Flags {
    ExtKeyUp = KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP,
    KeyDown = 0,
    KeyUp = KEYEVENTF_KEYUP,
}

pub struct StudioKeyboard<'a> {
    process: &'a process::Child,
}

struct KeyEvent<F: Fn()> {
    func: F,
    process_id: u32,
}

impl<'a> StudioKeyboard<'a> {
    pub fn new(process: &'a process::Child) -> StudioKeyboard {
        Self { process }
    }

    pub fn send_chord(&self, keys: &[Key]) {
        unsafe {
            self.hook(|| {
                Self::send_input(keys, Flags::KeyDown);
                Self::send_input(keys, Flags::KeyUp);
            })
        };
    }

    unsafe fn hook<F: Fn()>(&self, func: F) {
        let key_event = KeyEvent {
            func,
            process_id: self.process.id(),
        };

        unsafe extern "system" fn callback<F: Fn()>(
            window: *mut HWND__,
            event_ptr: LPARAM,
        ) -> BOOL {
            let mut window_process: DWORD = 0;
            let window_thread = GetWindowThreadProcessId(window, &mut window_process as *mut _);
            let key_event = event_ptr as *const KeyEvent<F>;

            if window_process == (*key_event).process_id && IsWindowVisible(window) == TRUE {
                StudioKeyboard::try_attach(window_thread, &*key_event);
                FALSE
            } else {
                TRUE
            }
        }

        EnumWindows(Some(callback::<F>), &key_event as *const _ as LPARAM);
    }

    fn try_attach<F: Fn()>(thread: DWORD, key_event: &KeyEvent<F>) {
        let foreground_window = unsafe { GetForegroundWindow() };
        let foreground_thread =
            unsafe { GetWindowThreadProcessId(GetForegroundWindow(), ptr::null_mut()) };

        // This is a hack! We attach Roblox Studio's input processing to the current
        // foreground thread's to ensure Studio receives our keystrokes.

        // AttachThreadInput fails when the attached-to thread is a system thread, or if
        // the attached-to thread doesn't have an input queue. This can happen when the
        // Start Menu or a Command Prompt window is open, among others. Studio will not
        // receive our keystrokes in these cases, so we bail out early.

        // AttachThreadInput also fails when the thread IDs are equal. This means Studio
        // is in the foreground and has keyboard focus. Studio will process our keystrokes
        // in this case, so we can continue.
        if foreground_thread != thread
            && unsafe { AttachThreadInput(thread, foreground_thread, TRUE) } == 0
        {
            log::warn!(
                "Failed to send keystrokes to Roblox Studio. Please save the place manually."
            )
        } else {
            // AttachThreadInput makes the threads share input state, so we need to clear
            // any modifiers that can mess up our keypresses - the "extended" keys are
            // ones on the right side of the keyboard.
            Self::send_input(&[Key::Alt, Key::Shift, Key::LeftWindows], Flags::KeyUp);
            Self::send_input(&[Key::Alt, Key::Shift, Key::RightWindows], Flags::ExtKeyUp);
            (key_event.func)();

            unsafe {
                AttachThreadInput(thread, foreground_thread, FALSE);
                SetForegroundWindow(foreground_window);
            }
        };
    }

    fn send_input(keys: &[Key], flags: Flags) {
        let mut inputs: Vec<INPUT> = keys
            .iter()
            .map(|key| {
                let mut input = INPUT_u::default();

                unsafe {
                    *input.ki_mut() = KEYBDINPUT {
                        dwExtraInfo: 0,
                        dwFlags: flags as DWORD,
                        time: 0,
                        wScan: 0,
                        wVk: *key as WORD,
                    };
                };

                INPUT {
                    type_: INPUT_KEYBOARD,
                    u: input,
                }
            })
            .collect();

        unsafe {
            SendInput(
                inputs.len() as c_uint,
                inputs.as_mut_ptr(),
                size_of::<INPUT>() as c_int,
            )
        };
    }
}
