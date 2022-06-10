package main

import (
	"fmt"
	"time"
	"watch/service"

	"github.com/robfig/cron"
)

func main() {
	c := cron.New()

	err := c.AddFunc("@every 20s", service.Watch)
	if err != nil {
		fmt.Println(err)
	}
	c.Start()
	for {
		time.Sleep(time.Second)
	}
}
