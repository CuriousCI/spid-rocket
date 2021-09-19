use xml;

pub enum Binding {
	POST,
	REDIRECT,
}

impl Binding {
	fn value(&self) -> &'static str {
		format!(
			"urn:oasis:names:tc:SAML:2.0:bindings:{}",
			match *self {
				Binding::POST => "HTTP-POST",
				Binding::REDIRECT => "HTTP-Redirect",
			}
		)
	}
}

pub struct SingleSignOnService {
	pub location: &'static str,
	pub binding: Binding,
}

pub struct SingleLogoutService {
	pub location: &'static str,
	pub binding: Binding,
	pub response_location: Option<&'static str>,
}

pub struct IdentityProvider {
	pub entity_id: &'static str,

	pub protocol_support_enumeration: Vec<String>,
	pub want_auth_request_signed: bool,

	pub key_descriptor: Vec<String>,

	format: &'static str,

	pub sign_on_services: Vec<SingleSignOnService>,
	pub logout_services: Vec<SingleLogoutService>,

	pub organization_name: &'static str,
	pub organization_url: &'static str,
}

// impl Default for IdentityProvider {
// 	fn default() -> Self {
// 		IdentityProvider {
// 			entity_id: Default::default(),

// 			protocol_support_enumeration: Default::default(),
// 			want_auth_request_signed: true,

// 			key_descriptor: Default::default(),

// 			format: "urn:oasis:names:tc:SAML:2.0:nameidformat:transient",

// 			organization_name: Default::default(),
// 			organization_url: Default::default(),
// 		}
// 	}
// }

impl IdentityProvider {
	pub fn xml() {}
}
