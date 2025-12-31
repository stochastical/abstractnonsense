use core::fmt;
use wasm_bindgen::prelude::*;

//:complex-struct
#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}
//:complex-struct

//:complex-impl
#[wasm_bindgen]
impl Complex {
    #[wasm_bindgen(constructor)]
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    fn imaginary(im: f64) -> Self {
        Self { re: 0.0, im }
    }

    fn conj(&self) -> Self {
        Self { re: self.re, im: -self.im }
    }

    pub fn norm(&self) -> f64 {
        self.re.hypot(self.im)
    }

    fn norm_squared(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }

    /// Compute the 'principal' square root (positive real part)
    fn sqrt(&self) -> Self {
        let r = self.norm();
        if r == 0.0 {
            return Complex::zero();
        }
        Self {
            re: ((r + self.re) / 2.0).sqrt(),
            im: self.im.signum() * ((r - self.re) / 2.0).sqrt()
        }
    }
}
//:complex-impl

//:complex-ops
impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {re: self.re + other.re, im: self.im + other.im }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {re: self.re - other.re, im: self.im - other.im }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let denominator = other.norm_squared();
        assert_ne!(denominator, 0.0, "the norm of the divisor cannot be zero for division");

        Self {
            re: (self.re * other.re + self.im * other.im) / denominator,
            im: (self.im * other.re - self.re * other.im) / denominator
        }
    }
}
//:complex-ops

//:circle-struct
#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub struct Circle {
    pub centre: Complex,
    pub radius: f64,
}

#[wasm_bindgen]
impl Circle {
    #[wasm_bindgen]
    pub fn contains(&self, z: Complex) -> bool {
        (z - self.centre).norm() <= self.radius
    }
}
//:circle-struct

fn norm(x: &[Complex]) -> f64 {
    let mut sum = 0.0;
    for elem in x {
        sum += elem.norm_squared()
    }
    sum.sqrt()
}

//:householder-reflector
fn householder_reflector(x: Vec<Complex>) -> Option<Vec<Complex>> {
    // Degenerate base case
    if norm(&x) == 0.0 {
        return None;
    }

    let mut z = x.clone();
    let rho = z[0].re.signum();
    z[0] = z[0] + Complex::real(rho * norm(&z));
    let norm_z = norm(&z);

    // Normalise z
    for elem in &mut z {
        *elem = *elem / Complex::real(norm_z);
    }
    Some(z)
}
//:householder-reflector

//:matrix-struct
#[wasm_bindgen]
#[derive(Clone)]
pub struct Matrix {
    order: usize,          // square matrices only
    entries: Vec<Complex>, // row-major indexing
}
//:matrix-struct

//:matrix-impl
#[wasm_bindgen]
impl Matrix {
    #[wasm_bindgen(constructor)]
    pub fn new(order: usize, entries: Vec<Complex>) -> Self {
        assert_eq!(entries.len(), order * order);
        Self { order, entries }
    }

    fn get(&self, row: usize, col: usize) -> Complex {
        self.entries[row * self.order + col]
    }

    fn set(&mut self, row: usize, col: usize, val: Complex) {
        self.entries[row * self.order + col] = val
    }

    fn zero(order: usize) -> Self {
        Self { order, entries: vec![Complex::zero(); order * order] }
    }

    fn eye(order: usize) -> Self {
        let mut eye = Matrix::zero(order);
        for i in 0..order {
            eye.set(i, i, Complex::real(1.0));
        }
        eye
    }

    fn scalar_mul(&self, alpha: Complex) -> Self {
        let n = self.order;
        let mut product = Matrix::zero(n);
        for idx in 0..(n*n) {
            product.entries[idx] = alpha * self.entries[idx];
        } 
        product
    }

    fn col_dot(&self, col1: usize, other: &Self, col2: usize) -> Complex {
        assert_eq!(self.order, other.order);
        let mut sum = Complex::zero();
        for i in 0..self.order {
            // Note the conjugation here for the complex inner product!
            sum = sum + self.get(i, col1).conj() * other.get(i, col2);
        }
        sum
    }

