#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::*;

const HELLO_STR: &str = "Hello, world. Press any key to return to UEFI firmware.";

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    println!("{}", HELLO_STR);

    let mut events = [ system_table.stdin().wait_for_key_event().unwrap() ];
    system_table.boot_services().wait_for_event(&mut events).discard_errdata().unwrap();

    Status::SUCCESS
}
