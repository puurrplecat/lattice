pub mod encrypt;
pub mod keygen;
pub mod decrypt;
use std::ops::{Index, IndexMut, Add, AddAssign, Mul, Rem, RemAssign};
use std::fmt::Display;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn rng (mut seed: usize) -> impl FnMut() -> usize {
    move || {
        let mut out = 0;
        for _ in 0..64 {
            let newbit = (seed ^ (seed >> 1) ^ (seed >> 3) ^ (seed >> 4)) & 1;
            out <<= 1;
            out |= seed & 1;
            seed = (seed >> 1) | (newbit << 63);
        }
        out
    }
}

pub struct Equation {
    matrix: Matrix,
    result: Matrix,
    modulus: i64,
}

impl Equation {
    pub fn from_matrices(matrix: Matrix, result: Matrix, modulus: i64) -> Self {
        Equation {
            matrix,
            result,
            modulus,
        }
    }
    pub fn get_height(&self) -> usize {
        self.matrix.height
    }
    pub fn get_width(&self) -> usize {
        self.matrix.width
    }
    pub fn read_equation_from_file(path: &Path) -> Equation {
        let file = BufReader::new(File::open(path.join("public_key.txt")).expect("File went and offed 'imself"));
        let mut lines = file.lines();
        let header = lines.next().unwrap().unwrap()
                          .split_whitespace()
                          .map(|x| x.parse().unwrap())
                          .collect::<Vec<i64>>();
        let (modulus, height) = (header[0], header[1] as usize);
        let matrix = Matrix::read_matrix(&mut lines, height);
        let result = Matrix::read_matrix(&mut lines, height);
        Equation::from_matrices(matrix, result, modulus)
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.modulus, self.matrix.height).unwrap();
        writeln!(f, "{}", self.matrix).unwrap();
        writeln!(f, "{}", self.result)
    }
}

impl std::fmt::Debug for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.matrix).unwrap();
        writeln!(f, "{:?}", self.result).unwrap();
        writeln!(f, "{:?}", self.modulus)
    }
}

#[derive(Clone)]
pub struct Matrix {
    matrix: Vec<Row>,
    height: usize,
    width: usize,
}

impl Matrix {
    pub fn from_slice(v: &[i64], height: usize) -> Matrix {
        assert!(height > 0);
        assert!(v.len() % height == 0);
        let width = v.len() / height;
        Matrix {
            matrix: v
                    .chunks(width)
                    .map(Row::new) // OMG NO CLOSURE
                    .collect::<Vec<Row>>(),
            height,
            width,
        }
    }
    pub fn matrix_builder(height: usize, width: usize) -> Matrix {
        Matrix {
            matrix: Vec::with_capacity(height),
            height,
            width,
        }
    }

    pub fn push_row(&mut self, row: Row) {
        self.matrix.push(row);
    }

    pub fn push_value(&mut self, v: i64) {
        assert!(self.width == 1);
        self.matrix.push(Row(vec![v]));
    }

    // This replaces the matrix, so we need to drop the original.
    pub fn map(&mut self, f: impl Fn(&i64) -> i64) {
        self.matrix = self.matrix
                      .iter()
                      .map(|x| Row::map(x, &f))
                      .collect::<Vec<Row>>()
    }
    // the next line is a sequence of numbers which can be read as a matrix of some height.
    pub fn read_matrix(lines: &mut Lines<BufReader<File>>, height: usize) -> Matrix {
        Matrix::from_slice(&lines.next().unwrap().unwrap()
                                 .split_whitespace()
                                 .map(|x| x.parse().unwrap())
                                 .collect::<Vec<i64>>()
            , height)
    }
}

