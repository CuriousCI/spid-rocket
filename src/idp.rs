pub enum SignOnBinding {
	REDIRECT,
	POST,
}

pub enum LogoutBinding {
	SOAP,
	REDIRECT,
	POST,
}

pub struct IdentityProvider {
	pub entity_id: &'static str,

	pub protocol_support_enumeration: Vec<String>,
	pub want_auth_request_signed: bool,

	pub key_descriptor: Vec<String>,

	format: &'static str,

	pub sign_on_location: &'static str,
	pub sign_on_binding: SignOnBinding,

	pub logout_location: &'static str,
	pub logout_binding: LogoutBinding,

	pub organization_name: &'static str,
	pub organization_url: &'static str,
}

impl Default for IdentityProvider {
	fn default() -> Self {
		IdentityProvider {
			entity_id: Default::default(),

			protocol_support_enumeration: Default::default(),
			want_auth_request_signed: true,

			key_descriptor: Default::default(),

			format: "urn:oasis:names:tc:SAML:2.0:nameidformat:transient",

			sign_on_location: Default::default(),
			sign_on_binding: SignOnBinding::REDIRECT,

			logout_location: Default::default(),
			logout_binding: LogoutBinding::POST,

			organization_name: Default::default(),
			organization_url: Default::default(),
		}
	}
}

impl IdentityProvider {
	pub fn xml() {}
}
