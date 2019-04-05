use crate::netmd_device_raw::NetMDDeviceRaw;

pub struct NetMDDevice<'a> {
    protocol_layer: NetMDDeviceRaw<'a>
}
