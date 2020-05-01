use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
extern crate chrono;
use chrono::Utc;
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

    thread::spawn(|| a_task(tx));

    println!("Result: {}", rx.iter().next().unwrap());

    // let mut option = String::new();
    // match io::stdin().read_line(&mut option) {
    //     Ok(_) => {}
    //     Err(e) => eprintln!("Read error: {}", e),
    // }
}

fn a_task(tx: mpsc::Sender<i32>) {
    println!("A started at {}", Utc::now().time());

    thread::sleep(Duration::from_millis(DELAY));

    let m1 = [1, 2, 3];
    let m2 = [4, 5, 6];
    let m3 = [7, 8, 8];

    let result1 = Arc::new(Mutex::new(Result1 {
        f1: Option::<i32>::None,
        f2: Option::<i32>::None,
        f7: Option::<i32>::None,
    }));

    let txb = mpsc::Sender::clone(&tx);
    let result1b = Arc::clone(&result1);
    thread::spawn(move || b_task(txb, m1, m2, m3, result1b));
    let txc = mpsc::Sender::clone(&tx);
    let result1c = Arc::clone(&result1);
    thread::spawn(move || c_task(txc, m1, m2, m3, result1c));
    thread::spawn(move || d_task(tx, m1, m2, m3, result1));

    println!("A ends at {}", Utc::now().time());
}

fn b_task(
    tx: mpsc::Sender<i32>,
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
    result1: Arc<Mutex<Result1>>,
) {
    println!("B started at {}", Utc::now().time());
    println!("B activated by Taks A");

    thread::sleep(Duration::from_millis(3 * DELAY));

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    let mut result = result1.lock().unwrap();
    result.f1 = Some(s);

    match result.f2 {
        Some(b) => match result.f7 {
            Some(c) => {
                thread::spawn(move || k_task(tx, s, b, c, "B"));
            }
            None => {}
        },
        None => {}
    }

    println!("B ends at {}", Utc::now().time());
}

fn c_task(
    tx: mpsc::Sender<i32>,
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
    result1: Arc<Mutex<Result1>>,
) {
    println!("C started at {}", Utc::now().time());
    println!("C activated by Taks A");

    thread::sleep(Duration::from_millis(3 * DELAY));

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    let mut result = result1.lock().unwrap();
    result.f2 = Some(s);

    match result.f1 {
        Some(a) => match result.f7 {
            Some(c) => {
                thread::spawn(move || k_task(tx, a, s, c, "C"));
            }
            None => {}
        },
        None => {}
    }

    println!("C ends at {}", Utc::now().time());
}

fn d_task(
    tx: mpsc::Sender<i32>,
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
    result1: Arc<Mutex<Result1>>,
) {
    println!("D started at {}", Utc::now().time());
    println!("D activated by Taks A");

    thread::sleep(Duration::from_millis(DELAY));

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    let result2 = Arc::new(Mutex::new(Result2 {
        f4: Option::<i32>::None,
        f5: Option::<i32>::None,
        f6: Option::<i32>::None,
    }));

    let txe = mpsc::Sender::clone(&tx);
    let result2e = Arc::clone(&result2);
    let result1e = Arc::clone(&result1);
    thread::spawn(move || e_task(txe, s, result2e, result1e));
    let txf = mpsc::Sender::clone(&tx);
    let result2f = Arc::clone(&result2);
    let result1f = Arc::clone(&result1);
    thread::spawn(move || f_task(txf, s, result2f, result1f));
    thread::spawn(move || g_task(tx, s, result2, result1));

    println!("D ends at {}", Utc::now().time());
}

fn e_task(
    tx: mpsc::Sender<i32>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    println!("E started at {}", Utc::now().time());
    println!("E activated by Taks D");

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result2.lock().unwrap();
    let r = n * 4;
    result.f4 = Some(r);

    match result.f5 {
        Some(b) => match result.f6 {
            Some(c) => {
                thread::spawn(move || h_task(tx, r, b, c, result1, "E"));
            }
            None => {}
        },
        None => {}
    }

    println!("E ends at {}", Utc::now().time());
}

fn f_task(
    tx: mpsc::Sender<i32>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    println!("F started at {}", Utc::now().time());
    println!("F activated by Taks D");

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result2.lock().unwrap();
    let r = n * 5;
    result.f5 = Some(r);

    match result.f4 {
        Some(a) => match result.f6 {
            Some(c) => {
                thread::spawn(move || h_task(tx, a, r, c, result1, "F"));
            }
            None => {}
        },
        None => {}
    }

    println!("F ends at {}", Utc::now().time());
}

fn g_task(
    tx: mpsc::Sender<i32>,
    n: i32,
    result2: Arc<Mutex<Result2>>,
    result1: Arc<Mutex<Result1>>,
) {
    println!("G started at {}", Utc::now().time());
    println!("G activated by Taks D");

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result2.lock().unwrap();
    let r = n * 6;
    result.f6 = Some(r);

    match result.f4 {
        Some(a) => match result.f5 {
            Some(b) => {
                thread::spawn(move || h_task(tx, a, b, r, result1, "G"));
            }
            None => {}
        },
        None => {}
    }

    println!("G ends at {}", Utc::now().time());
}

fn h_task(
    tx: mpsc::Sender<i32>,
    n1: i32,
    n2: i32,
    n3: i32,
    result1: Arc<Mutex<Result1>>,
    parent: &str,
) {
    println!("H started at {}", Utc::now().time());
    println!("H activated by Taks {}", parent);

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = result1.lock().unwrap();
    let r = n1 + n2 + n3;
    result.f7 = Some(r);

    match result.f1 {
        Some(a) => match result.f2 {
            Some(b) => {
                thread::spawn(move || k_task(tx, a, b, r, "H"));
            }
            None => {}
        },
        None => {}
    }

    println!("H ends at {}", Utc::now().time());
}

fn k_task(tx: mpsc::Sender<i32>, n1: i32, n2: i32, n3: i32, parent: &str) {
    println!("K started at {}", Utc::now().time());
    println!("K activated by Taks {}", parent);

    thread::sleep(Duration::from_millis(DELAY));

    let n = n1 + n2 + n3;

    println!("Task K ends at {}", Utc::now().time());
    tx.send(n).unwrap();
}
