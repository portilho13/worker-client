use std::{io::Write, net::{TcpListener, TcpStream}};

use crate::files;

pub fn create_tcp_conn(ip: String) -> Result<TcpStream, std::io::Error> {
    match TcpStream::connect(&ip) {
        Ok(conn) => {
            println!("Successfully connect to ip {}", conn.local_addr().unwrap());
            Ok(conn)
        },
        Err(e) => {
            println!("Error : {}", e);
            return Err(e);
        }
    }
}

pub fn send_data(mut conn: TcpStream, data: Vec<u8>) -> Result<(), std::io::Error> {
    
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

    match conn.write_all(&data) {
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