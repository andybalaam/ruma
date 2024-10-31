use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use ruma_macros::IdZst;

use super::{
    crypto_algorithms::SigningKeyAlgorithm, Base64PublicKey, Base64PublicKeyOrDeviceId, DeviceId,
    DeviceKeyAlgorithm, KeyName, OneTimeKeyAlgorithm, OneTimeKeyName, ServerSigningKeyVersion,
};

/// A key algorithm and key name delimited by a colon.
#[repr(transparent)]
#[derive(IdZst)]
#[ruma_id(
    validate = ruma_identifiers_validation::key_id::validate::<K>,
)]
pub struct KeyId<A: KeyAlgorithm, K: KeyName + ?Sized>(PhantomData<(A, K)>, str);

impl<A: KeyAlgorithm, K: KeyName + ?Sized> KeyId<A, K> {
    /// Creates a new `KeyId` from an algorithm and key name.
    pub fn from_parts(algorithm: A, key_name: &K) -> OwnedKeyId<A, K> {
        let algorithm = algorithm.as_ref();
        let key_name = key_name.as_ref();

        let mut res = String::with_capacity(algorithm.len() + 1 + key_name.len());
        res.push_str(algorithm);
        res.push(':');
        res.push_str(key_name);

        Self::from_borrowed(&res).to_owned()
    }

    /// Returns key algorithm of the key ID.
    pub fn algorithm(&self) -> A {
        A::from(&self.as_str()[..self.colon_idx()])
    }

    /// Returns the key name of the key ID.
    pub fn key_name<'a>(&'a self) -> &'a K
    where
        &'a K: TryFrom<&'a str>,
    {
        <&'a K>::try_from(&self.as_str()[(self.colon_idx() + 1)..])
            .unwrap_or_else(|_| unreachable!())
    }

    fn colon_idx(&self) -> usize {
        self.as_str().find(':').unwrap()
    }
}

/// Algorithm + key name for signing keys.
pub type SigningKeyId<K> = KeyId<SigningKeyAlgorithm, K>;

/// Algorithm + key name for signing keys.
pub type OwnedSigningKeyId<K> = OwnedKeyId<SigningKeyAlgorithm, K>;

/// Algorithm + key name for homeserver signing keys.
pub type ServerSigningKeyId = SigningKeyId<ServerSigningKeyVersion>;

/// Algorithm + key name for homeserver signing keys.
pub type OwnedServerSigningKeyId = OwnedSigningKeyId<ServerSigningKeyVersion>;

/// Algorithm + key name for [device signing keys].
///
/// [device signing keys]: https://spec.matrix.org/latest/client-server-api/#device-keys
pub type DeviceSigningKeyId = SigningKeyId<DeviceId>;

/// Algorithm + key name for [device signing] keys.
///
/// [device signing keys]: https://spec.matrix.org/latest/client-server-api/#device-keys
pub type OwnedDeviceSigningKeyId = OwnedSigningKeyId<DeviceId>;

/// Algorithm + key name for [cross-signing] keys.
///
/// [cross-signing]: https://spec.matrix.org/latest/client-server-api/#cross-signing
pub type CrossSigningKeyId = SigningKeyId<Base64PublicKey>;

/// Algorithm + key name for [cross-signing] keys.
///
/// [cross-signing]: https://spec.matrix.org/latest/client-server-api/#cross-signing
pub type OwnedCrossSigningKeyId = OwnedSigningKeyId<Base64PublicKey>;

/// Algorithm + key name for [cross-signing] or [device signing] keys.
///
/// [cross-signing]: https://spec.matrix.org/latest/client-server-api/#cross-signing
/// [device signing]: https://spec.matrix.org/latest/client-server-api/#device-keys
pub type CrossSigningOrDeviceSigningKeyId = SigningKeyId<Base64PublicKeyOrDeviceId>;

/// Algorithm + key name for [cross-signing] or [device signing] keys.
///
/// [cross-signing]: https://spec.matrix.org/latest/client-server-api/#cross-signing
/// [device signing]: https://spec.matrix.org/latest/client-server-api/#device-keys
pub type OwnedCrossSigningOrDeviceSigningKeyId = OwnedSigningKeyId<Base64PublicKeyOrDeviceId>;

/// Algorithm + key name for [device keys].
///
/// [device keys]: https://spec.matrix.org/latest/client-server-api/#device-keys
pub type DeviceKeyId = KeyId<DeviceKeyAlgorithm, DeviceId>;

/// Algorithm + key name for [device keys].
///
/// [device keys]: https://spec.matrix.org/latest/client-server-api/#device-keys
pub type OwnedDeviceKeyId = OwnedKeyId<DeviceKeyAlgorithm, DeviceId>;

/// Algorithm + key name for [one-time and fallback keys].
///
/// [one-time and fallback keys]: https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys
pub type OneTimeKeyId = KeyId<OneTimeKeyAlgorithm, OneTimeKeyName>;

/// Algorithm + key name for [one-time and fallback keys].
///
/// [one-time and fallback keys]: https://spec.matrix.org/latest/client-server-api/#one-time-and-fallback-keys
pub type OwnedOneTimeKeyId = OwnedKeyId<OneTimeKeyAlgorithm, OneTimeKeyName>;

// The following impls are usually derived using the std macros.
// They are implemented manually here to avoid unnecessary bounds.
impl<A: KeyAlgorithm, K: KeyName + ?Sized> PartialEq for KeyId<A, K> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<A: KeyAlgorithm, K: KeyName + ?Sized> Eq for KeyId<A, K> {}

impl<A: KeyAlgorithm, K: KeyName + ?Sized> PartialOrd for KeyId<A, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<A: KeyAlgorithm, K: KeyName + ?Sized> Ord for KeyId<A, K> {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(self.as_str(), other.as_str())
    }
}

impl<A: KeyAlgorithm, K: KeyName + ?Sized> Hash for KeyId<A, K> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

/// The algorithm of a key.
pub trait KeyAlgorithm: for<'a> From<&'a str> + AsRef<str> {}

impl KeyAlgorithm for SigningKeyAlgorithm {}

impl KeyAlgorithm for DeviceKeyAlgorithm {}

impl KeyAlgorithm for OneTimeKeyAlgorithm {}

#[cfg(test)]
mod tests {
    use assert_matches2::assert_matches;
    use ruma_identifiers_validation::Error;

    use super::DeviceKeyId;

    #[test]
    fn algorithm_and_key_name_are_correctly_extracted() {
        let key_id = DeviceKeyId::parse("ed25519:MYDEVICE").expect("Should parse correctly");
        assert_eq!(key_id.algorithm().as_str(), "ed25519");
        assert_eq!(key_id.key_name(), "MYDEVICE");
    }

    #[test]
    fn empty_key_name_is_correctly_extracted() {
        let key_id = DeviceKeyId::parse("ed25519:").expect("Should parse correctly");
        assert_eq!(key_id.algorithm().as_str(), "ed25519");
        assert_eq!(key_id.key_name(), "");
    }

    #[test]
    fn missing_colon_fails_to_parse() {
        let error = DeviceKeyId::parse("ed25519_MYDEVICE").expect_err("Should fail to parse");
        assert_matches!(error, Error::MissingColon);
    }

    #[test]
    fn empty_algorithm_fails_to_parse() {
        let error = DeviceKeyId::parse(":MYDEVICE").expect_err("Should fail to parse");
        // Weirdly, this also reports MissingColon
        assert_matches!(error, Error::MissingColon);
    }
}
