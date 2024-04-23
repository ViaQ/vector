// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `InventoryOptionalField`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let inventoryoptionalfield = unimplemented!();
/// match inventoryoptionalfield {
///     InventoryOptionalField::BucketKeyStatus => { /* ... */ },
///     InventoryOptionalField::ChecksumAlgorithm => { /* ... */ },
///     InventoryOptionalField::ETag => { /* ... */ },
///     InventoryOptionalField::EncryptionStatus => { /* ... */ },
///     InventoryOptionalField::IntelligentTieringAccessTier => { /* ... */ },
///     InventoryOptionalField::IsMultipartUploaded => { /* ... */ },
///     InventoryOptionalField::LastModifiedDate => { /* ... */ },
///     InventoryOptionalField::ObjectAccessControlList => { /* ... */ },
///     InventoryOptionalField::ObjectLockLegalHoldStatus => { /* ... */ },
///     InventoryOptionalField::ObjectLockMode => { /* ... */ },
///     InventoryOptionalField::ObjectLockRetainUntilDate => { /* ... */ },
///     InventoryOptionalField::ObjectOwner => { /* ... */ },
///     InventoryOptionalField::ReplicationStatus => { /* ... */ },
///     InventoryOptionalField::Size => { /* ... */ },
///     InventoryOptionalField::StorageClass => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `inventoryoptionalfield` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `InventoryOptionalField::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `InventoryOptionalField::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `InventoryOptionalField::NewFeature` is defined.
/// Specifically, when `inventoryoptionalfield` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `InventoryOptionalField::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum InventoryOptionalField {
    #[allow(missing_docs)] // documentation missing in model
    BucketKeyStatus,
    #[allow(missing_docs)] // documentation missing in model
    ChecksumAlgorithm,
    #[allow(missing_docs)] // documentation missing in model
    ETag,
    #[allow(missing_docs)] // documentation missing in model
    EncryptionStatus,
    #[allow(missing_docs)] // documentation missing in model
    IntelligentTieringAccessTier,
    #[allow(missing_docs)] // documentation missing in model
    IsMultipartUploaded,
    #[allow(missing_docs)] // documentation missing in model
    LastModifiedDate,
    #[allow(missing_docs)] // documentation missing in model
    ObjectAccessControlList,
    #[allow(missing_docs)] // documentation missing in model
    ObjectLockLegalHoldStatus,
    #[allow(missing_docs)] // documentation missing in model
    ObjectLockMode,
    #[allow(missing_docs)] // documentation missing in model
    ObjectLockRetainUntilDate,
    #[allow(missing_docs)] // documentation missing in model
    ObjectOwner,
    #[allow(missing_docs)] // documentation missing in model
    ReplicationStatus,
    #[allow(missing_docs)] // documentation missing in model
    Size,
    #[allow(missing_docs)] // documentation missing in model
    StorageClass,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for InventoryOptionalField {
    fn from(s: &str) -> Self {
        match s {
            "BucketKeyStatus" => InventoryOptionalField::BucketKeyStatus,
            "ChecksumAlgorithm" => InventoryOptionalField::ChecksumAlgorithm,
            "ETag" => InventoryOptionalField::ETag,
            "EncryptionStatus" => InventoryOptionalField::EncryptionStatus,
            "IntelligentTieringAccessTier" => InventoryOptionalField::IntelligentTieringAccessTier,
            "IsMultipartUploaded" => InventoryOptionalField::IsMultipartUploaded,
            "LastModifiedDate" => InventoryOptionalField::LastModifiedDate,
            "ObjectAccessControlList" => InventoryOptionalField::ObjectAccessControlList,
            "ObjectLockLegalHoldStatus" => InventoryOptionalField::ObjectLockLegalHoldStatus,
            "ObjectLockMode" => InventoryOptionalField::ObjectLockMode,
            "ObjectLockRetainUntilDate" => InventoryOptionalField::ObjectLockRetainUntilDate,
            "ObjectOwner" => InventoryOptionalField::ObjectOwner,
            "ReplicationStatus" => InventoryOptionalField::ReplicationStatus,
            "Size" => InventoryOptionalField::Size,
            "StorageClass" => InventoryOptionalField::StorageClass,
            other => InventoryOptionalField::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for InventoryOptionalField {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(InventoryOptionalField::from(s))
    }
}
impl InventoryOptionalField {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            InventoryOptionalField::BucketKeyStatus => "BucketKeyStatus",
            InventoryOptionalField::ChecksumAlgorithm => "ChecksumAlgorithm",
            InventoryOptionalField::ETag => "ETag",
            InventoryOptionalField::EncryptionStatus => "EncryptionStatus",
            InventoryOptionalField::IntelligentTieringAccessTier => "IntelligentTieringAccessTier",
            InventoryOptionalField::IsMultipartUploaded => "IsMultipartUploaded",
            InventoryOptionalField::LastModifiedDate => "LastModifiedDate",
            InventoryOptionalField::ObjectAccessControlList => "ObjectAccessControlList",
            InventoryOptionalField::ObjectLockLegalHoldStatus => "ObjectLockLegalHoldStatus",
            InventoryOptionalField::ObjectLockMode => "ObjectLockMode",
            InventoryOptionalField::ObjectLockRetainUntilDate => "ObjectLockRetainUntilDate",
            InventoryOptionalField::ObjectOwner => "ObjectOwner",
            InventoryOptionalField::ReplicationStatus => "ReplicationStatus",
            InventoryOptionalField::Size => "Size",
            InventoryOptionalField::StorageClass => "StorageClass",
            InventoryOptionalField::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "BucketKeyStatus",
            "ChecksumAlgorithm",
            "ETag",
            "EncryptionStatus",
            "IntelligentTieringAccessTier",
            "IsMultipartUploaded",
            "LastModifiedDate",
            "ObjectAccessControlList",
            "ObjectLockLegalHoldStatus",
            "ObjectLockMode",
            "ObjectLockRetainUntilDate",
            "ObjectOwner",
            "ReplicationStatus",
            "Size",
            "StorageClass",
        ]
    }
}
impl ::std::convert::AsRef<str> for InventoryOptionalField {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl InventoryOptionalField {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
