fn is_s1(x: i128) -> bool {
    modulo_euclid(x, 3) == 1
}

fn is_s2(x: i128) -> bool {
    modulo_euclid(x, 3) == 0
}

fn is_s3(x: i128) -> bool {
    modulo_euclid(x, 3) == 2
}

// fn f(x: i128, a: i128, b: i128, alfa: i128, beta: i128, n: i128) -> (u128, u128, u128) {
fn f(x_: u128, a_: u128, b_: u128, alfa_: u128, beta_: u128, n_: u128) -> (u128, u128, u128) {
    let (x, a, b, alfa, beta, n) = (
        x_ as i128,
        a_ as i128,
        b_ as i128,
        alfa_ as i128,
        beta_ as i128,
        n_ as i128,
    );

    let f: (i128, i128, i128);
    if is_s1(x) {
        // f = (modulo_euclid(beta * x, n), a, modulo_euclid(b + 1, n));
        f = (beta * x, a, modulo_euclid(b + 1, n));
    } else if is_s2(x) {
        println!("x = {x}, a = {a}, b = {b}");
        f = (x * x, modulo_euclid(2 * a, n), modulo_euclid(2 * b, n));
    } else {
        f = (alfa * x, modulo_euclid(a + 1, n), b);
    }
    // let res = f as (u128, u128, u128);
    let (k, l, o) = f;
    (k as u128, l as u128, o as u128)
    // res
    // f //(0, 0, 0)
}

// fn f(x: u128, a: u128, b: u128, alfa: u128, beta: u128, n: u128) -> (u128, u128, u128) {
//     let f: (u128, u128, u128);
//     if is_s1(x) {
//         f = (
//             modulo_euclid((beta * x) as i128, n as i128) as u128,
//             a as u128,
//             modulo_euclid((b + 1) as i128, n as i128) as u128,
//         );
//     } else if is_s2(x) {
//         f = (
//             modulo_euclid((x * x) as i128, n as i128) as u128,
//             modulo_euclid((2 * a) as i128, n as i128) as u128,
//             modulo_euclid((2 * b) as i128, n as i128) as u128,
//         );
//     } else {
//         f = (
//             modulo_euclid((alfa * x) as i128, n as i128) as u128,
//             modulo_euclid((a + 1) as i128, n as i128) as u128,
//             b as u128,
//         );
//     }
//     f //(0, 0, 0)
// }

fn rho_pollard(p: u128, n: u128, alfa: u128, beta: u128) -> Option<(u128, u128)> {
    let (mut x, mut a, mut b) = (1u128, 0u128, 0u128);
    let (mut xprim, mut aprim, mut bprim) = f(x, a, b, alfa, beta, n);
    let mut i_counter = 0;
    while x != xprim {
        println!("i = {i_counter}");
        (x, a, b) = f(x, a, b, alfa, beta, n);
        (xprim, aprim, bprim) = f(xprim, aprim, bprim, alfa, beta, p);
        (xprim, aprim, bprim) = f(xprim, aprim, bprim, alfa, beta, p);
        i_counter += 1;
    }
    if NWD(bprim - b, n) != 1 {
        return None;
    }
    let odwrotnosc_b = odwrotnosc_multiplikatywna(b as i128, p as i128) as u128;
    let value = (a as i128 - aprim as i128) * (bprim as i128 - odwrotnosc_b as i128);
    let res = Some((i_counter, modulo_euclid(value as i128, p as i128) as u128));
    return res;

    // return Some(modulo_euclid(
    //     ((a - aprim)
    //         * (bprim as u128 -  as u128) as i128),
    //     n as i128,
    // ));
    // return Some(1);
}

fn main() {
    // let p = 458009;
    // let alfa = 2;
    // let n = 57251;
    // let beta = 56851;

    let p = 809;
    let alfa = 89;
    let n = 101;
    let beta = 618;

    let res = rho_pollard(p, n, alfa, beta);
    println!("rho pollard res = {:?}", res);

    println!("Hello, world!");
}

fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res = j % k;
    if res < 0 {
        return res + k;
    } else {
        return res;
    }
}

fn NWD(j: u128, k: u128) -> u128 {
    if j == 0 {
        return k;
    }
    let r = k % j;
    return NWD(r, j);
}

fn rozNWD(j: i128, k: i128) -> (i128, i128, i128) {
    if j == 0 {
        return (k, 0, 1);
    }
    // let r = k % j;
    let r = modulo_euclid(k, j);
    // let r = k % j;
    let (d, x_prim, y_prim) = rozNWD(r, j);
    let x = y_prim - (k / j) * x_prim;
    let y = x_prim;
    return (d, x, y);

    // (k, 1, 0)
}

fn odwrotnosc_multiplikatywna(j: i128, k: i128) -> i128 {
    // a to jest 17 czyli j
    // n = 101 czyli k
    let (_d, x, _y) = rozNWD(j, k);
    // println!("d = {d}, x = {x}, y = {y}");
    return modulo_euclid(x, k);
}
