use std::rc::Rc;
#[derive(Debug)]
#[allow(unused)]
struct Node {
    val: i32,
    points_to: Option<Rc<Node>>,
}


#[allow(unused)]
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

// create a test
// We need to update the value of n3
// the value of n3 is updated to 4
// we should be able to see the updated value of n3
// is 4 in all the places where it is used

// configure test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rc() {

        let n3 = Rc::new(Node {
            val: 3,
            points_to: None,
        });

        let n2 = Node {
            val: 2,
            points_to: Some(Rc::clone(&n3)),
        };

        {
            let n1 = Node {
                val: 1,
                points_to: Some(Rc::clone(&n3)),
            };
        }
        // expect we would panic

        assert_eq!(4, n3.val);
    }
}

// create a test function




