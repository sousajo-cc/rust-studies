pub(crate) fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n!=0 && m!=0);
    while m!=0 {
        if m<n {
            let t = m; m = n; n = t;
        }
        m = m%n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(22,47),1);
}