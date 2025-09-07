use std::net::{TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let target = "127.0.0.1".to_string(); // Change to your target host
    let start_port = 1;
    let end_port = 60_000;
    let thread_count = 100;

    let ports: Vec<u16> = (start_port..=end_port).collect();
    let ports = Arc::new(Mutex::new(ports));
    let open_ports = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for _ in 0..thread_count {
        let ports = Arc::clone(&ports);
        let open_ports = Arc::clone(&open_ports);
        let target = target.clone();

        let handle = thread::spawn(move || {
            loop {
                let port = {
                    let mut ports = ports.lock().unwrap();
                    if ports.is_empty() {
                        break;
                    }
                    ports.pop()
                };

                if let Some(port) = port {
                    if scan_port(&target, port) {
                        let mut open = open_ports.lock().unwrap();
                        open.push(port);
                    }
                } else {
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let open_ports = open_ports.lock().unwrap();
    println!("Open ports:");
    for port in open_ports.iter().cloned().collect::<Vec<u16>>() {
        println!("  {}", port);
    }
}

fn scan_port(target: &str, port: u16) -> bool {
    let addr = format!("{}:{}", target, port);
    if let Ok(_) = TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        Duration::from_millis(200),
    ) {
        true
    } else {
        false
    }
}