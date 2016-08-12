mod point;

#[cfg(test)]
mod tests {
    use point::Point;

    #[test]
    fn it_works() {
        let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
        let p = Point { x: 1.0, y: 1.0, z: 1.0 };
        let still_p = origin + p;
        assert!(still_p.x == p.x);
    }
}
