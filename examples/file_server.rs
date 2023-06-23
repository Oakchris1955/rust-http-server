use oak_http_server::{
    handlers::{read_diff_dir, read_same_dir},
    Server,
};

fn main() {
    let hostname = "localhost";
    let port: u16 = 2300;

    let mut server = Server::new(hostname, port);

    server.on_directory("/www/same", read_same_dir);

    server.on_directory("/different", read_diff_dir("/www/different"));

    server.start(|| {
        println!("Started file server");
    });
}