    fn col_norm(&self, col: usize) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.order {
            sum += self.get(i, col).norm_squared()
        }
        sum.sqrt()
    }
    //:matrix-impl

    //:gershgorin_circles
    #[wasm_bindgen]
    pub fn gershgorin_circles(&self) -> Vec<Circle> {
        let n = self.order;
        let mut circles = Vec::with_capacity(n);
        for i in 0..n {
            // Centres are diagonal elements
            let centre = self.get(i, i);

            // Radii are the sum of the norms of the off-diagonal elements
            let mut radius = 0.0;
            for j in 0..n {
                if i != j {
                    radius += self.get(i, j).norm();
                }
            }

            circles.push(Circle { centre, radius });
        }
        circles
    }
    //:gershgorin_circles


    /// Uses the Gram-Schmidt process to perform QR decomposition
    //:qr-decomposition
    fn qr_decomposition(&self) -> (Matrix, Matrix) {
        let n = self.order;
        let mut q = Matrix::zero(n);
        let mut r = Matrix::zero(n);

        // Iterate over the column space
        for j in 0..n {
            // Copy over the column from A to Q
            for i in 0..n {
                q.set(i, j, self.get(i, j));
            }

            // Subtract projections onto previous orthonormal vectors
            for k in 0..j {
                let proj = q.col_dot(k, self, j);
                r.set(k, j, proj);

                for i in 0..n {
                    q.set(i, j, q.get(i, j) - proj * q.get(i, k));
                }
            }

            // Normalise
            let norm = Complex::real(q.col_norm(j));
            r.set(j, j, norm);

            if norm.re != 0.0 {
                for i in 0..n {
                    q.set(i, j, q.get(i, j) / norm);
                }
            }
        }

        (q, r)
    }
    //:qr-decomposition

    //:hessenberg-reduction
    /// Reduce matrix to upper Hessenberg form using Householder reflections.
    fn hessenberg_reduction(&self) -> Matrix {
        let n = self.order;
        let mut h = self.clone();
        
        for k in 0..(n - 2) {
            // Compute the Householder reflector on column k, from row k+1
            let m = n - (k + 1);
            let mut x_k: Vec<Complex> = Vec::with_capacity(m);
            for i in (k + 1)..n {
                x_k.push(h.get(i, k));
            }
            // Continue if we can't compute a valid reflector because the column
            // is already 0 below the subdiagonal
            let Some(u_k) = householder_reflector(x_k) else { continue };

            // Left-multiply: A[k+1:n, k:n] = A[k+1:n, k:n] - 2u_k(u_k† A[k+1:n, k:n])
            for j in k..n {
                // Compute u† * A[:,j] (dot product of u with column j)
                let mut dot = Complex::zero();
                for idx in 0..m {
                    dot = dot + u_k[idx].conj() * h.get(k + 1 + idx, j);
                }
                
                // Update column j in-place: A[:,j] = A[:,j] - 2 * (u† * A[:,j]) * u
                for idx in 0..m {
                    h.set(k + 1 + idx, j,
                        h.get(k + 1 + idx, j) - Complex::real(2.0) * dot * u_k[idx]
                    );
                }
            }

            // Right-multiply: A[0:n, k+1:n] = A[0:n, k+1:n] - 2(A[0:n, k+1:n] u)u†
            for i in 0..n {
                // Compute A[i,:] * u (dot product of row i with u)
                let mut dot = Complex::zero();
                for idx in 0..m {
                    dot = dot + h.get(i, k + 1 + idx) * u_k[idx];
                }
                
                // Update row i in-place: A[i,:] = A[i,:] - 2 * (A[i,:] * u) * u†
                for idx in 0..m {
                    h.set(i, k + 1 + idx,
                        h.get(i, k + 1 + idx) - Complex::real(2.0) * dot * u_k[idx].conj()
                    );
                }
            }
        }
        h
    }
    //:hessenberg-reduction

    //:qr-algorithm
    /// Take the top left nxn block of the matrix
    fn deflate(&self, n: usize) -> Matrix {
        assert!(n <= self.order);
        let mut entries = Vec::with_capacity(n * n);
        for i in 0..n {
            for j in 0..n {
                entries.push(self.get(i, j));
            }
        }
        Matrix { order: n, entries }
    }

    #[wasm_bindgen]
    pub fn qr_algorithm(&self, max_iter: u64, tol: f64) -> Vec<Complex> {
        let mut n = self.order;
        let mut eigvals: Vec<Complex> = vec![Complex::zero(); n];
        
        // First reduce to Hessenberg form to guarantee convergence.
        // This preserves eigenvalues as it is a similarity transformation.
        let mut a_k = self.clone();
        a_k = a_k.hessenberg_reduction();
        
        for _k in 0..max_iter {
            // Check to see if we have converged, then deflate matrix by 1
            if a_k.get(n - 1, n - 2).norm() <= tol {
                n -= 1;
                eigvals[n] = a_k.get(n, n); 
                a_k = a_k.deflate(n);
            }

            // Base case: 1x1 matrix
            if n == 1 {
                eigvals[0] = a_k.get(0, 0);
                break;
            }

            // Directly compute the eigenvalues of a 2x2 matrix
            let (mu_1, mu_2) = {
                let (a, b, c, d) = (
                    a_k.get(n - 2, n - 2),
                    a_k.get(n - 2, n - 1),
                    a_k.get(n - 1, n - 2),
                    a_k.get(n - 1, n - 1)
                );
                let trace = a + d;
                let det = a * d - b * c;
                let discriminant = (trace * trace) / Complex::real(4.0) - det;

                (
                    trace / Complex::real(2.0) + discriminant.sqrt(),
                    trace / Complex::real(2.0) - discriminant.sqrt()
                )
            };

            // Use explicit formula for 2×2 case
            if n == 2 {
                eigvals[0] = mu_1;
                eigvals[1] = mu_2;
                break;
            }

            // Select shift as the eigenvalue closest to the bottom-right element
            let a_nn = a_k.get(n - 1, n - 1);
            let s_k = if (mu_1 - a_nn).norm() < (mu_2 - a_nn).norm() {
                mu_1
            } else {
                mu_2
            };

            // Shifted QR decomposition
            let s_k = Matrix::eye(n).scalar_mul(s_k);
            let (q, r) = (&a_k - &s_k).qr_decomposition();

            // Add the shift back in: A = RQ + (s_k)I
            a_k = &(&r * &q) + &s_k;
        }

        eigvals
    }
    //:qr-algorithm
}

