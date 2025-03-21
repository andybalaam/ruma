//! Types for the `m.call.invite` event.

use js_int::UInt;
use ruma_events_macros::EventContent;
use serde::{Deserialize, Serialize};

use super::SessionDescription;

/// The content of an `m.call.invite` event.
///
/// This event is sent by the caller when they wish to establish a call.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.call.invite", kind = Message)]
pub struct CallInviteEventContent {
    /// A unique identifier for the call.
    pub call_id: String,

    /// The time in milliseconds that the invite is valid for.
    ///
    /// Once the invite age exceeds this value, clients should discard it. They should also no
    /// longer show the call as awaiting an answer in the UI.
    pub lifetime: UInt,

    /// The session description object.
    ///
    /// The session description type must be *offer*.
    pub offer: SessionDescription,

    /// The version of the VoIP specification this messages adheres to.
    pub version: UInt,
}

impl CallInviteEventContent {
    /// Creates a new `InviteEventContent` with the given call ID, lifetime and VoIP version.
    pub fn new(call_id: String, lifetime: UInt, offer: SessionDescription, version: UInt) -> Self {
        Self { call_id, lifetime, offer, version }
    }
}
