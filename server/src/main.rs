use std::net::UdpSocket;
use std::time::Duration;
use std::str;

fn main() -> std::io::Result<()> {
        let address = "127.0.0.1:45000";
        let socket = UdpSocket::bind(address).expect("couldn't bind to address");
        let mut recv_buff = [0; 8092];

        socket.set_read_timeout(Some(Duration::new(60, 0)))?;
        socket.set_broadcast(true)?;
        
        println!("Forbundet på adresse: {}", address);
        println!("Broadcast: {:?}", socket.broadcast());
        println!("Timeout: {:?}", socket.read_timeout());

        println!("Venter på respons...");   // self.recv_buff is a [u8; 8092]
        
        while let Ok((n, addr)) = socket.recv_from(&mut recv_buff) {
            println!("{} bytes response from {:?}", n, addr);
            
            let s = match str::from_utf8(&recv_buff) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            println!("Received: {}", s); 
        }
         
    Ok(())
}
