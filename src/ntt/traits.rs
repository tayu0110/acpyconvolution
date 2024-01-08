use super::FftCache;
use crate::Modulo;

pub trait NumberTheoreticTransform<M: Modulo> {
    const CACHE: FftCache<M> = FftCache::new();

    fn ntt(&mut self);
    fn intt(&mut self);
}
