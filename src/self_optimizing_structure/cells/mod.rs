mod genes;

pub use crate::genes::Genes;

pub struct StemCell {
    genes: Genes,
}

pub trait Cell {
    fn size() -> int;
    fn debug_print();
    fn print_prefix();
}