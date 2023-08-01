
struct Node {
    val: i32,
    points_to: Option<Box<Node>>,
}
/// This is an of Node struct with a pointer to another Node
/// that causes a multiple ownership problem.
/// The compiler will not allow this code to compile.
///
///             n3
///            /  \
///           n1  n2
fn main() {

    let n3 = Box::new(Node {
        val: 3,
        points_to: None,
    });

    let n2 =  Node {
        val: 2,
        points_to: Some(n3),
    };

    let n1 = Node {
        val: 1,
        points_to: Some(n3),
    };
}
