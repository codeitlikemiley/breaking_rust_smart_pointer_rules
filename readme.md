# RC<T> in Rust

## Rc<T> when to use?

> It's used to keep track of the number of references to a value, which allows you to have multiple owners of data.

## Memory Usage Breakdown

1. Data Pointer: The size of the pointer itself, typically **8 bytes** on a `64-bit system`.

2. Reference Count: An integer value keeping track of how many Rc<T> pointers are sharing the data. This is usually a usize, so it would be **8 bytes** on a `64-bit system`.

3. Weak Count: An integer value keeping track of the weak references to this data. This is also a usize, another **8 bytes** on a `64-bit system`.

4. Data of Type T: The actual data being stored, the size of which will **depend on the type T**.

example:

Rc<i32> on 64-bit system

- datapointer = 8
- reference count = 8
- weak count = 8
---------------------------
would take up **24 bytes** for the control block

plus **4 bytes** for the i32,

---------------------------
totaling **28 bytes**

## Use Cases

1. Sharing Immutable Data Between Multiple Parts: When you want to `share read-only data` between different parts of your program without copying it, Rc<T> can be a good choice.

```rust
use std::rc::Rc;

struct MyStruct {
    value: i32,
}

fn main() {
    let a = Rc::new(MyStruct { value: 10 });
    let b = a.clone();
    let c = a.clone();

    println!("Value of a: {}", a.value); // 10
    println!("Value of b: {}", b.value); // 10
    println!("Value of c: {}", c.value); // 10
}
```

2. Avoiding Deep Copy: If `cloning the data would be expensive` in terms of performance, you can use Rc<T> to share the data without deep copying it.

3. Cyclic or Graph-Like Data Structures: Rc<T> can be used to create data structures that have cycles, as long as the `data within the structure is immutable.`

example:
```sh
   n3
  /  \
 n1  n2
```

example:
```rust
use std::rc::Rc;

struct Node {
    val: i32,
    points_to: Option<Rc<Node>>,
}

fn main() {
    let n3 = Rc::new(Node {
        val: 3,
        points_to: None,
    });

    dbg!(Rc::strong_count(&n3));

    let n2 = Node {
        val: 2,
        points_to: Some(Rc::clone(&n3)),
    };

    dbg!(Rc::strong_count(&n3));
    {
        let n1 = Node {
            val: 1,
            points_to: Some(Rc::clone(&n3)),
        };
        dbg!(Rc::strong_count(&n3));
    }

    dbg!(Rc::strong_count(&n3));
}
```

## Common Mistakes on using Rc<T>

1.Mutating Data Inside Rc<T>:

> Rc<T> assumes that the data it's wrapping is **immutable**.

If you need to mutate the data, you should use Rc<RefCell<T>>.

Solution: Use RefCell<T> inside Rc<T> to `achieve` **interior mutability** :

```rs
use std::rc::Rc;
use std::cell::RefCell;

let value = Rc::new(RefCell::new(5));
*value.borrow_mut() += 10;
```

2. **Creating Reference Cycles** : If you create a reference cycle with Rc<T>, it can lead to `memory leaks`.

Solution: Be careful with the structure of your data, and consider using weak references if cycles are possible.

## About Lifetimes in Rc<T>

In the context of Rc<T>, lifetimes are typically handled by the reference counting mechanism, and you usually don't need to annotate them explicitly. The reference counting ensures that the value will not be deallocated as long as there are references to it.
