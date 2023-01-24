use crate::app::rx::Observable;

#[test]
fn test_observable() {
    let mut observable: Observable<u32> = Observable::new();

    observable.subscribe(|value| {
        assert_eq!(5, value);
    });

    observable.next(5);
}

