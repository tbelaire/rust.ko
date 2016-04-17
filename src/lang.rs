use core;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "eh_unwind_resume"]
extern "C" fn eh_unwind_resume() {}

#[lang = "panic_fmt"]
extern "C" fn panic_impl(_: core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}
