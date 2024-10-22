pub enum Action {
	// информация о пакете
	Info,
	// установить пакет
	Install,
	// обновить пакет
	Update,
	// обновить все пакеты
	UpdateAll,
	// добавить пакет в typst документ
	Add,
	// убрать пакет из typst документа
	Remove,
	// удалить пакет
	Uninstall,
	// ничего не делать
	Nothing
}