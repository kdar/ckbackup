#![cfg(windows)]
extern crate winapi;
use winapi::*;

pub const DDD_REMOVE_DEFINITION: DWORD = 0x00000002;
pub const DDD_RAW_TARGET_PATH: DWORD = 0x00000001;

extern "system" {
  //pub fn Beep(dwFreq: DWORD, dwDuration: DWORD) -> BOOL;
  pub fn DefineDosDeviceW(dwFlags: DWORD, lpDeviceName: LPCWSTR, lpTargetPath: LPCWSTR) -> BOOL;
  pub fn DefineDosDeviceA(dwFlags: DWORD, lpDeviceName: LPCSTR, lpTargetPath: LPCSTR) -> BOOL;
}
