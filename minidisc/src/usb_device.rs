use std::collections::HashMap;
use std::time::Duration;
use lazy_static::lazy_static;
use debug_stub_derive::DebugStub;

lazy_static! {
    pub static ref NETMD_PIDVID_TO_NAME: HashMap<u32, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0x04dd7202, "Sharp IM-MT899H");
        map.insert(0x054c0075, "Sony MZ-N1");
        map.insert(0x054c0080, "Sony LAM-1");
        map.insert(0x054c0081, "Sony MDS-JB980");
        map.insert(0x054c0084, "Sony MZ-N505");
        map.insert(0x054c0085, "Sony MZ-S1");
        map.insert(0x054c0086, "Sony MZ-N707");
        map.insert(0x054c00c6, "Sony MZ-N10");
        map.insert(0x054c00c7, "Sony MZ-N910");
        map.insert(0x054c00c8, "Sony MZ-N710/NF810");
        map.insert(0x054c00c9, "Sony MZ-N510/N610");
        map.insert(0x054c00ca, "Sony MZ-NE410/NF520D");
        map.insert(0x054c00eb, "Sony MZ-NE810/NE910");
        map.insert(0x054c0101, "Sony LAM-10");
        map.insert(0x054c0113, "Aiwa AM-NX1");
        map.insert(0x054c014c, "Aiwa AM-NX9");
        map.insert(0x054c017e, "Sony MZ-NH1");
        map.insert(0x054c0180, "Sony MZ-NH3D");
        map.insert(0x054c0182, "Sony MZ-NH900");
        map.insert(0x054c0184, "Sony MZ-NH700/NH800");
        map.insert(0x054c0186, "Sony MZ-NH600/NH600D");
        map.insert(0x054c0188, "Sony MZ-N920");
        map.insert(0x054c018a, "Sony LAM-3");
        map.insert(0x054c01e9, "Sony MZ-DH10P");
        map.insert(0x054c0219, "Sony MZ-RH10");
        map.insert(0x054c021b, "Sony MZ-RH710/MZ-RH910");
        map.insert(0x054c022c, "Sony CMT-AH10");
        map.insert(0x054c023c, "Sony DS-HMD1");
        map.insert(0x054c0286, "Sony MZ-RH1");
        map
    };
}

#[derive(DebugStub)]
pub enum UsbError {
    Unknown
}

#[derive(DebugStub)]
pub struct UsbDevice<T: UsbDeviceImplementation> {
    #[debug_stub="ReplacementValue"]
    usb_dev_impl: T,
    vid: u16,    
    pid: u16,
    name: &'static str
}

impl<T: UsbDeviceImplementation> UsbDevice<T> {
    fn write_bulk(self, endpoint: u8, buffer: &[u8], timeout: Duration) -> Result<usize, UsbError> {
        self.usb_dev_impl.write_bulk(endpoint, buffer, timeout)
    }

    fn write_control(self, request_type: u8, request: u8, value: u16, index: u16, buf: &[u8], timeout: Duration) -> Result<usize, UsbError> {
        self.usb_dev_impl.write_control(request_type, request, value, index, buf, timeout)
    }
}

pub trait UsbDeviceImplementation {
    fn open(&mut self) -> Result<(), UsbError>;
    fn write_bulk(self, endpoint: u8, buffer: &[u8], timeout: Duration) -> Result<usize, UsbError>;
    fn write_control(self, request_type: u8, request: u8, value: u16, index: u16, buf: &[u8], timeout: Duration) -> Result<usize, UsbError>;
}