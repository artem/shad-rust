use gc::{Arena, Gc, Scan};

use std::cell::RefCell;

////////////////////////////////////////////////////////////////////////////////

#[derive(Scan)]
struct Void;

#[derive(Scan)]
struct Int {
    x: i32,
}

#[derive(Default, Scan)]
struct Node {
    next: Option<Gc<RefCell<Node>>>,
}

#[derive(Default, Scan)]
struct Vertex {
    neigh: Vec<Gc<RefCell<Vertex>>>,
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_simple() {
    let mut arena = Arena::new();
    assert_eq!(arena.allocation_count(), 0);

    let void = arena.alloc(Void);
    assert_eq!(arena.allocation_count(), 1);

    let int = arena.alloc(Int { x: 35 });
    assert_eq!(int.borrow().x, 35);
    assert_eq!(arena.allocation_count(), 2);

    arena.sweep();
    assert_eq!(arena.allocation_count(), 2);

    drop(int);
    assert_eq!(arena.allocation_count(), 2);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 1);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 1);

    drop(void);
    assert_eq!(arena.allocation_count(), 1);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 0);
}

#[test]
fn test_self_referrential() {
    let mut arena = Arena::new();
    let node = arena.alloc(RefCell::new(Node::default()));

    arena.sweep();
    assert_eq!(arena.allocation_count(), 1);

    node.borrow().borrow_mut().next = Some(node.clone());
    arena.sweep();
    assert_eq!(arena.allocation_count(), 1);

    drop(node);
    assert_eq!(arena.allocation_count(), 1);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 0);
}

#[test]
fn test_cyclic_list() {
    let mut arena = Arena::new();

    let tail = arena.alloc(RefCell::new(Node::default()));
    let mut head = tail.clone();
    for _ in 0..2 {
        head = arena.alloc(RefCell::new(Node {
            next: Some(head.clone()),
        }));
    }
    tail.borrow().borrow_mut().next = Some(head.clone());

    assert_eq!(arena.allocation_count(), 3);
    drop(head);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 3);

    drop(tail);
    assert_eq!(arena.allocation_count(), 3);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 0);
}

#[test]
fn test_cliques() {
    let mut arena = Arena::new();

    fn make_clique(arena: &mut Arena) -> Vec<Gc<RefCell<Vertex>>> {
        let verts = (0..5)
            .map(|_| arena.alloc(RefCell::new(Vertex::default())))
            .collect::<Vec<_>>();
        for vert in verts.iter() {
            for neigh in verts.iter() {
                vert.borrow().borrow_mut().neigh.push(neigh.clone());
            }
        }
        verts
    }

    let mut c1 = make_clique(&mut arena);
    assert_eq!(arena.allocation_count(), 5);
    c1.truncate(1);
    assert_eq!(arena.allocation_count(), 5);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 5);

    let c2 = make_clique(&mut arena).into_iter().skip(2).next().unwrap();
    assert_eq!(arena.allocation_count(), 10);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 10);

    drop(c1);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 5);

    drop(c2);
    arena.sweep();
    assert_eq!(arena.allocation_count(), 0);
}
