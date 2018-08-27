use terminal::{misc, color, term_init::TermInit};

pub struct ProgramLifecycle {
    _term_init: TermInit
}

impl ProgramLifecycle {
    pub fn init() -> ProgramLifecycle {
        let term_init = TermInit::init();
        misc::use_alternate_screen_buffer();

        ProgramLifecycle{ _term_init: term_init }
    }
}

impl Drop for ProgramLifecycle {
    fn drop(&mut self) {
        color::reset();

        misc::use_main_screen_buffer();
    }
}
