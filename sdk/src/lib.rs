pub mod proto {
    pub mod prover;
}
pub mod client;

use crate::client::SP1ProverClient;
use anyhow::{Ok, Result};
use p3_commit::Pcs;
use p3_matrix::dense::RowMajorMatrix;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sp1_core::stark::OpeningProof;
use sp1_core::stark::ShardMainData;
use sp1_core::stark::StarkGenericConfig;
use sp1_core::utils::BabyBearBlake3;
use sp1_core::utils::StarkUtils;
use sp1_core::SP1ProofWithIO;
use sp1_core::SP1Prover;
use sp1_core::SP1Stdin;
use std::time::Duration;
use tokio::time::sleep;

pub struct SP1SDKProver;

impl SP1SDKProver {
    /// Generate a proof for the execution of the ELF with the given public inputs.
    pub fn prove(elf: &[u8], stdin: SP1Stdin) -> Result<SP1ProofWithIO<BabyBearBlake3>> {
        if std::env::var("SP1_SERVICE_ACCESS_TOKEN").is_ok() {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    let client = SP1ProverClient::new();
                    let id = client.create_proof(elf, &stdin.buffer.data).await?;

                    loop {
                        let status = client.get_proof_status(&id).await?;
                        if let Some(result) = status.1 {
                            return Ok(result);
                        }
                        sleep(Duration::from_secs(10)).await;
                    }
                })
            })
        } else {
            let result = SP1Prover::prove(elf, stdin)?;
            Ok(result)
        }
    }

    pub fn prove_with_config<SC>(
        elf: &[u8],
        stdin: SP1Stdin,
        config: SC,
    ) -> Result<SP1ProofWithIO<SC>>
    where
        SC: StarkUtils + Send + Sync + Serialize + DeserializeOwned + Clone,
        SC::Challenger: Clone,
        OpeningProof<SC>: Send + Sync,
        <SC::Pcs as Pcs<SC::Val, RowMajorMatrix<SC::Val>>>::Commitment: Send + Sync,
        <SC::Pcs as Pcs<SC::Val, RowMajorMatrix<SC::Val>>>::ProverData: Send + Sync,
        ShardMainData<SC>: Serialize + DeserializeOwned,
        <SC as StarkGenericConfig>::Val: p3_field::PrimeField32,
    {
        if std::env::var("SP1_SERVICE_ACCESS_TOKEN").is_ok() {
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(async move {
                    let client = SP1ProverClient::new();
                    let id = client.create_proof(elf, &stdin.buffer.data).await?;

                    loop {
                        let status = client.get_proof_status(&id).await?;
                        if let Some(result) = status.1 {
                            return Ok(result);
                        }
                        sleep(Duration::from_secs(10)).await;
                    }
                })
            })
        } else {
            let result = SP1Prover::prove_with_config(elf, stdin, config)?;
            Ok(result)
        }
    }
}