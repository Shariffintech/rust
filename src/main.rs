fn climb_stairs(n: i32) -> i32 {
    // base 
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }

    // recursive 
    let mut dp: [i32; 3] = [0, 1, 2];
    for i in 3..=n {
        dp[i as usize % 3] = dp[(i - 1) as usize % 3] + dp[(i - 2) as usize % 3];
    }
    dp[n as usize % 3]
}

fn main() {
    let n = 2;
    println!("{}", climb_stairs(n));
}