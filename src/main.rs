mod tunnel;
mod files;
mod args;

const SERVER_TCP_IP: &str = "127.0.0.1:1234";

fn main() {

    let args = args::get_args();

    let folder = &args[0];

    let data = files::collect_entries(folder.to_string()).unwrap();

    println!("{:?}", data);
    
    let conn = tunnel::create_tcp_conn(SERVER_TCP_IP.to_string()).unwrap();
    
    tunnel::send_data(conn, data).unwrap();
}
