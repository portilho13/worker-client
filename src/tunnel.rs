use std::{io::Write, net::{TcpListener, TcpStream}};

pub fn create_tcp_conn(ip: String) -> Result<(), std::io::Error> {
    let mut conn = match TcpStream::connect(ip) {
        Ok(conn) => {
            println!("Successfully connect to ip {}", conn.local_addr().unwrap());
            conn
        },
        Err(e) => {
            println!("Error : {}", e);
            return Err(e);
        }
    };

    let data = "Hello World";
    let data_len = data.len() as u32;

    let data_len_bytes = data_len.to_be_bytes();
    match conn.write_all(&data_len_bytes) {
        Ok(n) => {
            println!("Successfully sended {:?} bytes", n);
        },
        Err(e) => {
            println!("Error {}", e);
            return Err(e);
        }
    };

    match conn.write_all(&data.as_bytes()) {
        Ok(_) => {
            println!("Successfully sended data to IP");
        },
        Err(e) => {
            println!("Error {}", e);
            return Err(e);
        }
    };

    Ok(())
}