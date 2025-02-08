#![allow(dead_code)]

fn comp<F, G, T, R1, R2>(f: F, g: G) -> impl Fn(T) -> R2
where
    F: Fn(T) -> R1,
    G: Fn(R1) -> R2,
{
    move |x: T| g(f(x))
}

fn identity<T>(x: T) -> T {
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comp() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        assert_eq!(7, comp(double, add_one)(3));
        assert_eq!(8, comp(add_one, double)(3));
    }

    #[test]
    fn test_identity() {
        assert_eq!(42, identity(42));
    }

    #[test]
    fn test_comp_and_identity() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let double_id = comp(double, identity);
        let add_one_id = comp(add_one, identity);
        assert_eq!(add_one(42), add_one_id(42));
        assert_eq!(double(42), double_id(42));
    }
}
