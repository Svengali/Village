use std::sync::{Arc};
//use std::thread;

type Transaction = Vec<Box<dyn Fn() -> () + Send + 'static>>;

trait Transactional {
    fn checkout(&self) -> i32;
    fn checkin(&mut self, value: i32);
}


struct STM {
    data: Arc<Vec<Box<dyn Transactional>>>,
}

impl STM {
    fn new() -> STM {
        STM { data: Arc::new(Vec::new()) }
    }

    fn start_transaction(&self) -> Transaction {
        vec![]
    }

    fn add_to_transaction(&self, transaction: &mut Transaction, f: Box<dyn Fn() -> () + Send + 'static>) {
        transaction.push(f);
    }

    fn commit_transaction(&self, transaction: Transaction) -> Result<(), ()> {
        /*
        let mut data = self.data.lock().unwrap();
        for f in transaction {
            f();
        }
        */
        Ok(())
    }

    fn rollback_transaction(&self, transaction: Transaction) {
        /*
        for f in transaction.into_iter().rev() {
            f();
        }
        */
    }

    fn checkout(&self, index: usize, transaction: &mut Transaction) -> Result<i32, ()> {
        /*
        let mut data = self.data.lock().unwrap();
        let value = data.get(index).ok_or(())?.checkout();
        self.add_to_transaction(transaction, Box::new(move || {
            let mut data = self.data.lock().unwrap();
            data[index] = data[index].clone();
        }));
        Ok(value)
        */
        Ok(0)
    }

    fn checkin(&self, index: usize, value: i32, transaction: &mut Transaction) -> Result<(), ()> {
        /*
        let mut data = self.data.lock().unwrap();
        let mut boxed_data = data.get_mut(index).ok_or(())?;
        boxed_data.checkin(value);
        self.add_to_transaction(transaction, Box::new(move || {
            let mut data = self.data.lock().unwrap();
            data[index] = data[index].clone();
        }));
        */
        Ok(())
    }

    fn add_data<T>(&self, data: T) -> Result<(), ()>
    where
        T: Transactional + 'static,
    {
        //let mut data = self.data.lock().unwrap();
        //data.push(Box::new(data));
        Ok(())
    }
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Transactional for Point {
    fn checkout(&self) -> i32 {
        self.x
    }

    fn checkin(&mut self, value: i32) {
        self.x = value;
    }
}

#[derive(Clone)]
struct Counter {
    count: i32,
}

impl Transactional for Counter {

    fn checkout(&self) -> i32 {
        self.count
    }

    fn checkin(&mut self, value: i32) {
        self.count = value;
    }
}

fn _main() {
    let stm = STM::new();
    stm.add_data(Point { x: 0, y: 0 }).unwrap();
    stm.add_data(Counter { count: 0 }).unwrap();
    let mut transaction = stm.start_transaction();

    /*
    let thread1 = thread::spawn(move || {
        let value = stm.checkout(0, &mut transaction)?;
        thread::sleep(std::time::Duration::from_millis(500));
        let result = if value == 0 {
            stm.checkin(0, 1, &mut transaction)
        } else {
            Err(())
        };
        match result {
            Ok(_) => stm.commit_transaction(transaction),
            Err(_) => stm.rollback_transaction(transaction),
        }
    });

    let thread2 = thread::spawn(move || {
        let value = stm.checkout(1, &mut transaction)?;
        thread::sleep(std::time::Duration::from_millis(500));
        let result = if value == 0 {
            stm.checkin(1, 1, &mut transaction)
        } else {
            Err(())
        };
        match result {
            Ok(_) => stm.commit_transaction(transaction),
            Err(_) => stm.rollback_transaction(transaction),
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    */

    let data = stm.data;
    for item in data.iter() {
        println!("{}", item.checkout());
    }
}
