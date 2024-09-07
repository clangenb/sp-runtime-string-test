#![no_std]

fn main() {
    let string = sp_runtime::format_runtime_string!("Hello Worlds");

    log::info!("{:?}", string);
}