//:matrix-ops
impl std::ops::Add for &Matrix {
    type Output = Matrix;
    fn add(self, other: &Matrix) -> Matrix {
        assert_eq!(self.order, other.order);
        let n = self.order;

        let mut sum = Matrix::zero(n);
        for idx in 0..(n*n) {
            sum.entries[idx] = self.entries[idx] + other.entries[idx];
        }
        sum
    }
}

impl std::ops::Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, other: &Matrix) -> Matrix {
        assert_eq!(self.order, other.order);
        let n = self.order;

        let mut difference = Matrix::zero(n);
        for idx in 0..(n*n) {
            difference.entries[idx] = self.entries[idx] - other.entries[idx];
        }
        difference
    }
}

impl std::ops::Mul for &Matrix {
    type Output = Matrix;
    /// A classic naive O(n^3) matrix multiplication
    fn mul(self, other: &Matrix) -> Matrix {
        assert_eq!(self.order, other.order);
        let n = self.order;
        let mut product = Matrix::zero(n);

        for i in 0..n {
            for j in 0..n {
                let mut acc = Complex::zero();
                for k in 0..n {
                    acc = acc + self.get(i, k) * other.get(k, j);
                }
                product.set(i, j, acc);
            }
        }
        product
    }
}
//:matrix-ops

// TODO: remove
impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Matrix [")?;
        for i in 0..self.order {
            write!(f, "        ")?;
            for j in 0..self.order {
                if j > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.get(i, j))?;
            }
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}{:+.4}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests;
