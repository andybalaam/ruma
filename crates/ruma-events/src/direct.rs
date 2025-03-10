//! Types for the `m.direct` event.

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use ruma_events_macros::EventContent;
use ruma_identifiers::{RoomId, UserId};
use serde::{Deserialize, Serialize};

/// The content of an `m.direct` event.
///
/// A mapping of `UserId`s to a list of `RoomId`s which are considered *direct* for that particular
/// user.
///
/// Informs the client about the rooms that are considered direct by a user.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[allow(clippy::exhaustive_structs)]
#[ruma_event(type = "m.direct", kind = GlobalAccountData)]
pub struct DirectEventContent(pub BTreeMap<Box<UserId>, Vec<Box<RoomId>>>);

impl Deref for DirectEventContent {
    type Target = BTreeMap<Box<UserId>, Vec<Box<RoomId>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DirectEventContent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use ruma_identifiers::{server_name, RoomId, UserId};
    use serde_json::{from_value as from_json_value, json, to_value as to_json_value};

    use super::{DirectEvent, DirectEventContent};

    #[test]
    fn serialization() {
        let mut content = DirectEventContent(BTreeMap::new());
        let server_name = server_name!("ruma.io");
        let alice = UserId::new(server_name);
        let room = vec![RoomId::new(server_name)];

        content.insert(alice.clone(), room.clone());

        let event = DirectEvent { content };
        let json_data = json!({
            "content": {
                alice.to_string(): vec![room[0].to_string()],
            },
            "type": "m.direct"
        });

        assert_eq!(to_json_value(&event).unwrap(), json_data);
    }

    #[test]
    fn deserialization() {
        let server_name = server_name!("ruma.io");
        let alice = UserId::new(server_name);
        let rooms = vec![RoomId::new(server_name), RoomId::new(server_name)];

        let json_data = json!({
            "content": {
                alice.to_string(): vec![rooms[0].to_string(), rooms[1].to_string()],
            },
            "type": "m.direct"
        });

        let event: DirectEvent = from_json_value(json_data).unwrap();
        let direct_rooms = event.content.get(&alice).unwrap();

        assert!(direct_rooms.contains(&rooms[0]));
        assert!(direct_rooms.contains(&rooms[1]));
    }
}
