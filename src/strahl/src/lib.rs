#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod basis;
pub mod camera;
pub mod tracer;
pub mod vec;