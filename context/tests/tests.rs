use context::Context;

////////////////////////////////////////////////////////////////////////////////

trait SayHi {
    fn say_hi(&self) -> &str;
}

struct Greeter {}

impl SayHi for Greeter {
    fn say_hi(&self) -> &str {
        "hi!"
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_singletones() {
    let mut cx = Context::new();

    cx.insert_singletone(64i64);
    cx.insert_singletone(32i32);
    assert_eq!(*cx.get_singletone::<i64>(), 64);
    assert_eq!(*cx.get_singletone::<i32>(), 32);

    cx.insert_singletone(Box::new(Greeter {}) as Box<dyn SayHi>);
    assert_eq!(cx.get_singletone::<Box<dyn SayHi>>().say_hi(), "hi!");

    cx.insert_singletone::<Box<[u8]>>(Box::new(*b"binary data"));
    assert_eq!(
        cx.get_singletone::<Box<[u8]>>() as &[u8],
        b"binary data" as &[u8]
    );

    cx.insert_singletone("hello, world!");
    assert_eq!(*cx.get_singletone::<&'static str>(), "hello, world!");
    cx.insert_singletone("foo bar");
    assert_eq!(*cx.get_singletone::<&'static str>(), "foo bar");
}

#[test]
fn test_key() {
    let mut cx = Context::new();

    cx.insert("x", 128i32);
    cx.insert("y", 255i32);
    assert_eq!(*cx.get::<i32>("x"), 128);
    assert_eq!(*cx.get::<i32>("y"), 255);

    cx.insert_singletone(372i32);
    assert_eq!(*cx.get_singletone::<i32>(), 372);

    cx.insert("z", 100i32);
    assert_eq!(*cx.get::<i32>("z"), 100);
    assert_eq!(*cx.get::<i32>("x"), 128);
    assert_eq!(*cx.get::<i32>("y"), 255);

    cx.insert("my_str", "my favourite str");
    assert_eq!(*cx.get::<&'static str>("my_str"), "my favourite str");

    assert_eq!(*cx.get_singletone::<i32>(), 372);

    let key = "foo".to_string();
    cx.insert(key.clone(), true);
    assert_eq!(*cx.get::<bool>(&key), true);
}

#[test]
#[should_panic]
fn test_get_missing() {
    let cx = Context::new();
    cx.get::<Greeter>("greeter");
}

#[test]
#[should_panic]
fn test_get_missing_singletone() {
    let cx = Context::new();
    cx.get_singletone::<Greeter>();
}

#[test]
#[should_panic]
fn test_wrong_type() {
    let mut cx = Context::new();
    cx.insert("greeter", Greeter {});
    cx.get::<usize>("greeter");
}
