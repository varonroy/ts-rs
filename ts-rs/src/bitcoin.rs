use super::{impl_primitives, impl_shadow, TS};

use bitcoin::{
    Address, Amount, CompactTarget, CompressedPublicKey, PrivateKey, PubkeyHash, PublicKey,
    ScriptBuf, WPubkeyHash,
};

impl_primitives! { Address => "string" }
impl_primitives! { CompactTarget => "number" }
impl_primitives! { Amount => "bigint" }
impl_primitives! { PublicKey => "string" }
impl_primitives! { PrivateKey => "string" }
impl_primitives! { CompressedPublicKey => "string" }
impl_primitives! { PubkeyHash => "string" }
impl_primitives! { WPubkeyHash => "string" }

impl_shadow!(as Vec<u8>: impl TS for ScriptBuf);

mod secp256k1 {
    use super::TS;
    use super::{impl_primitives, impl_shadow};

    mod ffi {
        use super::impl_primitives;
        use super::TS;
        use bitcoin::secp256k1::ffi::Signature;

        impl_primitives! { Signature => "string" }

        mod recovery {
            use super::impl_primitives;
            use super::TS;
            use bitcoin::secp256k1::ffi::recovery::RecoverableSignature;

            impl_primitives! { RecoverableSignature => "string" }
        }
    }

    mod ecdsa {
        use super::impl_shadow;
        use super::TS;

        use bitcoin::secp256k1::ecdsa::{RecoverableSignature, RecoveryId, Signature};

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "Signature",
            export_to = "bitcoin/secp256k1/ecdsa"
        )]
        pub struct TsSignature(pub String);

        impl_shadow!(as TsSignature: impl TS for Signature);

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "RecoverableSignature",
            export_to = "bitcoin/secp256k1/ecdsa"
        )]
        pub struct TsRecoverableSignature(pub String);

        impl_shadow!(as TsRecoverableSignature: impl TS for RecoverableSignature);

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "RecoveryId",
            export_to = "bitcoin/secp256k1/ecdsa"
        )]
        pub struct TsRecoveryId(pub i32);

        impl_shadow!(as TsRecoveryId: impl TS for RecoveryId);
    }
}

mod ecdsa {
    use super::impl_primitives;
    use super::TS;
    use bitcoin::ecdsa::Signature;

    impl_primitives! { Signature => "string" }
}

mod network {
    use super::impl_shadow;
    use super::TS;
    use bitcoin::network::{Network, NetworkKind};

    #[derive(TS)]
    #[ts(
        crate = "crate",
        rename = "NetworkKind",
        export_to = "bitcoin/network/"
    )]
    pub enum TsNetworkKind {
        Main,
        Test,
    }

    #[derive(TS)]
    #[ts(crate = "crate", rename = "Network", export_to = "bitcoin/network/")]
    pub enum TsNetwork {
        Bitcoin,
        Testnet,
        Testnet4,
        Signet,
        Regtest,
    }

    impl_shadow!(as TsNetwork: impl TS for Network);
    impl_shadow!(as TsNetworkKind: impl TS for NetworkKind);
}

mod hashes {
    use super::impl_primitives;
    use super::TS;
    use bitcoin::hashes::{
        ripemd160, sha1, sha256, sha256d, sha384, sha512, sha512_256, siphash24,
    };

    impl_primitives! { ripemd160::Hash => "string" }
    impl_primitives! { sha1::Hash => "string" }
    impl_primitives! { sha256::Hash => "string" }
    impl_primitives! { sha256d::Hash => "string" }
    impl_primitives! { sha384::Hash => "string" }
    impl_primitives! { sha512::Hash => "string" }
    impl_primitives! { sha512_256::Hash => "string" }
    impl_primitives! { siphash24::Hash => "string" }
}

mod relative {
    use super::TS;
    use super::{impl_primitives, impl_shadow};

    use bitcoin::relative::{Height, LockTime, Time};

    impl_primitives! { Height => "number" }
    impl_primitives! { Time => "number" }

