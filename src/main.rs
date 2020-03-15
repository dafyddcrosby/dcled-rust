extern crate libusb;

use std::thread::sleep;
use std::time::Duration;
// DC board USB ID
const VENDORID: u16 = 0x1d34;
const PRODUCTID: u16 = 0x0013;

#[derive(Copy, Clone)]
struct Row([bool; 21]);

struct Board<'a> {
    device_handle: &'a libusb::DeviceHandle<'a>,
    brightness: u8,
    leds: [Row; 7], // lit if true
}

impl<'a> Board<'a> {
    fn new(dh: &'a libusb::DeviceHandle<'a>) -> Board<'a> {
        Board {
            device_handle: dh,
            brightness: 0,
            leds: [Row([false; 21]); 7],
        }
    }
    fn draw(&self) {
        let packets = [
            USBPacket::new(0, 0, &self.leds[0], &self.leds[1]),
            USBPacket::new(0, 2, &self.leds[2], &self.leds[3]),
            USBPacket::new(0, 4, &self.leds[4], &self.leds[5]),
            USBPacket::new(0, 6, &self.leds[6], &self.leds[6]), // No row 7
        ];

        for packet in packets.iter() {
            Board::<'a>::write_packet(&self, &packet.make_packet());
        }
    }

    fn clear(&mut self) {
        for i in 0..7 {
            self.leds[i] = Row([false; 21]);
        }
    }

    fn write_packet(&self, packet: &[u8]) {
        self.device_handle
            .write_control(0x21, 0x09, 0x0000, 0x0000, &packet, Duration::new(5, 0))
            .unwrap();
    }
}
struct USBPacket {
    brightness: u8,
    row: u8, // TODO better type to lock down to 0,2,4,6?
    data1: [u8; 3],
    data2: [u8; 3],
}

impl USBPacket {
    fn new(brightness: u8, row: u8, data1: &Row, data2: &Row) -> USBPacket {
        // TODO - actually proces data1 and data2
        //for row in self.leds.iter() {
        //    let mut byte: u8 = 0;
        //    for (index, bit) in row.iter().enumerate() {
        //        println!("{} {}", index % 8, bit);
        //        let mask: u8 = 1 << (index % 8);
        //        if *bit {
        //            byte |= mask;
        //        }
        //    }
        //}
        USBPacket {
            brightness: brightness,
            row: row,
            data1: [0xff, 0xfb, 0xbf],
            data2: [0xff, 0xfb, 0xbf],
        }
    }
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

fn main() {
    let context = libusb::Context::new().unwrap();

    for device in context.devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if is_dc_board(device_desc.vendor_id(), device_desc.product_id()) {
            println!("Found a DC board");
            let handle = device.open().unwrap();

            let mut b = Board::new(&handle);
            loop {
                b.draw();
                std::thread::sleep(Duration::from_secs(1));
                b.clear();
            }
        }
    }
    // TODO handle case when board not attached
}
