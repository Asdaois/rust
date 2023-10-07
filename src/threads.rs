use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{sync, thread};
use sync::mpsc;

pub fn run() {
  let join_handle = thread::spawn(|| {
    for i in 0..10 {
      println!("From spawned thread - {i}");
      thread::sleep(Duration::from_millis(2));
    }
  });
  for i in 0..3 {
    println!("From main thread - {i}");
    thread::sleep(Duration::from_millis(2));
  }

  join_handle.join().unwrap();

  let (tx1, rx) = mpsc::channel();
  let tx2 = tx1.clone();

  thread::spawn(move || {
    let num_vec = vec![1, 2, 3];
    for num in num_vec {
      tx1.send(num).unwrap();
      thread::sleep(Duration::from_millis(2))
    }
  });
  thread::spawn(move || {
    let num_vec = vec![4, 5, 6];
    for num in num_vec {
      tx2.send(num).unwrap();
      thread::sleep(Duration::from_millis(2))
    }
  });

  for received_val in rx {
    println!("Received: {received_val}")
  }

  let mut mu = Mutex::new(10);
  {
    let mut val = mu.lock().unwrap();
    println!("val = {:?}", mu);
    *val += 1;

    std::mem::drop(val);
    println!("val = {:?}", mu);
  }
  println!("val = {:?}", mu);

  let val = Arc::new(Mutex::new(10));
  let mut t_handles = vec![];
  println!("val = {}", *val.lock().unwrap());
  for _ in 0..5 {
    let val = val.clone();
    let h = thread::spawn(move || {
      let mut num = val.lock().unwrap();
      *num += 1;
    });
    t_handles.push(h);
  }
  for h in t_handles {
    h.join().unwrap();
  }
  println!("val = {}", *val.lock().unwrap());
}
