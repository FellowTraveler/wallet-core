use crate::brc20::{BRC20TransferInscription, Ticker};
use crate::entry::aliases::*;
use crate::{Error, Result};
use bitcoin::taproot::{LeafVersion, TapNodeHash};
use bitcoin::{PubkeyHash, ScriptBuf, WPubkeyHash};
use secp256k1::hashes::Hash;
use secp256k1::XOnlyPublicKey;
use tw_misc::traits::ToBytesVec;
use tw_proto::BitcoinV2::Proto;

pub struct OutputBuilder;

impl OutputBuilder {
    /// Creates the spending condition (_scriptPubkey_) for a given output.
    pub fn utxo_from_proto(
        output: &Proto::Output<'_>,
    ) -> Result<Proto::mod_PreSigningOutput::TxOut<'static>> {
        let secp = secp256k1::Secp256k1::new();
        let (script_pubkey, control_block) = match &output.to_recipient {
            // Script spending condition was passed on directly.
            ProtoOutputRecipient::script_pubkey(script) => {
                (ScriptBuf::from_bytes(script.to_vec()), None)
            },
            // Process builder methods. We construct the Script spending
            // conditions by using the specified parameters.
            ProtoOutputRecipient::builder(builder) => match &builder.variant {
                ProtoOutputBuilder::p2sh(_) => {
                    todo!()
                },
                ProtoOutputBuilder::p2pkh(pubkey_or_hash) => {
                    let pubkey_hash = pubkey_hash_from_proto(pubkey_or_hash)?;
                    (ScriptBuf::new_p2pkh(&pubkey_hash), None)
                },
                ProtoOutputBuilder::p2wsh(_) => {
                    todo!()
                },
                ProtoOutputBuilder::p2wpkh(pubkey_or_hash) => {
                    let wpubkey_hash = witness_pubkey_hash_from_proto(pubkey_or_hash)?;
                    (ScriptBuf::new_v0_p2wpkh(&wpubkey_hash), None)
                },
                ProtoOutputBuilder::p2tr_key_path(pubkey) => {
                    let pubkey = bitcoin::PublicKey::from_slice(pubkey.as_ref())?;
                    let xonly = XOnlyPublicKey::from(pubkey.inner);
                    (ScriptBuf::new_v1_p2tr(&secp, xonly, None), None)
                },
                ProtoOutputBuilder::p2tr_script_path(complex) => {
                    let node_hash = TapNodeHash::from_slice(complex.node_hash.as_ref())
                        .map_err(|_| Error::from(Proto::Error::Error_invalid_taproot_root))?;

                    let pubkey = bitcoin::PublicKey::from_slice(complex.public_key.as_ref())?;
                    let xonly = XOnlyPublicKey::from(pubkey.inner);

                    (ScriptBuf::new_v1_p2tr(&secp, xonly, Some(node_hash)), None)
                },
                ProtoOutputBuilder::brc20_inscribe(brc20) => {
                    let pubkey = bitcoin::PublicKey::from_slice(brc20.inscribe_to.as_ref())?;
                    let xonly = XOnlyPublicKey::from(pubkey.inner);

                    let ticker = Ticker::new(brc20.ticker.to_string())?;
                    let transfer =
                        BRC20TransferInscription::new(pubkey.into(), ticker, brc20.transfer_amount)
                            .expect("invalid BRC20 transfer construction");

                    // Explicit check
                    let control_block = transfer
                        .inscription()
                        .spend_info()
                        .control_block(&(
                            transfer.inscription().taproot_program().to_owned(),
                            LeafVersion::TapScript,
                        ))
                        .expect("incorrectly constructed control block");

                    let merkle_root = transfer
                        .inscription()
                        .spend_info()
                        .merkle_root()
                        .expect("incorrectly constructed Taproot merkle root");
                    (
                        ScriptBuf::new_v1_p2tr(&secp, xonly, Some(merkle_root)),
                        Some(control_block.serialize()),
                    )
                },
                ProtoOutputBuilder::None => todo!(),
            },
            // We derive the spending condition for the address.
            ProtoOutputRecipient::from_address(_) => todo!(),
            ProtoOutputRecipient::None => todo!(),
        };

        let utxo = Proto::mod_PreSigningOutput::TxOut {
            value: output.amount,
            script_pubkey: script_pubkey.to_vec().into(),
            control_block: control_block.map(|cb| cb.into()).unwrap_or_default(),
        };

        Ok(utxo)
    }
}

// Conenience helper function.
fn pubkey_hash_from_proto(pubkey_or_hash: &Proto::ToPublicKeyOrHash) -> Result<PubkeyHash> {
    let pubkey_hash = match &pubkey_or_hash.to_address {
        ProtoPubkeyOrHash::hash(hash) => PubkeyHash::from_slice(hash.as_ref())
            .map_err(|_| Error::from(Proto::Error::Error_invalid_pubkey_hash))?,
        ProtoPubkeyOrHash::pubkey(pubkey) => {
            bitcoin::PublicKey::from_slice(pubkey.as_ref())?.pubkey_hash()
        },
        ProtoPubkeyOrHash::None => todo!(),
    };

    Ok(pubkey_hash)
}

// Conenience helper function.
fn witness_pubkey_hash_from_proto(
    pubkey_or_hash: &Proto::ToPublicKeyOrHash,
) -> Result<WPubkeyHash> {
    let wpubkey_hash = match &pubkey_or_hash.to_address {
        ProtoPubkeyOrHash::hash(hash) => WPubkeyHash::from_slice(hash.as_ref())
            .map_err(|_| Error::from(Proto::Error::Error_invalid_witness_pubkey_hash))?,
        ProtoPubkeyOrHash::pubkey(pubkey) => bitcoin::PublicKey::from_slice(pubkey.as_ref())?
            .wpubkey_hash()
            .ok_or_else(|| Error::from(Proto::Error::Error_invalid_witness_pubkey_hash))?,
        ProtoPubkeyOrHash::None => todo!(),
    };

    Ok(wpubkey_hash)
}