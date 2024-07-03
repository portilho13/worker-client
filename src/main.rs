mod tunnel;

const SERVER_TCP_IP: &str = "127.0.0.1:1234";

fn main() {
    tunnel::create_tcp_conn(SERVER_TCP_IP.to_string()).unwrap();
}
