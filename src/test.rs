#[cfg(test)]
pub mod tests {
    use crate::{RectU, VectorU};

    #[test]
    fn check_size() {
        let result = RectU::new(2, 2, 10, 16);
        assert_eq!(result.size.y, 16);
    }

    #[test]
    fn check_vecu_mul() {
        let result = VectorU::new(2, 3) * 2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn check_vecu_div() {
        let result = VectorU::new(5, 9) / 3;
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 3);
    }

    #[test]
    fn check_vecu_add() {
        let result = VectorU::new(5, 9) + VectorU::new(7, 11);
        assert_eq!(result.x, 12);
        assert_eq!(result.y, 20);
    }

    #[test]
    #[should_panic]
    fn check_vecu_sub_overflow() {
        let result = VectorU::new(5, 9) - VectorU::new(7, 11);
        assert_eq!(result.x, 12);
        assert_eq!(result.y, 20);
    }

    #[test]
    fn check_vecu_sub() {
        let result = VectorU::new(5, 9) - VectorU::new(4, 1);
        assert_eq!(result.x, 1);
        assert_eq!(result.y, 8);
    }
}
