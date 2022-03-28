use wolfram_library_link::{
    export,
    wstp::{self, Link},
};

struct Point {
    x: f64,
    y: f64,
}

#[export(wstp)]
fn create_point(link: &mut Link) {
    // Assert that no arguments were given.
    assert_eq!(link.test_head("List"), Ok(0));

    let point = Point { x: 1.0, y: 2.0 };

    point.put(link).unwrap();
}

impl Point {
    fn put(&self, link: &mut Link) -> Result<(), wstp::Error> {
        let Point { x, y } = *self;

        // Point[{x, y}]
        link.put_function("System`Point", 1)?;
        link.put_function("System`List", 2)?;
        link.put_f64(x)?;
        link.put_f64(y)?;

        Ok(())
    }
}
