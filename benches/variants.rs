#[derive(Debug)]
pub enum Variant {
    GCRA,
    LeakyBucket,
    Allower,
}

impl Variant {
    pub const ALL: &'static [Variant; 3] = &[Variant::GCRA, Variant::LeakyBucket, Variant::Allower];
}

// I really wish I could just have a function that returns an impl
// Trait that was usable in all the benchmarks, but alas it should not
// be so.
macro_rules! run_with_variants {
    ($variant:expr, $var:ident, $code:block) => {
        match $variant {
            $crate::variants::Variant::GCRA => {
                let mut $var = ::ratelimit_meter::GCRA::for_capacity(50).unwrap().build();
                $code
            }
            $crate::variants::Variant::LeakyBucket => {
                let mut $var = ::ratelimit_meter::LeakyBucket::per_second(
                    ::std::num::NonZeroU32::new(50).unwrap(),
                );
                $code
            }
            $crate::variants::Variant::Allower => {
                let mut $var = ::ratelimit_meter::example_algorithms::Allower::new();
                $code
            }
        }
    };
}