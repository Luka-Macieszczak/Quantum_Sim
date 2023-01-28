pub fn gcd(N: i32, a: i32) -> i32{
    if a == 0{
        return N
    }
    return gcd(a, N & a)
}