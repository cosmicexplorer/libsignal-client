//
// Copyright 2020-2021 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::curve::KeyType;
use crate::MessageVersionType;

use displaydoc::Display;

use std::error::Error;
use std::panic::UnwindSafe;

pub type Result<T> = std::result::Result<T, SignalProtocolError>;

#[derive(Debug, Display)]
pub enum SignalProtocolError {
    /// invalid argument: {0}
    InvalidArgument(String),
    /// invalid state for call to {0} to succeed: {1}
    InvalidState(&'static str, String),

    /// failed to decode protobuf: {0}
    ProtobufDecodingError(prost::DecodeError),
    /// failed to encode protobuf: {0}
    ProtobufEncodingError(prost::EncodeError),
    /// protobuf encoding was invalid
    InvalidProtobufEncoding,

    /// ciphertext serialized bytes were too short <{0}>
    CiphertextMessageTooShort(usize),
    /// {1} ciphertext version was too old <{0}>
    LegacyCiphertextVersion(u8, MessageVersionType),
    /// {1} ciphertext version was unrecognized <{0}>
    UnrecognizedCiphertextVersion(u8, MessageVersionType),
    /// unrecognized {1} message version <{0}>
    UnrecognizedMessageVersion(u32, MessageVersionType),

    /// fingerprint identifiers do not match
    FingerprintIdentifierMismatch,
    /// fingerprint version number mismatch them {0} us {1}
    FingerprintVersionMismatch(u32, u32),
    /// fingerprint parsing error
    FingerprintParsingError,

    /// no key type identifier
    NoKeyTypeIdentifier,
    /// bad key type <{0:#04x}>
    BadKeyType(u8),
    /// bad key length <{1}> for key with type <{0}>
    BadKeyLength(KeyType, usize),

    /// invalid signature detected
    SignatureValidationFailed,

    /// untrusted identity for address {0}
    UntrustedIdentity(crate::ProtocolAddress),

    /// invalid prekey identifier
    InvalidPreKeyId,
    /// invalid signed prekey identifier
    InvalidSignedPreKeyId,

    /// invalid root key length <{0}>
    InvalidRootKeyLength(usize),
    /// invalid chain key length <{0}>
    InvalidChainKeyLength(usize),

    /// invalid MAC key length <{0}>
    InvalidMacKeyLength(usize),
    /// invalid cipher key length <{0}> or nonce length <{1}>
    InvalidCipherCryptographicParameters(usize, usize),
    /// invalid ciphertext message
    InvalidCiphertext,

    /// no sender key state
    NoSenderKeyState,

    /// session with '{0}' not found
    SessionNotFound(String),
    /// invalid session structure
    InvalidSessionStructure,
    /// session for {0} has invalid registration ID {1:X}
    InvalidRegistrationId(crate::ProtocolAddress, u32),

    /// message with old counter {0} / {1}
    DuplicatedMessage(u32, u32),
    /// invalid message {0}
    InvalidMessage(&'static str),
    /// internal error {0}
    InternalError(&'static str),
    /// error while invoking an ffi callback: {0}
    FfiBindingError(String),
    /// error in method call '{0}': {1}
    ApplicationCallbackError(
        &'static str,
        Box<dyn std::error::Error + Send + Sync + UnwindSafe + 'static>,
    ),

    /// invalid sealed sender message {0}
    InvalidSealedSenderMessage(String),
    /// unknown sealed sender message version {0}
    UnknownSealedSenderVersion(u8),
    /// self send of a sealed sender message
    SealedSenderSelfSend,
}

impl Error for SignalProtocolError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SignalProtocolError::ProtobufEncodingError(e) => Some(e),
            SignalProtocolError::ProtobufDecodingError(e) => Some(e),
            SignalProtocolError::ApplicationCallbackError(_, e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<prost::DecodeError> for SignalProtocolError {
    fn from(value: prost::DecodeError) -> SignalProtocolError {
        SignalProtocolError::ProtobufDecodingError(value)
    }
}

impl From<prost::EncodeError> for SignalProtocolError {
    fn from(value: prost::EncodeError) -> SignalProtocolError {
        SignalProtocolError::ProtobufEncodingError(value)
    }
}
