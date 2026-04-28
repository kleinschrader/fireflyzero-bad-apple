#![no_std]
#![no_main]
#![allow(static_mut_refs)]

pub mod prelude;
use prelude::*;

mod framebuffer;

use core::cell::UnsafeCell;

static mut FRAMEBUFFER: UnsafeCell<Option<framebuffer::Framebuffer>> = UnsafeCell::new(None);

#[unsafe(no_mangle)]
extern "C" fn boot() {
    ff::set_color(ff::Color::new(2), ff::RGB::new(0, 0, 0));
    ff::set_color(ff::Color::new(3), ff::RGB::new(0xFF, 0xFF, 0xFF));

    let fb = framebuffer::Framebuffer::new();

    unsafe {
        *FRAMEBUFFER.get_mut() = Some(fb);
    }
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    // ...
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    unsafe {
        FRAMEBUFFER.get_mut().as_mut().unwrap_unchecked().render();
    }

}
