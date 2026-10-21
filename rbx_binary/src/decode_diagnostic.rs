use std::str::Utf8Error;

use rbx_dom_weak::types::AttributeDecodeDiagnostic;
use rbx_reflection::MigrationError;

use crate::{
    BinaryStringFallbackCause, ColorSequenceEnvelope, FontFallback, InstanceLocation,
    PropertyLocation, PropertySkipReason, ReferenceKind,
};

/// A source distinction omitted or replaced by permissive binary decoding.
///
/// Document referents are the original signed native IDs. Instance refs are the
/// actual DOM identities allocated during this pass, not reconstructed IDs.
/// An unparented instance's allocated ref is absent from the returned DOM.
#[derive(Debug)]
pub enum DecodeDiagnostic {
    /// An otherwise successful Attributes value replaced a source distinction.
    AttributeLoss {
        /// The original serialized property identity containing the attributes.
        property: PropertyLocation,
        /// The original document ID and this pass's allocated DOM identity.
        instance: InstanceLocation,
        /// The evidence produced by the actual Attributes decoding pass.
        diagnostic: AttributeDecodeDiagnostic,
    },
    /// A native color envelope's source bits differed from writer normalization.
    ColorSequenceEnvelope {
        /// The original serialized property identity.
        property: PropertyLocation,
        /// The original document ID and this pass's allocated DOM identity.
        instance: InstanceLocation,
        /// The zero-based source keypoint index within this property's value.
        keypoint_index: u32,
        /// The exact original bits and the shared writer normalization.
        envelope: ColorSequenceEnvelope,
    },
    /// An unknown Font weight or style was replaced with a default value.
    FontFallback {
        /// The original serialized property identity.
        property: PropertyLocation,
        /// The original document ID and this pass's allocated DOM identity.
        instance: InstanceLocation,
        /// The raw discriminator and the typed replacement inserted in the DOM.
        fallback: FontFallback,
    },
    /// An unrecognized chunk was ignored in full.
    UnknownChunk {
        /// The exact four-byte chunk name, including non-UTF-8 bytes.
        name: [u8; 4],
        /// The ignored uncompressed chunk payload.
        data: Vec<u8>,
    },
    /// One PROP chunk was skipped for all of its class's declared instances.
    PropertySkipped {
        /// The original serialized class and property identity.
        property: PropertyLocation,
        /// All affected document/DOM identity pairs, in INST order.
        instances: Vec<InstanceLocation>,
        /// The exact policy or unsupported representation causing omission.
        reason: PropertySkipReason,
        /// The unconsumed payload after the type byte, or empty if it was absent.
        data: Vec<u8>,
    },
    /// A property migration failed and its original value was omitted.
    MigrationFailed {
        /// The original serialized property identity.
        property: PropertyLocation,
        /// The original document ID and this pass's allocated DOM identity.
        instance: InstanceLocation,
        /// The canonical property that attempted the migration.
        canonical_property: String,
        /// The typed migration operation, found value and accepted value evidence.
        cause: MigrationError,
    },
    /// A Name or ordinary String was decoded with replacement characters.
    InvalidUtf8 {
        /// The original serialized property identity.
        property: PropertyLocation,
        /// The original document ID and this pass's allocated DOM identity.
        instance: InstanceLocation,
        /// The original bytes before lossy UTF-8 replacement.
        bytes: Vec<u8>,
        /// The standard UTF-8 decoder's exact invalid-sequence location.
        cause: Utf8Error,
    },
    /// Binary bytes survived but their intended semantic value was unavailable.
    BinaryStringFallback {
        /// The original serialized property identity.
        property: PropertyLocation,
        /// The original document ID and this pass's allocated DOM identity.
        instance: InstanceLocation,
        /// The preserved bytes passed to the semantic decoder.
        bytes: Vec<u8>,
        /// The specific semantic decoder and its original typed error.
        cause: BinaryStringFallbackCause,
    },
    /// A non-null target was unknown at the point of decoding and became null.
    UnresolvedReference {
        /// The original serialized property identity.
        property: PropertyLocation,
        /// The original document ID and this pass's allocated DOM identity.
        instance: InstanceLocation,
        /// The non-null target document referent that could not be resolved.
        target: i32,
        /// The native property representation that lost the target.
        kind: ReferenceKind,
    },
    /// The Content external-reference section was ignored by the decoder.
    ExternalContentReferencesIgnored {
        /// The original serialized property identity.
        property: PropertyLocation,
        /// All source document/DOM pairs to which this Content PROP belongs.
        instances: Vec<InstanceLocation>,
        /// The exact count declared by the native Content payload.
        count: u32,
        /// The raw trailing external-reference bytes, before any interpretation.
        data: Vec<u8>,
    },
    /// An INST node was not reachable from a native PRNT root and was omitted.
    UnparentedInstance {
        /// The original document ID and allocated, but omitted, DOM identity.
        instance: InstanceLocation,
        /// The original instance class.
        class_name: String,
    },
}