impl Index<usize> for Matrix {
    type Output = Row;
    fn index(&self, i: usize) -> &Self::Output {
        &self.matrix[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl From<Row> for Matrix {
    fn from(row: Row) -> Self {
        let width = row.0.len();
        Matrix {matrix: vec![row], height: 1, width}
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert!(rhs.height == self.width);
        let mut new = Vec::with_capacity(rhs.height * self.width);
        for i in 0..self.height {
            for j in 0..rhs.width {
                let mut sum = 0;
                for k in 0..self.width { // note self.width == rhs.height
                    sum += self[i][k] * rhs[k][j];
                }
            new.push(sum);
            }
        } 
        Matrix::from_slice(&new, self.height)
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(mut self, rhs: Matrix) -> Self::Output {
        assert!(self.height == rhs.height && self.width == rhs.width);
        self.matrix
        .iter_mut()
        .zip(rhs.matrix.iter())
        .for_each(|(x, y)| *x += y);
        Matrix {matrix: std::mem::take(&mut self.matrix), height: self.height, width: self.width}
    }
}

impl Add<Row> for Matrix {
    type Output = Matrix;
    fn add(mut self, rhs: Row) -> Self::Output {
        assert!(self.height == 1);
        Matrix {matrix: vec![std::mem::take(&mut self.matrix[0]) + rhs], height: 1, width: self.width}
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, rhs: Self) {
        self.matrix
        .iter_mut()
        .zip(rhs.matrix.iter())
        .for_each(|(x, y)| *x += y);
    }
}

impl AddAssign<&Row> for Matrix {
    fn add_assign(&mut self, rhs: &Row) {
        assert!(self.height == 1);
        self.matrix[0] += rhs;
    }    
}

impl Rem<i64> for Matrix {
    type Output = Self;
    fn rem(mut self, rhs: i64) -> Self::Output {
        self.matrix
        .iter_mut()
        .for_each(|x| *x %= rhs);
        Matrix {matrix: std::mem::take(&mut self.matrix), height: self.height, width: self.width}
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height - 1 {
            write!(f, "{} ", self.matrix[i])?;
        }
        write!(f, "{}", self.matrix[self.height - 1])
    }
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.width == 1 {
            // Column vector 
            for i in 0..self.height - 1 {
                write!(f, "{} ", self[i][0])?;
            }
            write!(f, "{} ", self[self.height - 1][0])

        } else {
            for i in 0..self.height - 1 {
                writeln!(f, "{}", self[i])?;
            }
            write!(f, "{}", self[self.height - 1])
        }
    }
}

// The row struct represents a linear combination
#[derive(Clone, Default)]
pub struct Row (Vec<i64>);

impl Row {
    pub fn new(v: &[i64]) -> Self {
        Row(v.into())
    }
    fn map(&self, f: &impl Fn(&i64) -> i64) -> Self {
        Row (self.0
             .iter()
             .map(f)
             .collect::<Vec<i64>>()
        )
    }
    fn new_zeroed(width: usize) -> Self {
        let mut row = vec![0; width];
        row.fill(0);
        Row(row)
    }
}

impl Index<usize> for Row {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add for &mut Row {
    type Output = Row;
    fn add(self, rhs: &mut Row) -> Self::Output {
        self.0
        .iter_mut()
        .zip(rhs.0.iter())
        .for_each(|(x, y)| *x += *y);
        Row(std::mem::take(&mut self.0))
    }
}

impl Add for Row {
    type Output = Row;
    fn add(self, rhs: Row) -> Self::Output {
        let mut new = Vec::with_capacity(self.0.len());
        self.0
        .iter()
        .zip(rhs.0.iter())
        .for_each(|(x, y)| new.push(x + y));
        Row(new)
    }
}

impl AddAssign<&Row> for Row {
    fn add_assign(&mut self, rhs: &Row) {
        self.0
        .iter_mut()
        .zip(rhs.0.iter())
        .for_each(|(x, y)| *x += *y);
    }
}

impl AddAssign for Row {
    fn add_assign(&mut self, rhs: Self) {
        let mut i = 0;
        self.0
            .iter_mut()
            .for_each(|x| {*x += rhs[i]; i += 1});
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.0.len() - 1;
        for i in 0 .. n {
            write!(f, "{} ", self[i]).unwrap();
        }
        write!(f, "{}", self[n])
    }
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.0.len() - 1;
        write!(f, "[").unwrap();
        for i in 0 .. n {
            write!(f, "{}, ", self[i]).unwrap();
        }
        write!(f, "{}]", self[n])
    }
}

impl Rem<i64> for Row {
    type Output = Row;
    fn rem(self, m: i64) -> Row {
        Row (self.0
             .into_iter()
             .map(|x| x % m)
             .collect::<Vec<i64>>(),
                     )
    }
}

impl RemAssign<i64> for Row {
    fn rem_assign(&mut self, rhs: i64) {
        self.0
        .iter_mut()
        .for_each(|x| *x %= rhs);
    }
}