package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"log"

	"github.com/kr/pretty"
	"github.com/veraison/swid"
)

func main() {
	flag.Parse()
	args := flag.Args()

	var path string

	if len(args) > 0 {
		path = args[0]
		data, err := ioutil.ReadFile(path)
		if err != nil {
			log.Fatal(err)
		}
		var tag swid.SoftwareIdentity
		if err := tag.FromJSON(data); err != nil {
			log.Fatal(err)
		}

		fmt.Printf("%# +v \n", pretty.Formatter(tag))

		if payloads := tag.Payloads; payloads != nil {
			fmt.Printf("%v", payloads)
		}
	}
}
