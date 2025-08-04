fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:8001");
    for _ in listener.unwrap().incoming(){
        println!("New connection established");
    }
    Ok(())
}
