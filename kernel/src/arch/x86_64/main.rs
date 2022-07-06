use bkshared::Handover;

use crate::debug::{framebuffer::DebugFramebuffer, color::Color};


#[no_mangle]
extern "sysv64" fn main(handover: *mut Handover) -> ! {
    let mut handover = unsafe { *handover };
    let mut framebuffer = DebugFramebuffer::new(handover.framebuffer, handover.font, Color::DarkBlue, Color::Black, 4);
    unsafe { framebuffer.clear_screen(Color::Cyan); }
    
    
    
    // crate::main();
    loop {}
}