🔎 Rust Multi-threaded Port Scanner

A simple and fast multi-threaded port scanner written in Rust.
It demonstrates concurrency, networking, and safe shared state handling in Rust using std::thread, Arc, and Mutex.

⸻

🚀 Features
	•	Scans a range of TCP ports on a given host.
	•	Uses multiple threads for high performance.
	•	Detects open ports and prints them in a clean list.
	•	Configurable thread pool size.
	•	Timeout handling for faster scans.

⸻

🛠 How it Works
	•	Ports are collected into a shared queue (Vec<u16> inside Mutex).
	•	Multiple worker threads fetch ports from the queue.
	•	Each thread attempts a TCP connection with a short timeout.
	•	If the port is open, it’s added to the results list.
	•	Finally, all open ports are displayed.

⸻

📦 Installation

Clone the repo and build with Cargo:
```bash
git clone https://github.com/your-username/port_scanner.git
cd port_scanner
cargo build --release
```

▶️ Usage

Run the scanner with:
```bash
cargo run
```

Default settings:
	•	Host: 127.0.0.1
	•	Ports: 1-1024
	•	Threads: 50

📂 Example Output
```bash
Open ports:
  22
  80
  443
```

🔮 Roadmap
	•	Add CLI arguments (--host, --ports, --threads) using clap.
	•	JSON output with serde.
	•	Progress bar with indicatif.
	•	UDP scanning.

⸻

📚 Learning Goals

This project is part of my Rust learning journey (Week 6):
	•	Practicing threads, Arc, and Mutex.
	•	Understanding safe concurrency in Rust.
	•	Applying network programming concepts.

⸻

⚖️ Disclaimer

This project is for educational purposes only.
Please do not scan networks or hosts without permission.

⸻

Would you like me to also prepare a version with clap integration so your README can include CLI usage like:
```bash
cargo run -- --host scanme.nmap.org --ports 1-500 --threads 100
```
