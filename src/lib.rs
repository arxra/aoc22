#![feature(if_let_guard)]
#![allow(unused)]
mod d_01;
mod d_02;
mod d_03;
mod d_04;
mod d_05;
mod d_06;
mod d_07;
mod d_08;
mod d_09;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
