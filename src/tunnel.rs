use std::{io::Write, net::{TcpListener, TcpStream}};

use crate::files;

pub fn create_tcp_conn(ip: String) -> Result<TcpStream, std::io::Error> {
    TcpStream::connect(&ip).map(|conn| {
        println!("Successfully connected to ip {}", conn.local_addr().unwrap());
        conn
    }).map_err(|e| {
        println!("Error : {}", e);
        e
    })
}

pub fn send_data(mut conn: TcpStream, data: Vec<u8>) -> Result<(), std::io::Error> {
    let data_len = data.len() as u32;
    let data_len_bytes = data_len.to_be_bytes();
    
    conn.write_all(&data_len_bytes).map_err(|e| {
        println!("Error writing length: {}", e);
        e
    })?;
    
    conn.write_all(&data).map_err(|e| {
        println!("Error writing data: {}", e);
        e
    })?;
    
    println!("Successfully sent data to IP");
    Ok(())
}