use chrono::naive::NaiveDate;

pub struct Attributes {
	/// # [Attributi identificativi](https://docs.italia.it/italia/spid/spid-regole-tecniche/it/stabile/attributi.html#id1)

	/// Codice identificativo
	pub spid_code: String,

	/// Nome
	pub name: String,

	/// Cognome
	pub family_name: String,

	/// Luogo di nascita
	pub place_of_birth: String,

	/// Provincia di nascita
	pub country_of_birth: String,

	/// Data di nascita
	pub date_of_birth: NaiveDate,

	/// Sesso
	pub gender: String,

	/// Ragione o denominazione sociale
	pub company_name: String,

	/// Sede legale
	pub registered_office: String,

	/// Codice fiscale
	pub fiscal_number: String,

	/// Partita IVA
	pub iva_code: String,

	/// Documento d'identità
	pub id_card: String,

	/// # [Attributi secondari](https://docs.italia.it/italia/spid/spid-regole-tecniche/it/stabile/attributi.html#id2)
	/// Numero di telefono mobile
	pub mobile_phone: String,

	/// Indirizzo di posta elettronica
	pub email: String,

	/// Domicilio
	pub domicile_street_address: String,

	/// Codice Postale
	pub domicile_postal_code: String,

	/// Comune
	pub domicile_municipality: String,

	///Provincia
	pub domicile_province: String,

	/// Domicilio fisico
	pub address: String,

	/// Nazione
	pub domicile_nation: String,

	/// Data di scadenza identità
	pub expiration_date: NaiveDate,

	/// Domicilio digitale
	pub digital_address: String,
}

impl Attributes {
	pub fn test() -> &str {
		"test"
	}
	// pub fn new() -> Attributes {
	// 	Attributes {
	// 		name: "Mario",
	// 		family_name: "Rossi",
	// 	}
	// }
}
