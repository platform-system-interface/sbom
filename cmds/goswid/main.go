package main

import (
	"bytes"
	"encoding/binary"
	"errors"
	"flag"
	"fmt"
	"github.com/kr/pretty"
	"github.com/veraison/swid"
	"io"
	"io/ioutil"
	"log"
)

var (
	Magic string = "\x53\x42\x4F\x4D\xD6\xBA\x2E\xAC\xA3\xE6\x7A\x52\xAA\xEE\x3B\xAF"
)

type uSWID struct {
	Magic        [16]byte
	Header_ver   uint8
	Header_size  uint16
	Payload_size uint16
	Payload      []byte
}

var errEOF = errors.New("unexpected EOF while parsing fmap")

func readField(r io.Reader, data interface{}) error {
	// The endianness might depend on your machine or it might not.
	if err := binary.Read(r, binary.LittleEndian, data); err != nil {
		return errEOF
	}
	return nil
}

func NewSwid(data []byte) (uSWID, int) {
	pos := bytes.Index(data, []byte(Magic))

	in := bytes.NewReader(data[pos:])
	var uswid uSWID
	err := readField(in, &uswid.Magic)
	if err != nil {
		log.Fatal(err)
	}
	err = readField(in, &uswid.Header_ver)
	if err != nil {
		log.Fatal(err)
	}
	err = readField(in, &uswid.Header_size)
	if err != nil {
		log.Fatal(err)
	}
	err = readField(in, &uswid.Payload_size)
	if err != nil {
		log.Fatal(err)
	}
	uswid.Payload = make([]byte, uswid.Payload_size)
	in.Seek(int64(uswid.Header_size), io.SeekStart)
	_, err = in.Read(uswid.Payload)
	if err != nil {
		log.Fatal(err)
	}
	return uswid, pos
}

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
		uswid, pos := NewSwid(data)
		fmt.Printf("magic %x at: %x\n", Magic, pos)
		fmt.Printf("uswid:\n  Version: %v\n  Header size: %v\n  Payload size: %v\n", uswid.Header_ver, uswid.Header_size, uswid.Payload_size)

		var tag swid.SoftwareIdentity
		if err := tag.FromCBOR(uswid.Payload); err != nil {
			log.Fatal(err)
		}

		fmt.Printf("%# +v \n", pretty.Formatter(tag))

		if payloads := tag.Payload; payloads != nil {
			fmt.Printf("%v", payloads)
		}
	}
}
