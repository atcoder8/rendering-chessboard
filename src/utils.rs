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

/// ベクトルのノルムを計算します。
pub fn vector_norm<I: IntoIterator<Item = f64>>(v: I) -> f64 {
    v.into_iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
}

/// 与えられたベクトルを正規化して返します。
/// 零ベクトルが与えられた場合は零ベクトルを返します。
pub fn vector_normalized(v: [f64; 3]) -> [f64; 3] {
    let norm = vector_norm(v);
    if norm > 0.0 {
        v.map(|elem| elem / norm)
    } else {
        [0.0; 3]
    }
}

/// ベクトルの和を計算します。
pub fn vector_add<const N: usize>(lhs: [f64; N], rhs: [f64; N]) -> [f64; N] {
    std::array::from_fn(|i| lhs[i] + rhs[i])
}

/// ベクトルの差を計算します。
pub fn vector_sub<const N: usize>(lhs: [f64; N], rhs: [f64; N]) -> [f64; N] {
    std::array::from_fn(|i| lhs[i] - rhs[i])
}

/// ベクトルを定数倍します。
pub fn vector_scale<const N: usize>(v: [f64; N], scaler: f64) -> [f64; N] {
    std::array::from_fn(|i| v[i] * scaler)
}

/// ベクトルの平均を計算します。
pub fn vector_average<I, const N: usize>(vectors: I) -> [f64; N]
where
    I: IntoIterator<Item = [f64; N]>,
{
    let mut cnt = 0_usize;
    let sum = vectors.into_iter().fold([0.0; N], |acc, v| {
        cnt += 1;
        vector_add(acc, v)
    });
    vector_scale(sum, 1.0 / cnt as f64)
}

/// ベクトルのprefixを返します。
// pub fn vector_truncated<const DIM: usize, const IM: usize>(v: [f64; DIM]) -> [f64; IM] {
//     std::array::from_fn(|axis| v[axis])
// }

// /// ベクトルの長さを変更します。
// pub fn vector_resize<const DIM: usize, const IM: usize>(v: [f64; DIM], value: f64) -> [f64; IM] {
//     let mut resized = [value; IM];
//     resized[..DIM.min(IM)].copy_from_slice(&v);
//     resized
// }

/// ベクトルを重み付きで合成します。
pub fn vector_composited<const DIM: usize>(a: [f64; DIM], b: [f64; DIM], ratio: f64) -> [f64; DIM] {
    std::array::from_fn(|axis| (1.0 - ratio) * a[axis] + ratio * b[axis])
}

/// 平面上の1点を空間上で回転させます。
pub fn revolve_point(point: [f64; 2], azimuth: f64) -> [f64; 3] {
    let [x, y] = point;
    let (sin, cos) = azimuth.sin_cos();
    [x * cos, y, x * sin]
}
