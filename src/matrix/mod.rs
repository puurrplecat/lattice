use std::{ops::{Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, Rem, RemAssign}, fmt::Display};

pub struct Equation<const H: usize, const W: usize> {
    matrix: Matrix<H, W>,
    equals_to: Matrix<H, 1>,
    error_mod: i64,
    modulus: i64,
}
impl<const H: usize, const W: usize> Equation<H, W> {
    pub fn from_slices(a: &[i64], b: &[i64], modulus: i64) -> Self {
        Equation {
            matrix: Matrix::from_slice(a),
            equals_to: Matrix::from_slice(b),
            error_mod: modulus / 10,
            modulus,
        }
    }
    pub fn from_matrices(matrix: Matrix<H, W>, equals_to: Matrix<H, 1>, modulus: i64) -> Self {
        Equation {
            matrix,
            equals_to,
            error_mod: modulus / 10,
            modulus,
        }
    }
    pub fn add_noise(&mut self) {
        let mut rng = noise_rng(11);
        (0..H).for_each(
            |x| {
                let v = rng() % self.error_mod;
                let sign = if x % 2 == 1 {1} else {-1};
                self.equals_to[x][0] += sign * v;
            } // alternates sign to subtract or add error, this ensures on average the error does not exceed the mod
        );
    }
    pub fn encrypt(&mut self, bit: i64) -> (Matrix<1, W>, i64) {
        let mut rng = index_rng(10);
        let start = rng() % (H / 2);
        let end = rng() % (H / 2) + H / 2;
        // Adds some random rows together. This makes it harder than using a row directly from the public key.
        // (Is it hard to find which rows were added together?)
        let mut row = Matrix::<1, W>::default();
        let mut equals_to = 0;
        for i in start .. end {
            row += self.matrix[i];
            equals_to += self.equals_to[i][0];
        }

        match bit {
            1 => { (row, (equals_to + self.modulus / 2) % self.modulus) }
            0 => { (row, equals_to % self.modulus)                      }
            _ => { panic!("Cannot encrypt a non binary value")          }
        }
    }
}

// This is an lfrs
fn noise_rng (mut seed: i64) -> impl FnMut() -> i64 {
    move || {
        let mut out = 0;
        for _ in 0..63 { // otherwise it'll overflow if we do up to 64 and push one more bit into the sign bit
            let newbit = (seed ^ (seed >> 1) ^ (seed >> 3) ^ (seed >> 4)) & 1;
            out <<= 1;
            out |= seed & 1;
            seed = (seed >> 1) | (newbit << 63);
        }
        out
    }
}

fn index_rng (mut seed: usize) -> impl FnMut() -> usize {
    move || {
        let mut out = 0;
        for _ in 0..64 { // otherwise it'll overflow if we do up to 64 and push one more bit into the sign bit
            let newbit = (seed ^ (seed >> 1) ^ (seed >> 3) ^ (seed >> 4)) & 1;
            out <<= 1;
            out |= seed & 1;
            seed = (seed >> 1) | (newbit << 63);
        }
        out
    }
}

impl<const H: usize, const W: usize> Display for Equation<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.matrix).unwrap();
        writeln!(f, "{}", self.equals_to).unwrap();
        write!(f, "{}", self.modulus)
    }
}

#[derive(Clone, Copy)]
pub struct Matrix <const H: usize, const W: usize> {
    matrix: [Row<W>; H],
}

impl<const H: usize, const W: usize> Matrix<H, W> {
    pub fn from_slice(v: &[i64]) -> Matrix<H, W> {
        assert!(H > 0);
        assert!(v.len() % H == 0);
        let width = v.len() / H;
        Matrix {
            matrix: v
                    .chunks(width)
                    .map(|y| Row::new(y))
                    .collect::<Vec<Row<W>>>()
                    .try_into().unwrap()
        }
    }
    pub fn map(&mut self, f: fn(&i64) -> i64) {
        self.matrix = self.matrix
                      .iter()
                      .map(|x| Row::map(*x, f))
                      .collect::<Vec<Row<W>>>()
                      .try_into().unwrap();
    }
}