    #[derive(TS)]
    #[ts(crate = "crate", rename = "LockTime", export_to = "bitcoin/relative/")]
    pub enum TsLockTime {
        Blocks(Height),
        Time(Time),
    }

    impl_shadow!(as TsLockTime: impl TS for LockTime);
}

mod absolute {
    use super::impl_primitives;
    use super::TS;

    use bitcoin::absolute::{Height, LockTime, Time};

    impl_primitives! { Height => "number" }
    impl_primitives! { Time => "number" }
    impl_primitives! { LockTime => "number" }
}

mod blockdata {
    use super::TS;
    use super::{impl_primitives, impl_shadow};

    mod block {
        use super::TS;
        use super::{impl_primitives, impl_shadow};
        use bitcoin::block::{
            Block, BlockHash, Header, TxMerkleNode, Version, WitnessCommitment, WitnessMerkleNode,
        };
        use bitcoin::CompactTarget;

        impl_primitives! { BlockHash => "string" }
        impl_primitives! { TxMerkleNode => "string" }
        impl_primitives! { WitnessMerkleNode => "string" }
        impl_primitives! { WitnessCommitment => "string" }

        impl_primitives! { Version => "number" }

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "Header",
            export_to = "bitcoin/blockdata/block/"
        )]
        pub struct TsHeader {
            pub version: Version,
            pub prev_blockhash: BlockHash,
            pub merkle_root: TxMerkleNode,
            pub time: u32,
            pub bits: CompactTarget,
            pub nonce: u32,
        }

        impl_shadow!(as TsHeader: impl TS for Header);

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "Block",
            export_to = "bitcoin/blockdata/block/"
        )]
        pub struct TsBlock {
            pub header: Header,
            pub txdata: Vec<bitcoin::blockdata::transaction::Transaction>,
        }

        impl_shadow!(as TsBlock: impl TS for Block);
    }

    mod transaction {
        use super::TS;
        use super::{impl_primitives, impl_shadow};

        use bitcoin::blockdata::transaction::{Sequence, Transaction, TxIn, TxOut, Txid, Version};
        use bitcoin::{Amount, OutPoint, ScriptBuf, Witness};

        impl_primitives! { Txid => "string" }
        impl_primitives! { Version => "number" }
        impl_primitives! { Sequence => "number" }

        // The `bitcoin` crate serializes `OutPoint` as a string (`txid:vout`).
        impl_primitives! { OutPoint => "string" }

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "Transaction",
            export_to = "bitcoin/blockdata/transaction/"
        )]
        pub struct TsTransaction {
            pub version: Version,
            pub lock_time: bitcoin::absolute::LockTime,
            pub input: Vec<TxIn>,
            pub output: Vec<TxOut>,
        }

        impl_shadow!(as TsTransaction: impl TS for Transaction);

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "TxIn",
            export_to = "bitcoin/blockdata/transaction/"
        )]
        pub struct TsTxIn {
            pub previous_output: OutPoint,
            pub script_sig: ScriptBuf,
            pub sequence: Sequence,
            pub witness: Witness,
        }

        impl_shadow!(as TsTxIn: impl TS for TxIn);

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "TxOut",
            export_to = "bitcoin/blockdata/transaction/"
        )]
        pub struct TsTxOut {
            pub value: Amount,
            pub script_pubkey: ScriptBuf,
        }

        impl_shadow!(as TsTxOut: impl TS for TxOut);
    }

    pub mod witness {
        use super::impl_shadow;
        use super::TS;

        use bitcoin::blockdata::witness::Witness;

        #[derive(TS)]
        #[ts(
            crate = "crate",
            rename = "Witness",
            export_to = "bitcoin/blockdata/witness/"
        )]
        pub struct TsWitness {
            pub content: Vec<u8>,
            pub witness_elements: usize,
            pub indices_start: usize,
        }

        impl_shadow!(as TsWitness: impl TS for Witness);
    }
}
