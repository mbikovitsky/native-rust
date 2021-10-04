#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const STATUS_SUCCESS: NTSTATUS = NTSTATUS(0);
pub const STATUS_UNSUCCESSFUL: NTSTATUS = NTSTATUS(0xC0000001);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NTSTATUS(pub u32);

impl NTSTATUS {
    pub fn ok(&self) -> Result<Self, Self> {
        if (self.0 as i32) >= 0 {
            Ok(*self)
        } else {
            Err(*self)
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct HANDLE(pub isize);

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct BOOLEAN(u8);

impl From<bool> for BOOLEAN {
    fn from(value: bool) -> Self {
        Self(value as _)
    }
}

impl From<BOOLEAN> for bool {
    fn from(value: BOOLEAN) -> Self {
        value.0 != 0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default)]
pub struct LARGE_INTEGER(pub i64);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct UNICODE_STRING {
    Length: u16,
    MaximumLength: u16,
    Buffer: *const u16,
}

impl Default for UNICODE_STRING {
    fn default() -> Self {
        Self {
            Length: 0,
            MaximumLength: 0,
            Buffer: core::ptr::null(),
        }
    }
}

pub fn NtCurrentProcess() -> HANDLE {
    HANDLE(-1)
}

#[link(name = "ntdll")]
extern "system" {
    pub fn RtlInitUnicodeString(DestinationString: *mut UNICODE_STRING, SourceString: *const u16);

    pub fn NtTerminateProcess(ProcessHandle: HANDLE, ExitStatus: NTSTATUS) -> NTSTATUS;

    pub fn NtDelayExecution(Alertable: BOOLEAN, DelayInterval: *const LARGE_INTEGER) -> NTSTATUS;

    pub fn NtDrawText(Text: *const UNICODE_STRING) -> NTSTATUS;
}
