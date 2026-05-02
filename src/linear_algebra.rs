#![allow(dead_code)]

/// ベクトルの型
pub type Vector<const N: usize> = [f64; N];

/// 行列の型
pub type Matrix<const M: usize, const N: usize> = [[f64; N]; M];

/// 正方行列の型
pub type SquareMatrix<const N: usize> = Matrix<N, N>;

/// ベクトルの内積を計算します。
pub fn inner_product<const N: usize>(a: Vector<N>, b: Vector<N>) -> f64 {
    a.into_iter().zip(b).map(|(x, y)| x * y).sum()
}

/// ベクトルの外積を計算します。
pub fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    let [ax, ay, az] = a;
    let [bx, by, bz] = b;
    [ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx]
}

/// ベクトルのノルムを計算します。
pub fn vector_norm<const N: usize>(v: Vector<N>) -> f64 {
    v.into_iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
}

/// 与えられたベクトルを正規化して返します。
/// 零ベクトルが与えられた場合は零ベクトルを返します。
pub fn vector_normalized<const N: usize>(v: Vector<N>) -> Vector<N> {
    let norm = vector_norm(v);
    if norm > 0.0 {
        v.map(|elem| elem / norm)
    } else {
        [0.0; N]
    }
}

/// ベクトルの和を計算します。
pub fn vector_add<const N: usize>(lhs: Vector<N>, rhs: Vector<N>) -> Vector<N> {
    std::array::from_fn(|i| lhs[i] + rhs[i])
}

/// ベクトルの差を計算します。
pub fn vector_sub<const N: usize>(lhs: Vector<N>, rhs: Vector<N>) -> Vector<N> {
    std::array::from_fn(|i| lhs[i] - rhs[i])
}

/// ベクトルを定数倍します。
pub fn vector_scale<const N: usize>(v: Vector<N>, scaler: f64) -> Vector<N> {
    std::array::from_fn(|i| v[i] * scaler)
}

/// ベクトルの平均を計算します。
/// ベクトルが1つも与えられなかった場合は零ベクトルを返します。
pub fn vector_average<I, const N: usize>(vectors: I) -> Vector<N>
where
    I: IntoIterator<Item = Vector<N>>,
{
    let mut cnt = 0_usize;
    let sum = vectors.into_iter().fold([0.0; N], |acc, v| {
        cnt += 1;
        vector_add(acc, v)
    });

    if cnt == 0 {
        return [0.0; N];
    }

    vector_scale(sum, 1.0 / cnt as f64)
}

/// ベクトルを重み付きで合成します。
pub fn vector_composited<const N: usize>(a: [f64; N], b: [f64; N], ratio: f64) -> [f64; N] {
    std::array::from_fn(|axis| (1.0 - ratio) * a[axis] + ratio * b[axis])
}

/// 平面上の1点を空間上で回転させます。
pub fn revolve_point(point: [f64; 2], azimuth: f64) -> [f64; 3] {
    let [x, y] = point;
    let (sin, cos) = azimuth.sin_cos();
    [x * cos, y, x * sin]
}
