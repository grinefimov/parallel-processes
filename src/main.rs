use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
extern crate chrono;
use chrono::Utc;

type Amoi = Arc<Mutex<Option<i32>>>;

const DELAY: u64 = 0;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    thread::spawn(|| a_task(tx));

    println!("Result: {}", rx.iter().next().unwrap());
}

fn a_task(tx: mpsc::Sender<i32>) {
    println!("A started at {}", Utc::now().time());

    thread::sleep(Duration::from_millis(DELAY));

    let m1 = [1, 2, 3];
    let m2 = [4, 5, 6];
    let m3 = [7, 8, 8];

    let f1 = Arc::new(Mutex::new(Option::<i32>::None));
    let f2 = Arc::new(Mutex::new(Option::<i32>::None));
    let f7 = Arc::new(Mutex::new(Option::<i32>::None));

    let f1b = Arc::clone(&f1);
    let f2b = Arc::clone(&f2);
    let f7b = Arc::clone(&f7);
    let txb = mpsc::Sender::clone(&tx);
    thread::spawn(move || b_task(txb, m1, m2, m3, f1b, f2b, f7b));
    let f1c = Arc::clone(&f1);
    let f2c = Arc::clone(&f2);
    let f7c = Arc::clone(&f7);
    let txc = mpsc::Sender::clone(&tx);
    thread::spawn(move || c_task(txc, m1, m2, m3, f1c, f2c, f7c));
    thread::spawn(move || d_task(tx, m1, m2, m3, f1, f2, f7));

    println!("A ends at {}", Utc::now().time());
}

fn b_task(
    tx: mpsc::Sender<i32>,
    m1: [i32; 3],
    m2: [i32; 3],
    m3: [i32; 3],
    f1: Amoi,
    f2: Amoi,
    f7: Amoi,
) {
    println!("B started at {}", Utc::now().time());
    println!("B activated by Taks A");

    thread::sleep(Duration::from_millis(3 * DELAY));

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    let mut result = f1.lock().unwrap();
    *result = Some(s);

    match *(f2.lock().unwrap()) {
        Some(b) => match *(f7.lock().unwrap()) {
            Some(c) => {
                let a = (*result).unwrap();
                thread::spawn(move || k_task(tx, a, b, c, "B"));
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
    f1: Amoi,
    f2: Amoi,
    f7: Amoi,
) {
    println!("C started at {}", Utc::now().time());
    println!("C activated by Taks A");

    thread::sleep(Duration::from_millis(3 * DELAY));

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    let mut result = f2.lock().unwrap();
    *result = Some(s);

    match *(f1.lock().unwrap()) {
        Some(a) => match *(f7.lock().unwrap()) {
            Some(c) => {
                let b = (*result).unwrap();
                thread::spawn(move || k_task(tx, a, b, c, "C"));
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
    f1: Amoi,
    f2: Amoi,
    f7: Amoi,
) {
    println!("D started at {}", Utc::now().time());
    println!("D activated by Taks A");

    thread::sleep(Duration::from_millis(DELAY));

    let s = m1.iter().sum::<i32>() + m2.iter().sum::<i32>() + m3.iter().sum::<i32>();

    let f4 = Arc::new(Mutex::new(Option::<i32>::None));
    let f5 = Arc::new(Mutex::new(Option::<i32>::None));
    let f6 = Arc::new(Mutex::new(Option::<i32>::None));

    let f4e = Arc::clone(&f4);
    let f5e = Arc::clone(&f5);
    let f6e = Arc::clone(&f6);
    let f1e = Arc::clone(&f1);
    let f2e = Arc::clone(&f2);
    let f7e = Arc::clone(&f7);
    let txe = mpsc::Sender::clone(&tx);
    thread::spawn(move || e_task(txe, s, f4e, f5e, f6e, f1e, f2e, f7e));
    let f4f = Arc::clone(&f4);
    let f5f = Arc::clone(&f5);
    let f6f = Arc::clone(&f6);
    let f1f = Arc::clone(&f1);
    let f2f = Arc::clone(&f2);
    let f7f = Arc::clone(&f7);
    let txf = mpsc::Sender::clone(&tx);
    thread::spawn(move || f_task(txf, s, f4f, f5f, f6f, f1f, f2f, f7f));
    thread::spawn(move || g_task(tx, s, f4, f5, f6, f1, f2, f7));

    println!("D ends at {}", Utc::now().time());
}

fn e_task(
    tx: mpsc::Sender<i32>,
    n: i32,
    f4: Amoi,
    f5: Amoi,
    f6: Amoi,
    f1: Amoi,
    f2: Amoi,
    f7: Amoi,
) {
    println!("E started at {}", Utc::now().time());
    println!("E activated by Taks D");

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = f4.lock().unwrap();
    *result = Some(n * 4);

    match *(f5.lock().unwrap()) {
        Some(b) => match *(f6.lock().unwrap()) {
            Some(c) => {
                let a = (*result).unwrap();
                thread::spawn(move || h_task(tx, a, b, c, f1, f2, f7, "E"));
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
    f4: Amoi,
    f5: Amoi,
    f6: Amoi,
    f1: Amoi,
    f2: Amoi,
    f7: Amoi,
) {
    println!("F started at {}", Utc::now().time());
    println!("F activated by Taks D");

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = f5.lock().unwrap();
    *result = Some(n * 5);

    match *(f4.lock().unwrap()) {
        Some(a) => match *(f6.lock().unwrap()) {
            Some(c) => {
                let b = (*result).unwrap();
                thread::spawn(move || h_task(tx, a, b, c, f1, f2, f7, "F"));
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
    f4: Amoi,
    f5: Amoi,
    f6: Amoi,
    f1: Amoi,
    f2: Amoi,
    f7: Amoi,
) {
    println!("G started at {}", Utc::now().time());
    println!("G activated by Taks D");

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = f6.lock().unwrap();
    *result = Some(n * 6);

    match *(f4.lock().unwrap()) {
        Some(a) => match *(f5.lock().unwrap()) {
            Some(b) => {
                let c = (*result).unwrap();
                thread::spawn(move || h_task(tx, a, b, c, f1, f2, f7, "G"));
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
    f1: Amoi,
    f2: Amoi,
    f7: Amoi,
    parent: &str,
) {
    println!("H started at {}", Utc::now().time());
    println!("H activated by Taks {}", parent);

    thread::sleep(Duration::from_millis(DELAY));

    let mut result = f7.lock().unwrap();
    *result = Some(n1 + n2 + n3);

    match *(f1.lock().unwrap()) {
        Some(a) => match *(f2.lock().unwrap()) {
            Some(b) => {
                let c = (*result).unwrap();
                thread::spawn(move || k_task(tx, a, b, c, "H"));
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