impl<const H: usize, const W: usize> Default for Matrix<H, W> {
    fn default() -> Self {
        Matrix {
            matrix: (0..H)
                    .map(|_| Row::default())
                    .collect::<Vec<Row<W>>>()
                    .try_into().unwrap()
        }
    }
}

impl<const H: usize, const W: usize> Index<usize> for Matrix<H, W> {
    type Output = Row<W>;
    fn index(&self, i: usize) -> &Row<W> {
        &self.matrix[i]
    }
}

impl<const H: usize, const W: usize> IndexMut<usize> for Matrix<H, W> {
    fn index_mut(&mut self, i: usize) -> &mut Row<W> {
        &mut self.matrix[i]
    }
}

impl<const W: usize> From<Row<W>> for Matrix<1, W> {
    fn from(value: Row<W>) -> Self {
        Matrix {matrix: [value]}
    }
}

// Due to type constraits, I can force correct matrix multiplication at compile time due to the const dimensions H and W and M
impl<const H: usize, const W: usize, const M: usize> Mul<Matrix<W, M>> for Matrix<H, W> {
    type Output = Matrix<H, M>;
    fn mul(self, rhs: Matrix<W, M>) -> Matrix<H, M> {
        let mut new = Vec::with_capacity(H * M);
        for i in 0..H {
            for j in 0..M {
                let mut sum = 0;
                for k in 0..W { // note self.width == rhs.height
                    sum += self[i][k] * rhs[k][j];
                }
            new.push(sum);
            }
        } 
        Matrix::from_slice(&new)
    }
}

impl<const H: usize, const W: usize> Add for Matrix<H, W> {
    type Output = Matrix<H, W>;
    fn add(self, rhs: Matrix<H, W>) -> Matrix<H, W> {
        Matrix {
            matrix: self.matrix
                    .iter()
                    .zip(rhs.matrix)
                    .map(|(x, y)| *x + y)
                    .collect::<Vec<Row<W>>>()
                    .try_into().unwrap()
        }
    }
}

impl<const W: usize> Add<Row<W>> for Matrix<1, W> {
    type Output = Matrix<1, W>;
    fn add(self, rhs: Row<W>) -> Self::Output {
        Matrix {matrix: [self.matrix[0] + rhs]}
    }
}

impl<const H: usize, const W: usize> AddAssign for Matrix<H, W> {
    fn add_assign(&mut self, mut rhs: Matrix<H, W>) {
        let mut i = 0;
        // We know rhs is owned and dropped by this function, so mem::take won't mess things up
        self.matrix
            .iter_mut()
            .for_each(|x| {*x += std::mem::take(&mut rhs[i]); i += 1});
    }
}

impl<const W: usize> AddAssign<Row<W>> for Matrix<1, W> {
    fn add_assign(&mut self, rhs: Row<W>) {
        // Guaranteed a row at index 0 in a Matrix<1, W>
        self.matrix[0] += rhs;
    }
}

impl<const H: usize, const W: usize> Sub for Matrix<H, W> {
    type Output = Matrix<H, W>;
    fn sub(self, rhs: Matrix<H, W>) -> Matrix<H, W> {
        Matrix {
            matrix: self.matrix
                    .iter()
                    .zip(rhs.matrix)
                    .map(|(x, y)| *x - y)
                    .collect::<Vec<Row<W>>>()
                    .try_into().unwrap()
        }
    }
}

impl<const H: usize, const W: usize> SubAssign for Matrix<H, W> {
    fn sub_assign(&mut self, mut rhs: Matrix<H, W>) {
        let mut i = 0;
        self.matrix
            .iter_mut()
            .for_each(|x| {*x += std::mem::take(&mut rhs[i]); i -= 1});
    }
}

