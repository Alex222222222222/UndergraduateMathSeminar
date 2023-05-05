static FACTORIAL_CACHE: once_cell::sync::Lazy<std::sync::Mutex<lru::LruCache<(u128, u128), u128>>> =
    once_cell::sync::Lazy::new(|| {
        let cache: lru::LruCache<(u128, u128), u128> =
            lru::LruCache::new(std::num::NonZeroUsize::new(1000).unwrap());

        std::sync::Mutex::new(cache)
    });

static COMB_CACHE: once_cell::sync::Lazy<std::sync::Mutex<lru::LruCache<(u128, u128), u128>>> =
    once_cell::sync::Lazy::new(|| {
        let cache: lru::LruCache<(u128, u128), u128> =
            lru::LruCache::new(std::num::NonZeroUsize::new(1000).unwrap());

        std::sync::Mutex::new(cache)
    });

fn main() {
    println!("Hello, world!");

    let n = 5;
    let k = 1;
    let m = 3;
    println!("p({}, {}, {}) = {}", n, k, m, p_m_adjacent(n, k, m));
}

fn p_m_adjacent(n: u128, k: u128, m: u128) -> f64 {
    let mut res: f64 = 0.0;

    for i in 2..(m + 1) {
        println!("res: {}", res);
        res += p_i_adjacent(n, k, i) * sign(i) as f64 * comb(m, i) as f64;
    }

    res
}

fn p_i_adjacent(n: u128, k: u128, i: u128) -> f64 {
    let mut res1: i128 = 0;
    let mut pow2 = 1;
    for j in 1..(k + 1) {
        pow2 *= 2;
        res1 += comb(pow2, i) as i128 * comb(k, j) as i128 * sign(k - j);
    }

    let nk = n.pow(k as u32);
    let res2 = factorial(nk - 1, nk - i);
    let res2 = res2 / factorial(i, 1);

    res1 as f64 / res2 as f64
}

/// Returns the number of combinations of n things taken k at a time.
fn comb(n: u128, k: u128) -> u128 {
    if k > n {
        return 0;
    }
    let k = min(k, n - k);
    if k == 0 {
        return 1;
    }

    let mut cache = COMB_CACHE.lock().unwrap();
    if let Some(res) = cache.get(&(n, k)) {
        return *res;
    }

    let res: u128 = factorial(n, n - k);
    let res1 = factorial(k, 1);

    let res = res / res1;

    cache.put((n, k), res);

    res
}

fn min(a: u128, b: u128) -> u128 {
    if a < b {
        a
    } else {
        b
    }
}

fn sign(a: u128) -> i128 {
    if a % 2 == 0 {
        1
    } else {
        -1
    }
}

/// Returns the factorial of n with a lower bound l.
///
/// $$
/// factorial(n, l) = \frac{n!}{l!}
/// $$
fn factorial(n: u128, l: u128) -> u128 {
    if l >= n {
        return 1;
    }

    let mut cache = FACTORIAL_CACHE.lock().unwrap();
    if let Some(res) = cache.get(&(n, l)) {
        return *res;
    }

    let mut res = 1;
    for i in l + 1..n + 1 {
        res *= i;
    }

    cache.put((n, l), res);
    res
}
