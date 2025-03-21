# [unreleased]

Breaking changes:

* Update `send_{message,state}_event::Request::new`'s `content` parameters to be
  generic, such that custom events can easily be sent
  * To migrate, simply stop wrapping content structs in `AnyMessageEventContent`
    before passing them to those constructors

# 0.4.0

Breaking changes:

* Upgrade ruma-state-res to 0.4.0
  * If you are not using state-res, there is no need to upgrade

# 0.3.0

Breaking changes:

* Upgrade sub-crates. The relevant breaking changes can be found in the changelogs of
  * ruma-events 0.24.1 (0.24.0 was yanked)
  * ruma-appservice-api 0.4.0
  * ruma-client-api 0.12.0
  * ruma-federation-api 0.3.0
  * ruma-identity-service-api 0.3.0
  * ruma-push-gateway-api 0.3.0
  * ruma-signatures 0.9.0
  * ruma-state-res 0.3.0

# 0.2.0

Breaking changes:

* Upgrade sub-crates. The relevant breaking changes can be found in the changelogs of
  * ruma-events 0.23.0
  * ruma-appservice-api 0.3.0
  * ruma-client-api 0.11.0
  * ruma-federation-api 0.2.0
  * ruma-identity-service-api 0.2.0
  * ruma-push-gateway-api 0.2.0
  * ruma-signatures 0.8.0
  * ruma-state-res 0.2.0

# 0.1.2

Improvements:

* Bump version of `ruma-common` and `ruma-client-api`, switching the canonical
  location of `ThirdPartyIdentifier`
  (now `ruma::thirdparty::ThirdPartyIdentifier`)

  For backwards compatibility, it is still available at the old path in
  `ruma::client::api::r0::contact::get_contacts`

# 0.1.1

Improvements:

* Bump versions of `ruma-client-api` and `ruma-events` for unstable spaces
  support

# 0.1.0

First release with non-prerelease dependencies! 🎉
