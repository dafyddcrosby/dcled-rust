extern crate libusb;

use std::time::Duration;

// DC board USB ID
const VENDORID: u16 = 0x1d34;
const PRODUCTID: u16 = 0x0013;

struct Board {
    brightness: u8,
    leds: [[bool; 21]; 7],
}

impl Board {
    fn draw(&self) {
        for row in &self.leds {}
    }
}
struct USBPacket {
    brightness: u8,
    row: u8, // TODO better type to lock down to 0,2,4,6?
    data1: [u8; 3],
    data2: [u8; 3],
}

impl USBPacket {
    fn make_packet(&self) -> [u8; 8] {
        // TODO There has to be a simpler way...
        [
            self.brightness,
            self.row,
            self.data1[0],
            self.data1[1],
            self.data1[2],
            self.data2[0],
            self.data2[1],
            self.data2[2],
        ]
    }
}

fn is_dc_board(vendor: u16, product: u16) -> bool {
    vendor == VENDORID && product == PRODUCTID
}

// This would be better handled in the Board struct...
fn write_packet(dh: &libusb::DeviceHandle, packet: &[u8]) {
    dh.write_control(0x21, 0x09, 0x0000, 0x0000, &packet, Duration::new(5, 0))
        .unwrap();
}

fn main() {
    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if is_dc_board(device_desc.vendor_id(), device_desc.product_id()) {
            println!("Found a DC board");
            let handle = device.open().unwrap();

            loop {
                // TODO do something more interesting than writing the diamond test pattern ;-)
                let diamond_test_packets = [
                    [0x00, 0x00, 0xff, 0xfe, 0xff, 0xff, 0xfd, 0x7f],
                    [0x00, 0x02, 0xff, 0xfb, 0xbf, 0xff, 0xf7, 0xdf],
                    [0x00, 0x04, 0xff, 0xfb, 0xbf, 0xff, 0xfd, 0x7f],
                    [0x00, 0x06, 0xff, 0xfe, 0xff, 0x00, 0x00, 0x00],
                ];
                for packet in diamond_test_packets.iter() {
                    write_packet(&handle, packet);
                }
            }
        }
    }
    // TODO handle case when board not attached
}
