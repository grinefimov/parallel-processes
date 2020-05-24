use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
extern crate chrono;

use std::fs;
use parallel_processes::protocol::*;

const DELAY: u64 = 0;

#[derive(Copy, Clone)]
struct Ms {
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
}

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
}

fn a_task(tx: mpsc::Sender<i32>, p: Arc<Mutex<Vec<String>>>) {
    protocol_start("A", "main", Arc::clone(&p));

    let ms = Ms {
        m1: [1, 2, 3],
        m2: [4, 5, 6],
        m3: [7, 8, 8],
    };

    thread::sleep(Duration::from_millis(DELAY));

    let result = format!("M1: {:?}, M2: {:?}, M3: {:?}", ms.m1, ms.m2, ms.m3);
    protocol_result_and_initiates(result, "A", "B, C, D", Arc::clone(&p));

    let result1 = Arc::new(Mutex::new(Result1 {
        f1: Option::<i32>::None,
        f2: Option::<i32>::None,
        f7: Option::<i32>::None,
    }));

    let txb = tx.clone();
    let pb = Arc::clone(&p);
    let result1b = Arc::clone(&result1);
    thread::spawn(move || b_task(txb, pb, ms, result1b));
    let txc = tx.clone();
    let pc = Arc::clone(&p);
    let result1c = Arc::clone(&result1);
    thread::spawn(move || c_task(txc, pc, ms, result1c));
    let pd = Arc::clone(&p);
    thread::spawn(move || d_task(tx, pd, ms, result1));

    protocol_end("A", Arc::clone(&p));
}

fn b_task(tx: mpsc::Sender<i32>, p: Arc<Mutex<Vec<String>>>, ms: Ms, result1: Arc<Mutex<Result1>>) {
    protocol_start("B", "A", Arc::clone(&p));

    let s = ms.m1.iter().sum::<i32>() + ms.m2.iter().sum::<i32>() + ms.m3.iter().sum::<i32>();

    thread::sleep(Duration::from_millis(3 * DELAY));

    let mut result = result1.lock().unwrap();
    result.f1 = Some(s);

    protocol_result(s, "B", Arc::clone(&p));

    match result.f2 {
        Some(b) => match result.f7 {
            Some(c) => {
                protocol_initiates("B", "K", Arc::clone(&p));
                let pk = Arc::clone(&p);
                thread::spawn(move || k_task(tx, pk, s, b, c, "B"));
            }
            None => {}
        },
        None => {}
    }

    protocol_end("B", Arc::clone(&p));
}

fn c_task(tx: mpsc::Sender<i32>, p: Arc<Mutex<Vec<String>>>, ms: Ms, result1: Arc<Mutex<Result1>>) {
    protocol_start("C", "A", Arc::clone(&p));

    let s = ms.m1.iter().sum::<i32>() + ms.m2.iter().sum::<i32>() + ms.m3.iter().sum::<i32>();

    thread::sleep(Duration::from_millis(3 * DELAY));

    let mut result = result1.lock().unwrap();
    result.f2 = Some(s);

    protocol_result(s, "C", Arc::clone(&p));

    match result.f1 {
        Some(a) => match result.f7 {
            Some(c) => {
                protocol_initiates("C", "K", Arc::clone(&p));
                let pk = Arc::clone(&p);
                thread::spawn(move || k_task(tx, pk, a, s, c, "C"));
            }
            None => {}
        },
        None => {}
    }

    protocol_end("C", Arc::clone(&p));
}

