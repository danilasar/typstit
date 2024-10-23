use crate::api::PackageNameList;

pub fn get_package_name_list(args: Vec<String>, required: bool, error_message: &str) -> std::io::Result<crate::api::PackageNameList> {
	if args.len() < 3 {
		if !required {
			return Ok(PackageNameList(Vec::new()));
		}
		log::error!("{error_message}");
		return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
	}
	let packages = crate::api::PackageNameList(
		args[2..]
			.iter()
			.filter(|s| !s.starts_with("-"))
			.map(|x| crate::api::PackageName(x.clone()))
			.collect());
	if required && packages.0.is_empty() {
		log::error!("{error_message}");
		return Err(std::io::Error::from(std::io::ErrorKind::InvalidInput));
	}
	Ok(packages)
}