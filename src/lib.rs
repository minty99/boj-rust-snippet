#![allow(dead_code)]
#![allow(non_snake_case)]

use std::error::Error;
use std::fmt::Debug;
use std::io::stdin;
use std::io::stdout;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

pub fn solve(mut input: impl BufRead, mut output: impl Write) -> Result<(), Box<dyn Error>> {
    // Your solution here
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut input_buf = String::new();
    stdin().read_to_string(&mut input_buf).unwrap();
    let stdout = stdout().lock();

    solve(&mut input_buf.as_bytes(), &mut BufWriter::new(stdout))?;
    Ok(())
}

trait SimpleRead: BufRead {
    fn read_tuple<T>(&mut self) -> T
    where
        T: ReadableTuple,
    {
        let mut line = String::new();
        self.read_line(&mut line).unwrap();
        let mut iter = line.split_ascii_whitespace().map(|s| s.to_string());
        T::from_iter(&mut iter)
    }

    fn read_array<T, const N: usize>(&mut self) -> [T; N]
    where
        T: FromStr + Debug,
        <T as FromStr>::Err: Debug,
    {
        self.read_vec().try_into().unwrap()
    }

    fn read_vec<T>(&mut self) -> Vec<T>
    where
        T: FromStr + Debug,
        <T as FromStr>::Err: Debug,
    {
        let mut line = String::new();
        self.read_line(&mut line).unwrap();
        line.split_ascii_whitespace()
            .map(|e| e.parse::<T>().unwrap())
            .collect()
    }

    fn read_single<T>(&mut self) -> T
    where
        T: FromStr + Debug,
        <T as FromStr>::Err: Debug,
    {
        let mut line = String::new();
        self.read_line(&mut line).unwrap();
        line.trim().parse::<T>().unwrap()
    }
}

impl<T: BufRead> SimpleRead for T {}

trait ReadableTuple {
    fn from_iter<I>(iter: &mut I) -> Self
    where
        I: Iterator<Item = String>;
}

macro_rules! impl_readable_tuple {
    ($($name:ident),+) => {
        impl<$($name),+> ReadableTuple for ($($name),+)
        where
            $($name: FromStr + Debug, <$name as FromStr>::Err: Debug),+
        {
            fn from_iter<I>(iter: &mut I) -> Self
            where
                I: Iterator<Item = String>,
            {
                (
                    $(
                        iter.next().expect("Not enough elements").parse::<$name>().unwrap()
                    ),+
                )
            }
        }
    };
}

impl_readable_tuple!(T1, T2);
impl_readable_tuple!(T1, T2, T3);
impl_readable_tuple!(T1, T2, T3, T4);
