use bitcoin::RBF;
use bitcoin::BlockData::Fee_Rate;

struct Feerate {
    rbf: RBF,
    // Other fields related to the feerate
  pub const MIN: FeeRate = FeeRate::ZERO;
  pub const MAX: FeeRate = FeeRate(u64::MAX);
  pub const BROADCAST_MIN: FeeRate = FeeRate::from_sat_per_vb_unchecked(1);
}

impl Feerate {
    fn new(rbf: RBF) -> Self {
        Feerate { rbf }
    }
}

    
