pub mod protocol {
    use chrono::Utc;
    use std::sync::{Arc, Mutex};

    pub fn protocol_start(name: &str, parent: &str, protocol: Arc<Mutex<Vec<String>>>) {
        let start = format!("{} starts at {}", name, Utc::now().time());
        let initiated = format!("{} initiated by {}", name, parent);
        println!("{}", start);
        println!("{}", initiated);
        let mut p = protocol.lock().unwrap();
        p.push(start);
        p.push(initiated);
    }

    pub fn protocol_end(name: &str, protocol: Arc<Mutex<Vec<String>>>) {
        let end = format!("{} ends at {}", name, Utc::now().time());
        println!("{}", end);
        let mut p = protocol.lock().unwrap();
        p.push(end);
    }

    pub fn protocol_result(result: i32, name: &str, protocol: Arc<Mutex<Vec<String>>>) {
        let r = format!("{} result: {}", name, result);
        println!("{}", r);
        let mut p = protocol.lock().unwrap();
        p.push(r);
    }

    pub fn protocol_initiates(name: &str, child: &str, protocol: Arc<Mutex<Vec<String>>>) {
        let initiates = format!("{} initiates {}", name, child);
        println!("{}", initiates);
        let mut p = protocol.lock().unwrap();
        p.push(initiates);
    }

    pub fn protocol_result_and_initiates(
        result: String,
        name: &str,
        children: &str,
        protocol: Arc<Mutex<Vec<String>>>,
    ) {
        let r = format!("{} result: {}", name, result);
        let initiates = format!("{} initiates {}", name, children);
        println!("{}", r);
        println!("{}", initiates);
        let mut p = protocol.lock().unwrap();
        p.push(r);
        p.push(initiates);
    }
}
