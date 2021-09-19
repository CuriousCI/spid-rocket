use chrono::naive::NaiveDate;

pub enum Attribute {
	SPID_CODE,
}

impl Attribute {
	fn name(&self) -> &'static str {
		match *self {
			Attribute::SPID_CODE => "spidCode",
		}
	}

	fn xsi_type(&self) -> &'static str {
		match *self {
			Attribute::SPID_CODE => "xs:string",
		}
	}
}

// #[derive(Default)]
pub struct Attributes {
	pub spid_code: String,
	pub name: String,
	pub family_name: String,
	pub place_of_birth: String,
	pub country_of_birth: String,
	pub date_of_birth: NaiveDate,
	pub gender: String,
	pub company_name: String,
	pub registered_office: String,
	pub fiscal_number: String,
	pub iva_code: String,
	pub id_card: String,
	pub mobile_phone: String,
	pub email: String,
	pub domicile_street_address: String,
	pub domicile_postal_code: String,
	pub domicile_municipality: String,
	pub domicile_province: String,
	pub address: String,
	pub domicile_nation: String,
	pub expiration_date: NaiveDate,
	pub digital_address: String,
}
