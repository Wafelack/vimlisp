use std::io::{self, Write};
use std::net::UdpSocket;

fn main() {
    let clientaddr: &str = "127.0.0.1:31415";
    let socket = UdpSocket::bind(clientaddr).expect("Failed to create binding");
    print!("Write the target IP>");
    io::stdout().flush().unwrap();
    let mut server_ip = String::new();
    io::stdin()
        .read_line(&mut server_ip)
        .expect("Failed to read server IP");
    let server: &str = server_ip.trim();
    socket
        .connect(server)
        .expect("Cannot connect to ip. Verify if the server is installed on the target machine");
    let mut command = String::new();
    loop {
        command.clear();
        print!("remote@{} $ ", server);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command. please retry");
        let trimmed_cmd: &str = command.trim();
        if trimmed_cmd == "exit" {
            break;
        }
        socket
            .send(trimmed_cmd.as_bytes())
            .expect("Could not send command");
        if trimmed_cmd == "kill" {
            break;
        }
        let mut buf = [0; 1024];
        let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf).unwrap();
        let filled = &mut buf[..number_of_bytes];
        println!("{}", std::str::from_utf8(&filled).unwrap());
    }
}
