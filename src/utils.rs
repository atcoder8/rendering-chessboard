/// ガンマ補正を行います。
pub fn gamma_correction(value: f64) -> f64 {
    value.powf(1.0 / 2.2)
}

/// 2つのベクトルの外積を計算します。
pub fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    let [ax, ay, az] = a;
    let [bx, by, bz] = b;
    [ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx]
}

/// 与えられたベクトルを正規化して返します。
/// 零ベクトルが与えられた場合は零ベクトルを返します。
pub fn vector_normalized(v: [f64; 3]) -> [f64; 3] {
    let [x, y, z] = v;
    let norm = (x.powi(2) + y.powi(2) + z.powi(2)).powf(0.5);
    if norm > 0.0 {
        v.map(|elem| elem / norm)
    } else {
        [0.0; 3]
    }
}

/// ベクトルの差を計算します。
pub fn vector_sub<const N: usize>(lhs: [f64; N], rhs: [f64; N]) -> [f64; N] {
    std::array::from_fn(|i| lhs[i] - rhs[i])
}

/// ベクトルのprefixを返します。
pub fn vector_truncated<const DIM: usize, const IM: usize>(v: [f64; DIM]) -> [f64; IM] {
    std::array::from_fn(|axis| v[axis])
}

/// ベクトルの長さを変更します。
pub fn vector_resize<const DIM: usize, const IM: usize>(v: [f64; DIM], value: f64) -> [f64; IM] {
    let mut resized = [value; IM];
    resized[..DIM.min(IM)].copy_from_slice(&v);
    resized
}

/// ベクトルを重み付きで合成します。
pub fn vector_composited<const DIM: usize>(a: [f64; DIM], b: [f64; DIM], ratio: f64) -> [f64; DIM] {
    std::array::from_fn(|axis| (1.0 - ratio) * a[axis] + ratio * b[axis])
}
