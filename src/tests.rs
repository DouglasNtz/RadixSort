use super::{radix_sort, radix_sort_limit_digits};

#[test]
fn test_radix_sort() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    radix_sort(&mut v);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])

}

#[test]
fn test_radix_sort_limit_digits() {

    let mut v = vec![10,5,2,9,0,3,3,0,9,1,5,0,10,5,6];
    radix_sort_limit_digits(&mut v, 2);
    assert_eq!(v, [0,0,0,1,2,3,3,5,5,5,6,9,9,10,10])

}
