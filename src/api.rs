#[derive(Clone, Debug)]
pub struct PackageName(pub String);
#[derive(Debug)]
pub struct PackageNameList(pub Vec<PackageName>);
#[derive(Debug)]
pub enum Request {
	// информация о пакете
	Info(PackageName),
	// установить пакет
	Install(PackageNameList),
	// обновить пакет
	Update(PackageNameList),
	// обновить все пакеты
	UpdateAll,
	// добавить пакеты в typst документ
	Add(PackageNameList),
	// убрать пакеты из typst документа
	Remove(PackageNameList),
	// удалить пакеты
	Uninstall(PackageNameList),
	// ничего не делать
	Nothing
}

pub enum Response {
	Nothing
}