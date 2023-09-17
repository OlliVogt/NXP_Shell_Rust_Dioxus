pub use serialport::SerialPort;
use std::time::Duration;
pub use std::str;
pub use dioxus::prelude::*;

pub fn open_serial_port(com_port : &str, baud_rate: u32) -> Box<dyn SerialPort> {
    let sport: serialport::SerialPortBuilder = serialport::new(com_port, baud_rate);
    let hundred_millisecond_in_nanos: u32 = 100 * 1000 * 1000;
    let sport: serialport::SerialPortBuilder = sport.timeout(Duration::new(0, hundred_millisecond_in_nanos));
    match sport.open() {
         Ok(port) => 
        {
            print!("serial port open {:?}", port);
            port
        },
        Err(e) =>
        {
            eprintln!("{:?}", e);
            std::process::exit(1);
        } 
    }
}

pub fn search_ports() {
    let ports: Result<Vec<serialport::SerialPortInfo>, serialport::Error>  = serialport::available_ports();
    let ports: Vec<serialport::SerialPortInfo> = match ports {
        Ok(v) => {
            if v.is_empty() {
                println!("No ports found!");
                std::process::exit(1);
            }
            else {
                println!("Following serial ports are found: {:?}", v);   
            }
            v
        },
        Err(e) => {
            println!("error reading available ports {:?}", e);
            panic!("Problem opening the file: {:?}", e);
        }

    };

    let slice: &[serialport::SerialPortInfo] = &ports[..];
    for p in slice.iter().cloned() {
        println!("{}", p.port_name);
    }

}

pub fn send_command(cmd: &str, port: &mut Box<dyn SerialPort>)
{
    read_and_print_data(port, &(cmd.to_owned() + "\n"));
}

fn read_and_print_data(port : &mut Box<dyn SerialPort>, cmd: &str) {
    println!("send: {}", cmd);
    let no_of_send_bytes: usize = match port.write(cmd.as_bytes())
    {
        Ok(s) => s,
        Err(e) => {
            println!("error writing to port: {:?}", e);
            let o: usize = 0;
            o
        }
    };

    println!("send: {} bytes", no_of_send_bytes);

    let mut input_buf: [u8; 10240] = [0; 10240];
    let _n = match port.read(&mut input_buf[..]) {
        Ok(s) => s,
        Err(e) => {
            println!("error reading from port: {:?}", e);
            let o: usize = 0;
            o
        }
    };

    println!("receive: {}", str::from_utf8(&input_buf).unwrap());
}