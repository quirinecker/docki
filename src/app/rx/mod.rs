type Subscriber<T> = fn(value: T) -> ();

pub struct Observable<T: Copy> {
    subscriber: Vec<Subscriber<T>> 
}

impl<T: Copy> Observable<T> {

    pub fn new() -> Observable<T> {
        return Observable {
            subscriber: Vec::new(),
        }
    }

    pub fn next(&self, value: T) {
        for callback in &self.subscriber  {
            callback(value);
        }
    }

    pub fn subscribe(&mut self, subscriber: Subscriber<T>) {
        self.subscriber.push(subscriber);
    }
}
