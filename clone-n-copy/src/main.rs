#[derive(Debug, Clone, Copy)]
pub struct PointCloneAndCopy {
    pub x: f64,
}

#[derive(Debug, Clone)]
pub struct PointCloneOnly {
    pub x: f64,
}

fn main() {
    // Copy is implicit, inexpensive, and cannot be re-implemented (memcpy).
    // Clone is explicit, may be expensive, and may be re-implement arbitrarily.

    let p1 = PointCloneAndCopy { x: 0. };
    let p2 = p1; // because type has `Copy`, it gets copied automatically.
    println!("{p1:?} {p2:?}");

    let p1 = PointCloneOnly { x: 0. };
    let p2 = p1.clone(); // because type has no `Copy`, this is a move instead.
    println!("{:?} {:?}", p1, p2);
}
