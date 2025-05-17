use crate::Vector;
use crate::dot_product;
use anyhow::Result;
use anyhow::anyhow;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::sync::mpsc;
use std::thread;

const NUM_THREADS: usize = 4;

// [[1,2],[1,2],[1,2]]=>[1,2,1,2,1,2]

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    // sender to send the result back
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("matrix multiply error"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("send error :{:?}", e);
                    };
                }

                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // 4 threads which receive msg and do dot product

    let mut data = vec![T::default(); a.row * b.col];
    let mut recevier = Vec::with_capacity(a.row * b.col);
    for i in 0..a.row {
        for j in 0..b.col {
            // data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let idx = i * b.col + j;
            let col = Vector::new(col_data);
            let input = MsgInput::new(i * b.col + j, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("send error :{:?}", e);
            }
            recevier.push(rx);
        }
    }

    for rx in recevier {
        let ret = rx.recv()?;
        data[ret.idx] = ret.value;
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display as {1 2 3, 4 5 6, 7 8 9}
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }
            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;

        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={}, {}, {})", self.row, self.col, self)
    }
}

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Default + Send + 'static,
{
    type Output = Matrix<T>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        multiply(&self, &other).expect("matrix multiply error")
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_multiply() -> Result<()> {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, &[22, 28, 49, 64]);
        assert_eq!(format!("{:?}", c), "Matrix(row=2, 2, {22 28, 49 64})");
        Ok(())
    }

    #[test]
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new(&[1, 2, 3, 4], 2, 2);
        let b = Matrix::new(&[1, 2, 3, 4], 2, 2);
        let c = a * b;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, &[7, 10, 15, 22]);
        assert_eq!(format!("{:}", c), "{7 10, 15 22}");
        Ok(())
    }

    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b);
        assert!(c.is_err())
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new(&[1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(&[1, 2, 3, 4], 2, 2);
        let _c = a * b;
    }
}
