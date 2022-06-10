package service

import "fmt"

func Watch() {
	items := GetDdosList()
	for _, item := range items {
		fmt.Println(item.Email)
		SendMail(item.Email)
	}
}
