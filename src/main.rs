#![no_std]
#![no_main]

mod crt;
mod ntapi;

use core::panic::PanicInfo;

use wide_literals::w;

use ntapi::{
    NtCurrentProcess, NtDelayExecution, NtDrawText, NtTerminateProcess, RtlInitUnicodeString,
    LARGE_INTEGER, NTSTATUS, STATUS_SUCCESS, STATUS_UNSUCCESSFUL, UNICODE_STRING,
};

#[no_mangle]
pub extern "system" fn _start() -> ! {
    let status = match main() {
        Ok(status) => status,
        Err(error) => error,
    };
    exit_process(status);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    exit_process(STATUS_UNSUCCESSFUL);
}

fn exit_process(exit_status: NTSTATUS) -> ! {
    unsafe {
        NtTerminateProcess(NtCurrentProcess(), exit_status);
    }
    unreachable!()
}

fn main() -> Result<NTSTATUS, NTSTATUS> {
    let mut text = UNICODE_STRING::default();
    unsafe {
        RtlInitUnicodeString(&mut text, w!("Hello, Native World!"));
    }

    unsafe {
        NtDrawText(&text).ok()?;
    }

    unsafe {
        NtDelayExecution(false.into(), &LARGE_INTEGER(-10000 * 5000)).ok()?;
    }

    Ok(STATUS_SUCCESS)
}
