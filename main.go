package main

import (
	"flag"
	"fmt"
	"os"
	"time"
)

var url = flag.String("url", "", "input url path")

func main() {
	flag.Parse()

	if *url == "" {
		fmt.Printf("please enter a url  ex: -url sample-url\n")
	}

	if *url != "" {
		directoryPath := "content/" + *url
		if err := os.Mkdir(directoryPath, 0777); err != nil {
			fmt.Println(err)
			os.Exit(1)
		}
		file, err := os.Create(directoryPath + "/index.mdx")
		if err != nil {
			os.Exit(1)
		}
		defer file.Close()

		output := `---
title:
date: "%v"
tags: []
spoiler:
---
`
		file.Write(([]byte)(fmt.Sprintf(output, time.Now().Format(time.RFC3339))))
	}
}
