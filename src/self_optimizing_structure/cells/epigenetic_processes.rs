pub trait GeneTransvection {
    fn activated_gene(&self) -> GeneTransvection;
    fn suppressed_gene(&self) -> GeneTransvection;
}