use clap::App;
use minidisc::libusb_usb_impl;
use minidisc::usb_device;

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .get_matches();

    let usb_pidvids = usb_device::NETMD_PIDVID_TO_NAME.keys().map(|&x| x).collect::<Vec<u32>>();
    let devices = libusb_usb_impl::get_all_devices(usb_pidvids.as_slice());
    println!("{:#?}", devices);
}
