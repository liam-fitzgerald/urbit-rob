
mod phonemes;

mod patq;

pub use patq::{*};

#[cfg(test)]
mod tests {
    #[test]
    fn bytes_to_patq() {
        assert_eq!(super::patq::bytes_to_patq(vec!(0)), String::from("~zod"));
        assert_eq!(super::patq::bytes_to_patq(vec!(174, 1)), String::from("~marhut"));
        assert_eq!(super::patq::bytes_to_patq(vec!(103, 0, 1)), String::from("~nec-dozmyn"));
    }
    #[test]
    fn patq_to_bytes() {
        assert_eq!(super::patq::patq_to_bytes(&mut String::from("~zod")), Some(vec!(0)));
        assert_eq!(super::patq::patq_to_bytes(&mut String::from("~marhut")), Some(vec!(174,1)));
        assert_eq!(super::patq::patq_to_bytes(&mut String::from("~nec-dozmyn")), Some(vec!(103,0,1)));
    }
}
