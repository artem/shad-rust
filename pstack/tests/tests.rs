use pstack::PStack;

#[test]
fn test_simple() {
    let mut stack = PStack::new();
    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());

    for i in 0..10 {
        stack = stack.push(i);
        assert_eq!(stack.len(), i + 1);
    }

    for i in (0..10).rev() {
        let (last, stack_new) = stack.pop().unwrap();
        assert_eq!(stack_new.len(), i);
        assert_eq!(*last, i);
        stack = stack_new;
    }
}

#[test]
fn test_persistence() {
    let mut stacks = vec![PStack::new()];
    for i in 0..100 {
        let st = stacks.last_mut().unwrap().push(i);
        stacks.push(st);
    }

    for i in (0..100).rev() {
        let (top, tail) = stacks.last().unwrap().pop().unwrap();
        assert_eq!(*top, i);
        stacks.push(tail);
    }

    for i in 0..100 {
        let stack = stacks[i].clone();
        assert_eq!(stack.len(), i);

        for (item, i) in stack.iter().zip((0..i).rev()) {
            assert_eq!(i, *item);
        }
    }

    for i in 100..201 {
        let stack = stacks[i].clone();
        assert_eq!(stack.len(), 200 - i);

        for (item, i) in stack.iter().zip((0..200 - i).rev()) {
            assert_eq!(i, *item);
        }
    }
}

#[test]
fn test_no_clone() {
    struct Int(i32);

    let mut stack = PStack::new();
    for i in 0..100 {
        stack = stack.push(Int(i));
    }

    for i in (0..100).rev() {
        let (top, tail) = stack.pop().unwrap();
        assert_eq!(top.0, i);
        stack = tail;
    }
}

#[test]
fn test_iter_simple() {
    let mut stack = PStack::new();
    for i in 0..100 {
        stack = stack.push(i);
    }

    for (i, value) in stack.iter().enumerate() {
        assert_eq!((100 - i - 1) as i32, *value);
    }
}

#[test]
fn test_iter_parallel() {
    let mut stack = PStack::new();
    for i in 0..100 {
        stack = stack.push(i);
    }
    let mut iter_one = stack.iter();

    for i in 100..200 {
        stack = stack.push(i);
    }
    let mut iter_two = stack.iter();

    for i in 0..200 {
        if i < 100 {
            assert_eq!(iter_one.next().as_deref().copied(), Some(100 - i - 1));
        }
        assert_eq!(iter_two.next().as_deref().copied(), Some(200 - i - 1));
    }
}
