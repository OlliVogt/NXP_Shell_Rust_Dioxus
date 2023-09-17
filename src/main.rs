use std::collections::HashMap;
use dioxus_desktop::Config;
use std::cell::RefCell;

mod serial_port;
use crate::serial_port::open_serial_port;
use crate::serial_port::SerialPort;
use crate::serial_port::*;

const COM_PORT: &str = "COM3";
const BAUD_RATE: u32 = 115200;

fn main() {
    println!("Program starts!");

    serial_port::search_ports();

    let app_props = AppProps {
        port: RefCell::new(open_serial_port(COM_PORT, BAUD_RATE))
    };

    dioxus_desktop::launch_with_props(
        app,
        app_props,
        Config::default(),
    )
}

struct AppProps {
    port: RefCell<Box<dyn SerialPort>>,
}

fn app(cx: Scope<AppProps>) -> Element {
    let led0_state: &UseState<bool> = use_state(cx, || false);
    let led1_state: &UseState<bool> = use_state(cx, || false);

    let led_state: HashMap<bool, &str> = HashMap::from([
        (false, "off"),
        (true, "on"),
    ]);
    
    cx.render(rsx! {
            h1 { "LED 0 state: {led_state[led0_state]}"}
            h1 { "LED 1 state: {led_state[led1_state]}" }
            button { onclick: move |_| 
                {
                    led0_state.set(!led0_state);
                    let led = "led 1 ".to_owned();
                    let output = led + led_state[&!led0_state];
                    serial_port::send_command( &output, &mut *cx.props.port.borrow_mut());
                }, "LED 0" }
            button { onclick: move |_| 
                {
                    let led_state: HashMap<bool, &str> = HashMap::from([
                        (false, "off"),
                        (true, "on"),
                    ]);

                    led1_state.set(!led1_state);
                    let led = "led 2 ".to_owned();
                    let output = led + led_state[&!led1_state];
                    serial_port::send_command( &output, &mut *cx.props.port.borrow_mut());
                }, "LED 1" }
            button { onclick: move |_| 
                {
                    serial_port::send_command( "help", &mut *cx.props.port.borrow_mut());
                }, "help" }
    })
}
