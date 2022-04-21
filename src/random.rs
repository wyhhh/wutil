use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use rand::thread_rng;
use rand::Rng;

pub fn gen<T, R>(r: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    thread_rng().gen_range(r)
}
