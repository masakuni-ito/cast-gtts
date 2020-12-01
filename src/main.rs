#[macro_use]
extern crate clap;
extern crate rust_cast;
extern crate gtts;

use clap::{App, Arg};

use rust_cast::CastDevice;
use rust_cast::channels::media::{Media, StreamType};
use rust_cast::channels::receiver::CastDeviceApp;

use std::str::FromStr;

const MAX_CHARS_LENGTH: usize = 200;
const DEFAULT_DESTINATION_ID: &str = "receiver-0";

fn validate_chars_length(v: String) -> Result<(), String> {
    if v.chars().count() > MAX_CHARS_LENGTH {
        return Err(String::from("ERROR: The specified number of characters has been exceeded."));
    }
    Ok(())
}

fn main() {

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("ip")
            .help("sample flag")
            .short("i")
            .long("ip")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("port")
            .help("sample flag")
            .short("p")
            .long("port")
            .default_value("8009")
            .takes_value(true)
        )
        .arg(Arg::with_name("text")
            .help("sample flag")
            .short("t")
            .long("text")
            .takes_value(true)
            .required(true)
            .validator(validate_chars_length)
        )
        .get_matches();

    let ip: &str = matches.value_of("ip").unwrap();
    let port: u16 = matches.value_of("port").unwrap().parse().unwrap();
    let text: &str = matches.value_of("text").unwrap();

    let cast_device = CastDevice::connect_without_host_verification(ip, port).unwrap();

    cast_device
        .connection
        .connect(DEFAULT_DESTINATION_ID.to_string())
        .unwrap();

    let app_to_run = &CastDeviceApp::from_str("default").unwrap();
    let app = cast_device.receiver.launch_app(app_to_run).unwrap();

    cast_device
        .connection
        .connect(app.transport_id.as_str())
        .unwrap();

    cast_device
        .media
        .load(
            app.transport_id.as_str(),
            app.session_id.as_str(),
            &Media {
                content_id: gtts::get_url(text, "ja"),
                content_type: "audio/mp3".to_string(),
                stream_type: StreamType::from_str("buffered").unwrap(),
                duration: None,
                metadata: None,
            },
        )
        .unwrap();
}
