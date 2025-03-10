//! [POST /_matrix/federation/v1/user/keys/query](https://matrix.org/docs/spec/server_server/r0.1.4#post-matrix-federation-v1-user-keys-query)

use std::collections::BTreeMap;

use ruma_api::ruma_api;
#[cfg(feature = "unstable-pre-spec")]
use ruma_common::encryption::CrossSigningKey;
use ruma_common::encryption::DeviceKeys;
use ruma_identifiers::{DeviceId, UserId};

ruma_api! {
    metadata: {
        description: "Returns the current devices and identity keys for the given users.",
        method: POST,
        name: "get_keys",
        path: "/_matrix/federation/v1/user/keys/query",
        rate_limited: false,
        authentication: ServerSignatures,
    }

    request: {
        /// The keys to be downloaded.
        ///
        /// Gives all keys for a given user if the list of device ids is empty.
        pub device_keys: BTreeMap<Box<UserId>, Vec<Box<DeviceId>>>,
    }

    #[derive(Default)]
    response: {
        /// Keys from the queried devices.
        pub device_keys: BTreeMap<Box<UserId>, BTreeMap<Box<DeviceId>, DeviceKeys>>,

        /// Information on the master cross-signing keys of the queried users.
        #[cfg(feature = "unstable-pre-spec")]
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        pub master_keys: BTreeMap<Box<UserId>, CrossSigningKey>,

        /// Information on the self-signing keys of the queried users.
        #[cfg(feature = "unstable-pre-spec")]
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        pub self_signing_keys: BTreeMap<Box<UserId>, CrossSigningKey>,
    }
}

impl Request {
    /// Creates a new `Request` asking for the given device keys.
    pub fn new(device_keys: BTreeMap<Box<UserId>, Vec<Box<DeviceId>>>) -> Self {
        Self { device_keys }
    }
}

impl Response {
    /// Creates a new `Response` with the given device keys.
    pub fn new(device_keys: BTreeMap<Box<UserId>, BTreeMap<Box<DeviceId>, DeviceKeys>>) -> Self {
        Self { device_keys, ..Default::default() }
    }
}