fn d_task(tx: mpsc::Sender<i32>, p: Arc<Mutex<Vec<String>>>, ms: Ms, result1: Arc<Mutex<Result1>>) {
    protocol_start("D", "A", Arc::clone(&p));

    let s = ms.m1.iter().sum::<i32>() + ms.m2.iter().sum::<i32>() + ms.m3.iter().sum::<i32>();

    thread::sleep(Duration::from_millis(DELAY));

    let result = s.to_string();
    protocol_result_and_initiates(result, "D", "E, F, G", Arc::clone(&p));

    let result2 = Arc::new(Mutex::new(Result2 {
        f4: Option::<i32>::None,
        f5: Option::<i32>::None,
        f6: Option::<i32>::None,
    }));

    let txe = tx.clone();
    let pe = Arc::clone(&p);
    let result2e = Arc::clone(&result2);
    let result1e = Arc::clone(&result1);
    thread::spawn(move || e_task(txe, pe, s, result2e, result1e));
    let txf = tx.clone();
    let pf = Arc::clone(&p);
    let result2f = Arc::clone(&result2);
    let result1f = Arc::clone(&result1);
    thread::spawn(move || f_task(txf, pf, s, result2f, result1f));
    let pg = Arc::clone(&p);
    thread::spawn(move || g_task(tx, pg, s, result2, result1));

    protocol_end("D", Arc::clone(&p));
}

fn e_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    protocol_start("E", "D", Arc::clone(&p));

    let r = n * 4;

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result2.lock().unwrap();
    result.f4 = Some(r);

    protocol_result(r, "E", Arc::clone(&p));

    match result.f5 {
        Some(b) => match result.f6 {
            Some(c) => {
                protocol_initiates("E", "H", Arc::clone(&p));
                let ph = Arc::clone(&p);
                thread::spawn(move || h_task(tx, ph, r, b, c, result1, "E"));
            }
            None => {}
        },
        None => {}
    }

    protocol_end("E", Arc::clone(&p));
}

fn f_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    protocol_start("F", "D", Arc::clone(&p));

    let r = n * 5;

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result2.lock().unwrap();
    result.f5 = Some(r);

    protocol_result(r, "F", Arc::clone(&p));

    match result.f4 {
        Some(a) => match result.f6 {
            Some(c) => {
                protocol_initiates("F", "H", Arc::clone(&p));
                let ph = Arc::clone(&p);
                thread::spawn(move || h_task(tx, ph, a, r, c, result1, "F"));
            }
            None => {}
        },
        None => {}
    }

    protocol_end("F", Arc::clone(&p));
}

fn g_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    protocol_start("G", "D", Arc::clone(&p));

    let r = n * 6;

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result2.lock().unwrap();
    result.f6 = Some(r);

    protocol_result(r, "G", Arc::clone(&p));

    match result.f4 {
        Some(a) => match result.f5 {
            Some(b) => {
                protocol_initiates("G", "H", Arc::clone(&p));
                let ph = Arc::clone(&p);
                thread::spawn(move || h_task(tx, ph, a, b, r, result1, "G"));
            }
            None => {}
        },
        None => {}
    }

    protocol_end("G", Arc::clone(&p));
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
    protocol_start("H", parent, Arc::clone(&p));

    let r = n1 + n2 + n3;

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result1.lock().unwrap();
    result.f7 = Some(r);

    protocol_result(r, "H", Arc::clone(&p));

    match result.f1 {
        Some(a) => match result.f2 {
            Some(b) => {
                protocol_initiates("H", "K", Arc::clone(&p));
                let pk = Arc::clone(&p);
                thread::spawn(move || k_task(tx, pk, a, b, r, "H"));
            }
            None => {}
        },
        None => {}
    }

    protocol_end("H", Arc::clone(&p));
}

fn k_task(
    tx: mpsc::Sender<i32>,
    p: Arc<Mutex<Vec<String>>>,
    n1: i32,
    n2: i32,
    n3: i32,
    parent: &str,
) {
    protocol_start("K", parent, Arc::clone(&p));

    let r = n1 + n2 + n3;

    thread::sleep(Duration::from_millis(DELAY));

    protocol_result(r, "K", Arc::clone(&p));

    protocol_end("K", Arc::clone(&p));

    tx.send(r).unwrap();
}
