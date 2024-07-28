
use core::panic::PanicInfo;
use crate::acpi::Pm1Cnt;
pub fn shutdown() -> ! {
    let mut pm1_cnt: Pm1Cnt = Pm1Cnt::empty();
    pm1_cnt.set_s5().write();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        // Some(location) => {
        //     println!(
        //         "[kernel] panicked at '{}', {}:{}:{}",
        //         info.message().unwrap(),
        //         location.file(),
        //         location.line(),
        //         location.column()
        //     );
        // }
        Some(location) => {
            todo!(
                "[kernel] panicked at '{}', {}:{}:{}",
                info.message().unwrap(),
                location.file(),
                location.line(),
                location.column()
            );
        }
        //None => println!("[kernel] panicked at '{}'", info.message().unwrap()),
        None => todo!("[kernel] panicked at '{}'", info.message().unwrap()),

    }
    shutdown()
}

#[macro_export]
macro_rules! color_text {
    ($text:expr, $color:expr) => {{
        format_args!("\x1b[{}m{}\x1b[0m", $color, $text)
    }};
}

pub trait Bytes<T> {
    fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<T>();
        unsafe {
            core::slice::from_raw_parts(self as *const _ as *const T as usize as *const u8, size)
        }
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        let size = core::mem::size_of::<T>();
        unsafe {
            core::slice::from_raw_parts_mut(self as *mut _ as *mut T as usize as *mut u8, size)
        }
    }
}
