#[macro_use]
extern crate rocket;
use spid::test;

#[get("/")]
fn index() -> &'static str {
	test()
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index])
}
