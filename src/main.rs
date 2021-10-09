use std::env;
use std::io::prelude::*;
use std::net::{TcpStream, TcpListener, Ipv4Addr};

use rustls;
use webpki_roots;

const PORT: i32 = 9091;

fn handle_client(stream: TcpStream) {
    println!("new connection!");
} 

fn main() -> std::io::Result<()> {
    // setup tls
    let mut root_store = rustls::RootCertStore::empty();
    root_store.add_server_trust_anchors(
	webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
		rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
		)
            })
    );
    let config = rustls::ServerConfig::builder()
	.with_safe_defaults()
	.with_no_client_auth();
    
    // address to connect to
    let address = &format!("192.168.1.239:{}", PORT);
    let rc_config = rustls::server::ServerConfig::builder();
	
    let mut client = rustls::ServerConnection::new(config);

    // parse commandline args
    let args: Vec<String> = env::args().collect();
    let cmd = match args.get(1) {
	Some(arg) => { arg },
	None => { println!("No argument specified."); return Ok(()) },
    };

    // run
    match cmd.as_str() {
	 "listen" => {
	     let mut result = String::new();
	     let listener = TcpListener::bind(address)?;
	     println!("Listening on {}...", address);
	     match listener.accept() {
		 Ok((mut con, _)) => { con.read_to_string(&mut result);
		 println!("Connected!"); },
		 Err(err) => { return Err(err) }, 
	     }
	     println!("{}", result);
	 },
	 "send" => {
	     let mut stream = TcpStream::connect(address)?;
	     stream.write(&[1])?;
	 },
	_ => { println!("Not a valid command") },

    }
    Ok(())
} // the stream is closed here