impl<const H: usize, const W: usize> Display for Matrix<H, W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if W == 1 {
            // Column vector 
            for i in 0..H - 1 {
                write!(f, "{} ", self[i][0]).unwrap();
            }
            write!(f, "{} ", self[H - 1][0])

        } else {
            for i in 0..H - 1 {
                writeln!(f, "{}", self[i]).unwrap();
            }
            write!(f, "{}", self[H - 1])
        }
    }
}

// The row struct represents a linear combination
#[derive(Clone, Copy)]
pub struct Row <const W: usize> {
    eq: [i64; W],
}

impl<const W: usize> Row<W> {
    pub fn new(v: &[i64]) -> Row<W> {
        Row { eq: v.try_into().unwrap() }
    }
    fn map(self, f: fn(&i64) -> i64) -> Row<W> {
        Row {
            eq: self.eq
                .iter()
                .map(f)
                .collect::<Vec<i64>>()
                .try_into().unwrap()
        }
    }
}

impl<const W: usize> Default for Row<W> {
    // How nice are const generics
    fn default() -> Row<W> {
        Row { eq: [0; W] }
    }
}

impl<const W: usize> Index<usize> for Row<W> {
    type Output = i64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.eq[i]
    }
}

impl<const W: usize> IndexMut<usize> for Row<W> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.eq[i]
    }
}

impl<const W: usize> Add for Row<W> {
    type Output = Row<W>;
    fn add(self, rhs: Row<W>) -> Row<W> {
        Row { 
            eq: self.eq
                .iter()
                .zip(rhs.eq)
                .map(|(x, y)| x + y)
                .collect::<Vec<i64>>()
                .try_into().unwrap()
        }
    }
}

impl<const W: usize> AddAssign for Row<W> {
    fn add_assign(&mut self, rhs: Row<W>) {
        let mut i = 0;
        self.eq
            .iter_mut()
            .for_each(|x| {*x += rhs[i]; i += 1});
    }
}


impl<const W: usize> Sub for Row<W> {
    type Output = Row<W>;
    fn sub(self, rhs: Row<W>) -> Row<W> {
        Row {
            eq: self.eq
                .iter()
                .zip(rhs.eq)
                .map(|(x, y)| x - y)
                .collect::<Vec<i64>>()
                .try_into().unwrap()
        }
    }
}

impl<const W: usize> SubAssign for Row<W> {
    fn sub_assign(&mut self, rhs: Row<W>) {
        let mut i = 0;
        self.eq
            .iter_mut()
            .for_each(|x| {*x -= rhs[i]; i += 1});
    }
}

// The Inner Product
impl<const W: usize> Mul for Row<W> {
    type Output = i64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.eq
            .iter()
            .zip(rhs.eq)
            .fold(0, |acc, (x, y)| acc + x * y)
    }
}

impl<const W: usize> Display for Row<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.eq.len() - 1;
        for i in 0 .. n {
            write!(f, "{} ", self[i]).unwrap();
        }
        write!(f, "{}", self[n])
    }
}

impl<const W: usize> std::fmt::Debug for Row<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.eq.len() - 1;
        write!(f, "[").unwrap();
        for i in 0 .. n {
            write!(f, "{}, ", self[i]).unwrap();
        }
        write!(f, "{}]", self[n])
    }
}



impl<const W: usize> std::iter::IntoIterator for Row<W> {
    type Item = i64;
    type IntoIter = std::array::IntoIter<Self::Item, W>;
    fn into_iter(self) -> Self::IntoIter {
        self.eq.into_iter()
    }
}

impl<const W: usize> Rem<i64> for Row<W> {
    type Output = Row<W>;
    fn rem(self, m: i64) -> Row<W> {
        Row {
            eq: self.eq
                .into_iter()
                .map(|x| x % m)
                .collect::<Vec<i64>>()
                .try_into().unwrap()
        }
    }
}

impl<const W: usize> RemAssign<i64> for Row<W> {
    fn rem_assign(&mut self, m: i64) {
        self.eq
            .iter_mut()
            .for_each(|x| *x %= m);
    }
}
