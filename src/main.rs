fn main() {
    // タプル
    let tup = (500, 6.4, -10);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("");

    // タプルでindex指定
    let xx = tup.0;
    let yy = tup.1;
    let zz = tup.2;
    println!("The value of xx is: {}", xx);
    println!("The value of yy is: {}", yy);
    println!("The value of zz is: {}", zz);
    println!("");

    // 配列
    let months = ["January", "February", "March", "April"];
    let index = 0;
    let january = months[index];
    let february = months[index + 1];
    let march = months[index + 2];
    let april = months[index + 3];

    println!("The value of january is: {}", january);
    println!("The value of yy february: {}", february);
    println!("The value of march is: {}", march);
    println!("The value of april is: {}", april);
    println!("");

    // ループ処理
    for month in months.iter() {
        println!("Value:{}", month);
    }

    // ループ処理2
    for index in (0..4).rev() {
        println!("Mewmew!");
    }
    
}
