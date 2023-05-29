use std::{net::TcpStream, io::{Write, Read}, time::Duration};

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // #[arg(long, default_value=DEFAULT_LOCAL_PORT)]
    
    /// IP address of target
    ip: String,

    /// TCP port of target
    port: u16,

    /// Request string
    req: String,
}

fn main() {
    let args = Args::parse();
    let mut stream = match TcpStream::connect(format!("{}:{}", args.ip, args.port)) {
        Ok(s) => s,
        Err(e) => {
            println!("Fail toconnect: {:?}", e);
            return;
        }
    };
    stream.set_read_timeout(Some(Duration::from_millis(2000))).unwrap();
    match stream.write_all(args.req.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            println!("Fail to write: {:?}", e);
            return;
        }
    }
    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => {
            println!("Fail to read: {:?}", e);
            return;
        }
    }
    println!("{}", buf);
}
