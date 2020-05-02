use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
extern crate chrono;
use chrono::Utc;
use std::fs;
// use std::io;

const DELAY: u64 = 0;

struct Result1 {
    f1: Option<i32>,
    f2: Option<i32>,
    f7: Option<i32>,
}

struct Result2 {
    f4: Option<i32>,
    f5: Option<i32>,
    f6: Option<i32>,
}

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let protocol = Arc::new(Mutex::new(Vec::<String>::new()));

    let pa = Arc::clone(&protocol);
    thread::spawn(|| a_task(tx, pa));

    println!("Result: {}", rx.iter().next().unwrap());

    const RESULT_PATH: &'static str = "protocol.txt";
    let data = protocol.lock().unwrap().join("\n");
    fs::write(RESULT_PATH, data).unwrap();
    println!("Completed! The protocol is in the {}", RESULT_PATH);

    // let mut option = String::new();
    // match io::stdin().read_line(&mut option) {
    //     Ok(_) => {}
    //     Err(e) => eprintln!("Read error: {}", e),
    // }
}

fn a_task(tx: mpsc::Sender<i32>, p: Arc<Mutex<Vec<String>>>) {
    let mut protocol = p.lock().unwrap();
    let start = format!("A starts at {}", Utc::now().time());
    println!("{}", start);
    protocol.push(start);
    drop(protocol);

    let m1 = [1, 2, 3];
    let m2 = [4, 5, 6];
    let m3 = [7, 8, 8];

    thread::sleep(Duration::from_millis(DELAY));

    let result = format!("A Result: M1: {:?}, M2: {:?}, M3: {:?}", m1, m2, m3);
    let initiates = String::from("A initiates B, C, D");
    let mut protocol = p.lock().unwrap();
    println!("{}", result);
    println!("{}", initiates);
    protocol.push(result);
    protocol.push(initiates);
    drop(protocol);

    let result1 = Arc::new(Mutex::new(Result1 {
        f1: Option::<i32>::None,
        f2: Option::<i32>::None,
        f7: Option::<i32>::None,
    }));

    let txb = mpsc::Sender::clone(&tx);
    let pb = Arc::clone(&p);
    let result1b = Arc::clone(&result1);
    thread::spawn(move || b_task(txb, pb, m1, m2, m3, result1b));
    let txc = mpsc::Sender::clone(&tx);
    let pc = Arc::clone(&p);
    let result1c = Arc::clone(&result1);
    thread::spawn(move || c_task(txc, pc, m1, m2, m3, result1c));
    let pd = Arc::clone(&p);
    thread::spawn(move || d_task(tx, pd, m1, m2, m3, result1));

    let mut protocol = p.lock().unwrap();
    let end = format!("A ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn b_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
    result1: Arc<Mutex<Result1>>,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("B starts at {}", Utc::now().time());
    println!("{}", start);
    println!("B initiated by A");
    protocol.push(start);
    protocol.push(String::from("B initiated by A"));
    drop(protocol);

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    thread::sleep(Duration::from_millis(3 * DELAY));

    let result = format!("B result: {}", s);
    let mut protocol = p.lock().unwrap();
    println!("{}", result);
    protocol.push(result);
    drop(protocol);

    let mut result = result1.lock().unwrap();
    result.f1 = Some(s);

    match result.f2 {
        Some(b) => match result.f7 {
            Some(c) => {
                let mut protocol = p.lock().unwrap();
                let initiates = String::from("B initiates K");
                println!("{}", initiates);
                protocol.push(initiates);
                drop(protocol);
                let pk = Arc::clone(&p);
                thread::spawn(move || k_task(tx, pk, s, b, c, "B"));
            }
            None => {}
        },
        None => {}
    }

    let mut protocol = p.lock().unwrap();
    let end = format!("B ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn c_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
    result1: Arc<Mutex<Result1>>,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("C starts at {}", Utc::now().time());
    println!("{}", start);
    println!("C initiated by A");
    protocol.push(start);
    protocol.push(String::from("C initiated by A"));
    drop(protocol);

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    thread::sleep(Duration::from_millis(3 * DELAY));

    let result = format!("C result: {}", s);
    let mut protocol = p.lock().unwrap();
    println!("{}", result);
    protocol.push(result);
    drop(protocol);

    let mut result = result1.lock().unwrap();
    result.f2 = Some(s);

    match result.f1 {
        Some(a) => match result.f7 {
            Some(c) => {
                let mut protocol = p.lock().unwrap();
                let initiates = String::from("C initiates K");
                println!("{}", initiates);
                protocol.push(initiates);
                drop(protocol);
                let pk = Arc::clone(&p);
                thread::spawn(move || k_task(tx, pk, a, s, c, "C"));
            }
            None => {}
        },
        None => {}
    }

    let mut protocol = p.lock().unwrap();
    let end = format!("C ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn d_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
    result1: Arc<Mutex<Result1>>,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("D starts at {}", Utc::now().time());
    println!("{}", start);
    println!("D initiated by A");
    protocol.push(start);
    protocol.push(String::from("D initiated by A"));
    drop(protocol);

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    thread::sleep(Duration::from_millis(DELAY));

    let result = format!("D result: {}", s);
    let initiates = String::from("D initiates E, F, G");
    let mut protocol = p.lock().unwrap();
    println!("{}", result);
    protocol.push(result);
    println!("{}", initiates);
    protocol.push(initiates);
    drop(protocol);

    let result2 = Arc::new(Mutex::new(Result2 {
        f4: Option::<i32>::None,
        f5: Option::<i32>::None,
        f6: Option::<i32>::None,
    }));

    let txe = mpsc::Sender::clone(&tx);
    let pe = Arc::clone(&p);
    let result2e = Arc::clone(&result2);
    let result1e = Arc::clone(&result1);
    thread::spawn(move || e_task(txe, pe, s, result2e, result1e));
    let txf = mpsc::Sender::clone(&tx);
    let pf = Arc::clone(&p);
    let result2f = Arc::clone(&result2);
    let result1f = Arc::clone(&result1);
    thread::spawn(move || f_task(txf, pf, s, result2f, result1f));
    let pg = Arc::clone(&p);
    thread::spawn(move || g_task(tx, pg, s, result2, result1));

    let mut protocol = p.lock().unwrap();
    let end = format!("D ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn e_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("E starts at {}", Utc::now().time());
    println!("{}", start);
    println!("E initiated by D");
    protocol.push(start);
    protocol.push(String::from("E initiated by D"));
    drop(protocol);

    let mut result = result2.lock().unwrap();
    let r = n * 4;
    result.f4 = Some(r);

    thread::sleep(Duration::from_millis(DELAY));

    let res = format!("E result: {}", r);
    let mut protocol = p.lock().unwrap();
    println!("{}", res);
    protocol.push(res);
    drop(protocol);

    match result.f5 {
        Some(b) => match result.f6 {
            Some(c) => {
                let mut protocol = p.lock().unwrap();
                let initiates = String::from("E initiates H");
                println!("{}", initiates);
                protocol.push(initiates);
                drop(protocol);
                let ph = Arc::clone(&p);
                thread::spawn(move || h_task(tx, ph, r, b, c, result1, "E"));
            }
            None => {}
        },
        None => {}
    }

    let mut protocol = p.lock().unwrap();
    let end = format!("E ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn f_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("F starts at {}", Utc::now().time());
    println!("{}", start);
    println!("F initiated by D");
    protocol.push(start);
    protocol.push(String::from("F initiated by D"));
    drop(protocol);

    let mut result = result2.lock().unwrap();
    let r = n * 5;
    result.f5 = Some(r);

    thread::sleep(Duration::from_millis(DELAY));

    let res = format!("F result: {}", r);
    let mut protocol = p.lock().unwrap();
    println!("{}", res);
    protocol.push(res);
    drop(protocol);

    match result.f4 {
        Some(a) => match result.f6 {
            Some(c) => {
                let mut protocol = p.lock().unwrap();
                let initiates = String::from("F initiates H");
                println!("{}", initiates);
                protocol.push(initiates);
                drop(protocol);
                let ph = Arc::clone(&p);
                thread::spawn(move || h_task(tx, ph, a, r, c, result1, "F"));
            }
            None => {}
        },
        None => {}
    }

    let mut protocol = p.lock().unwrap();
    let end = format!("F ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn g_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("G starts at {}", Utc::now().time());
    println!("{}", start);
    println!("G initiated by D");
    protocol.push(start);
    protocol.push(String::from("G initiated by D"));
    drop(protocol);

    let mut result = result2.lock().unwrap();
    let r = n * 6;
    result.f6 = Some(r);

    thread::sleep(Duration::from_millis(DELAY));

    let res = format!("G result: {}", r);
    let mut protocol = p.lock().unwrap();
    println!("{}", res);
    protocol.push(res);
    drop(protocol);

    match result.f4 {
        Some(a) => match result.f5 {
            Some(b) => {
                let mut protocol = p.lock().unwrap();
                let initiates = String::from("G initiates H");
                println!("{}", initiates);
                protocol.push(initiates);
                drop(protocol);
                let ph = Arc::clone(&p);
                thread::spawn(move || h_task(tx, ph, a, b, r, result1, "G"));
            }
            None => {}
        },
        None => {}
    }

    let mut protocol = p.lock().unwrap();
    let end = format!("G ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn h_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n1: i32,
    n2: i32,
    n3: i32,
    result1: Arc<Mutex<Result1>>,
    parent: &str,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("H starts at {}", Utc::now().time());
    println!("{}", start);
    println!("H initiated by {}", parent);
    protocol.push(start);
    protocol.push(format!("H initiated by {}", parent));
    drop(protocol);

    let mut result = result1.lock().unwrap();
    let r = n1 + n2 + n3;
    result.f7 = Some(r);

    thread::sleep(Duration::from_millis(DELAY));

    let res = format!("H result: {}", r);
    let mut protocol = p.lock().unwrap();
    println!("{}", res);
    protocol.push(res);
    drop(protocol);

    match result.f1 {
        Some(a) => match result.f2 {
            Some(b) => {
                let mut protocol = p.lock().unwrap();
                let initiates = String::from("H initiates K");
                println!("{}", initiates);
                protocol.push(initiates);
                drop(protocol);
                let pk = Arc::clone(&p);
                thread::spawn(move || k_task(tx, pk, a, b, r, "H"));
            }
            None => {}
        },
        None => {}
    }

    let mut protocol = p.lock().unwrap();
    let end = format!("H ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);
}

fn k_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n1: i32,
    n2: i32,
    n3: i32,
    parent: &str,
) {
    let mut protocol = p.lock().unwrap();
    let start = format!("K starts at {}", Utc::now().time());
    println!("{}", start);
    println!("K initiated by {}", parent);
    protocol.push(start);
    protocol.push(format!("K initiated by {}", parent));
    drop(protocol);

    let n = n1 + n2 + n3;

    thread::sleep(Duration::from_millis(DELAY));

    let result = format!("H result: {}", n);
    let mut protocol = p.lock().unwrap();
    println!("{}", result);
    protocol.push(result);
    drop(protocol);

    let mut protocol = p.lock().unwrap();
    let end = format!("K ends at {}", Utc::now().time());
    println!("{}", end);
    protocol.push(end);

    tx.send(n).unwrap();
}
