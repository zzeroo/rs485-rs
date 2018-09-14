extern crate rs485;
/// This Example shows how to interact with the `serial-rs` crate
///
extern crate serial;

use rs485::*;
use serial::prelude::*;
use std::env;
use std::thread;
use std::time::Duration;

const TTY_PATH1: &str = "/dev/ttyUSB0";
const TTY_PATH2: &str = "/dev/ttyUSB1";

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

fn main() {
    let mut args = env::args_os().skip(1);
    let tty_path1 = args.next().unwrap_or_else(|| TTY_PATH1.into() );
    let port_path2 = args.next().unwrap_or_else(|| TTY_PATH2.into() );

    println!("opening port: {:?}", &tty_path1);
    let mut port = serial::open(&tty_path1).unwrap();
    let mut rs485_settings = SerialRs485::new();
    rs485_settings = *rs485_settings.set_rts_on_send(true);
    port.set_rs485_conf(&rs485_settings)
        .expect("Could not add the rs485 settings to port");
    thread::spawn(move || {
        write(&mut port).unwrap();
    });

    println!("opening port: {:?}", &port_path2);
    let mut port = serial::open(&port_path2).unwrap();
    thread::spawn(move || {
        read(&mut port).unwrap();
    });

    thread::sleep(Duration::from_millis(1000));
}

fn read<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    port.configure(&SETTINGS)?;
    port.set_timeout(Duration::from_secs(1))?;

    let mut buf: Vec<u8> = vec![0; 255];

    println!("reading bytes");
    port.read_exact(&mut buf[..])?;

    println!("{:?}", buf);
    Ok(())
}

fn write<T: SerialPort>(port: &mut T) -> serial::Result<()> {
    port.configure(&SETTINGS)?;
    port.set_timeout(Duration::from_secs(1))?;

    let buf: Vec<u8> = (0..255).collect();

    println!("writing bytes");
    port.write(&buf[..])?;

    Ok(())
}
