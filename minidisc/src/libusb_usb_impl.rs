use crate::usb_device::{UsbDeviceImplementation, UsbError};
use libusb::{Context, Device, DeviceHandle, Error};
use std::time::Duration;
use debug_stub_derive::DebugStub;
use lazy_static::lazy_static;

lazy_static! {
    static ref LIBUSB_CONTEXT: Context = {
        Context::new().unwrap()
    };
}

#[derive(DebugStub)]
pub struct LibusbUsbDevice<'a> {
    #[debug_stub="ReplacementValue"]
    _device: Device<'a>,
    #[debug_stub="ReplacementValue"]
    _deviceHandle: Option<DeviceHandle<'a>>,
}

impl From<Error> for UsbError {
    fn from(error: Error) -> Self {
        UsbError::Unknown
    }
}

impl<'a> UsbDeviceImplementation for LibusbUsbDevice<'a> {
    fn open(&mut self) -> Result<(), UsbError> {
        self._device.open()?;
        Result::Ok(())
    }

    fn write_bulk(self, endpoint: u8, buffer: &[u8], timeout: Duration) -> Result<usize, UsbError> {
        let size = self
            ._deviceHandle
            .unwrap()
            .write_bulk(endpoint, buffer, timeout)?;
        Result::Ok(size)
    }

    fn write_control(
        self,
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        buf: &[u8],
        timeout: Duration,
    ) -> Result<usize, UsbError> {
        let size = self._deviceHandle.unwrap().write_control(
            request_type,
            request,
            value,
            index,
            buf,
            timeout,
        )?;
        Result::Ok(size)
    }
}

pub fn get_all_devices<'a>(
    filter_vidpids: &[u32],
) -> Result<Vec<LibusbUsbDevice<'a>>, UsbError> {
    let libusb_devices = LIBUSB_CONTEXT.devices()?;

    Result::Ok(
        libusb_devices
            .iter()
            .filter(|dev| {
                let dev_desc_result = dev.device_descriptor();
                let dev_desc = match dev_desc_result {
                    Ok(dev_desc) => dev_desc,
                    Err(_usb_err) => return false,
                };
                let vidpid = ((dev_desc.vendor_id() as u32) << 16) | (dev_desc.product_id() as u32);
                filter_vidpids.contains(&vidpid)
            })
            .map(|dev| {
                return LibusbUsbDevice {
                    _device: dev,
                    _deviceHandle: Option::None,
                };
            })
            .collect(),
    )
}
