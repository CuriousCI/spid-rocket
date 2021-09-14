mod attributes;

pub use self::attributes::Attributes;

pub fn test() -> &'static str {
	"Hello, SPID!"
}

pub fn hello() {
	Attributes::test();
}
