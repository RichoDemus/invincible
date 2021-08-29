use nalgebra::Vector2;
use quicksilver::geom;
use uuid::Uuid;

pub fn convert(vec: geom::Vector) -> Vector2<f64> {
    Vector2::new(vec.x.into(), vec.y.into())
}

#[cfg(test)]
pub fn uuid(i: i32) -> Uuid {
    Uuid::parse_str(format!("00000000-0000-0000-0000-0000000000{:02}", i).as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(uuid(0), Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap(), "0");
        assert_eq!(uuid(1), Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(), "1");
        assert_eq!(uuid(10), Uuid::parse_str("00000000-0000-0000-0000-000000000010").unwrap(), "10");
    }
}