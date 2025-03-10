use std::sync::{Arc, Mutex};

struct Counter {
    count: u32,
}





#[tokio::main]
async fn main() {
    let counter = Counter { count: 0 };

    let arc = Arc::new(Mutex::new(counter));

    for _ in 0..100{
        tokio::spawn(count(arc.clone())).await.unwrap();
    }


    println!("final count : {:?}", arc.lock().unwrap().count);


}

async fn count(c: Arc<Mutex<Counter>>) {
    let mut counter = c.lock().unwrap();
    counter.count += 1;
    println!("count: {:?}", counter.count);
}
