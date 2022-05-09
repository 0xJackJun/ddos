package service

import (
	"fmt"
	"io/ioutil"
	"log"
	"os/exec"
	"regexp"
	"watch/model"
)

func GetDdosList() []model.Ddos {
	// local command
	args := []string{"canister", "call", "ryjl3-tyaaa-aaaaa-aaaba-cai", "get_ddos"}
	cmd := exec.Command("dfx", args...)

	stdout, err := cmd.StdoutPipe()
	if err != nil {
		log.Fatal(err)
	}
	defer stdout.Close()
	if err := cmd.Start(); err != nil {
		log.Fatal(err)
	}
	opBytes, err := ioutil.ReadAll(stdout)
	if err != nil {
		log.Fatal(err)
	}
	return Regular(string(opBytes))
}

func Regular(op string) []model.Ddos {
	reg1 := regexp.MustCompile(`"(.*?)"`)
	if reg1 == nil {
		fmt.Println("err = ")
		return nil
	}

	result := reg1.FindAllStringSubmatch(op, -1)
	users := make([]model.Ddos, 0, 2)
	for i := 0; i < len(result); i += 1 {
		user := model.Ddos{}
		user.Email = result[i][1]
		users = append(users, user)
	}
	return users
}
