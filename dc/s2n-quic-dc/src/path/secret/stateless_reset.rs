// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::schedule;
use crate::credentials::Id;
use aws_lc_rs::hkdf::{Prk, Salt, HKDF_SHA384};

#[derive(Debug)]
pub struct Signer {
    prk: Prk,
}

impl Signer {
    /// Creates a signer with the given secret
    pub fn new(secret: &[u8]) -> Self {
        let prk = Salt::new(HKDF_SHA384, secret).extract(b"rst");
        Self { prk }
    }

    /// Returns a random `Signer`
    ///
    /// Note that this signer cannot be used across restarts and will result in an endpoint
    /// producing invalid `UnknownPathSecret` packets.
    pub fn random() -> Self {
        let mut secret = [0u8; 32];
        aws_lc_rs::rand::fill(&mut secret).unwrap();
        Self::new(&secret)
    }

    pub fn sign(&self, id: &Id) -> [u8; 16] {
        let mut stateless_reset = [0; 16];

        self.prk
            .expand(&[&[16], b"rst ", &**id], schedule::OutLen(16))
            .unwrap()
            .fill(&mut stateless_reset)
            .unwrap();

        stateless_reset
    }
}
