// #![no_std]

#![cfg(windows)]
extern crate winapi;
use winapi::*;

extern "system" {
  pub fn SetSuspendState(Hibernate: BOOL, ForceCritical: BOOL, DisableWakeEvent: BOOL) -> BOOL;
}
