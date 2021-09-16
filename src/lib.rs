mod attributes;
mod idp;
mod sp;

pub use self::attributes::Attributes;
pub use self::idp::IdentityProvider;
pub use self::sp::ServiceProvider;

pub fn test() -> &'static str {
	"Hello, SPID!"
}
