use num_bigint::BigUint;

pub struct VerificationData {
    pub challenge: u64,
    pub y1: u64,
    pub y2: u64,
    pub r1: u64,
    pub r2: u64,
    pub s: u64,
}

// Abstraction over zkp verification algorithm
pub trait Verifier: Send + Sync {
    fn verify(&self, data: VerificationData) -> bool;
}

pub struct ZkpVerifier {
    pub p: BigUint,
    pub g: BigUint,
    pub h: BigUint,
}

impl Verifier for ZkpVerifier {
    fn verify(&self, data: VerificationData) -> bool {
        let expected_r1 = self.g.modpow(&data.s.into(), &self.p)
            * BigUint::from(data.y1).modpow(&data.challenge.into(), &self.p)
            % &self.p;

        if expected_r1 != data.r1.into() {
            return false;
        }

        let expected_r2 = self.h.modpow(&data.s.into(), &self.p)
            * BigUint::from(data.y2).modpow(&data.challenge.into(), &self.p)
            % &self.p;

        if expected_r2 != data.r2.into() {
            return false;
        }

        true
    }
}
