use crate::usb_device::UsbDevice;
use crate::libusb_usb_impl::LibusbUsbDevice;

pub struct NetMDDeviceRaw<'a> {
    usb_layer: UsbDevice<LibusbUsbDevice<'a>>
}

impl<'a> NetMDDeviceRaw<'a> {

}