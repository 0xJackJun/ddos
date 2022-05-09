package service

func Watch() {
	items := GetDdosList()
	for _, item := range items {
		SendMail(item.Email)
	}
}
