#![no_std]
#![no_main]

use core::panic::PanicInfo;

//  이 함수는 panic 발생 시 호출됩니다.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static GREETING: &[u8] = b"The curtain rises.";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let text_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in GREETING.iter().enumerate() {
        unsafe {
            *text_buffer.offset(i as isize * 2) = byte;
            *text_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}