/* Reccommend build with "go build -tags -netgo"
 Reference: https://www.arp242.net/static-go.html
 Reference: https://community.tmpdir.org/t/problem-with-go-binary-portability-lib-x86-64-linux-gnu-libc-so-6-version-glibc-2-32-not-found/123

 Slightly smaller binary: go build -ldflags="-w -s"
 Even more small binary: upx -k --brute gozpassmd
*/

package main

import (
	"bufio"
	"encoding/base64"
	"fmt"
	"github.com/tidwall/gjson"
	"io"
	"log"
	"math/rand"
	"net/http"
	"net/smtp"
	"os"
	"strings"
	"time"
)

var TestDebug, NoMail bool = false, false

const QueryAPI string = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGF5bWVudC9QZW5kaW5nLw=="

type Record struct {
	Type    string
	Data    string
	State   string
	Zipcode string
	Email   []string
}

func CreateRecordFromConfig(cfgdata string) *Record {

	var rec *Record
	var chunks []string

	chunks = strings.Split(cfgdata, "||")

	if len(chunks) == 5 {
		for i := range chunks {
			chunks[i] = strings.TrimSpace(chunks[i])
		}
	} else {
		log.Fatal("config file has insufficient chunks.")
	}

	/*
	   switch r.Type {
	       case "PLATE":
	           fmt.Println("Plate")
	           new()
	       case "LIC":
	           fmt.Println("Drivers License")
	       case "MAIL":
	           fmt.Println("Mailing Notice")
	       case "DEVICE":
	           fmt.Println("Device")
	       default:
	           log.Fatal("Unable to determine record type.")
	   } //end switch
	*/

	rec = &Record{chunks[0], chunks[1], chunks[2], chunks[3], strings.Split(chunks[4], ",")}

	return rec

}

func QueryNotice(r *Record) {

	// Currently built only to to use the License plate + zip

	baseURL, _ := base64.StdEncoding.DecodeString(QueryAPI)


	   if TestDebug == true {
	       fmt.Printf("Base URL => %s\n", baseURL)
	       fmt.Printf("Zipcode is => %q\n", r.Zipcode)
	       fmt.Printf("License Plate is => %q\n", r.Data)
	   }


	// Explicitfy the separators for easier
	QueryURL := string(baseURL) + "0/" + r.Zipcode + "/" + r.Data + "/1/25/"
	fmt.Println("Target URL", QueryURL)

	var data []byte

	// If we're not in Test Mode then dont look for a sample.json file
	if TestDebug == false {
		var err error
		resp, err := http.Get(QueryURL)
		if err != nil {
			log.Fatalf("Error on URL request.", err)
			SendErrorMail(err.Error(), r.Email)
		}

		data, err = io.ReadAll(resp.Body)
		if err != nil {
			log.Fatalf("Error on io read. ", err)
			SendErrorMail(err.Error(), r.Email)
		}

		if resp.StatusCode > 299 {
			// local 'err' created as 'err' is nil as we do get a Response if it reaches here and will segfault
			err := fmt.Errorf("Response failed with StatusCode: %d\n Body: %s\n\n", resp.StatusCode, data)
			log.Println(err.Error())
			SendErrorMail(err.Error(), r.Email)
		}
		resp.Body.Close()

	} else {

		log.Println("Utilizing local sample.json file")

		// Read in sample.json since no current tolls exist
		file, err := os.Open("sample.json")
		defer file.Close()
		if err != nil {
			log.Fatal("Failed to open sample.json")
		}

		data, err = io.ReadAll(file)
                fmt.Printf("Data JSON read:", string(data))

	}

	// Check length of bytes here.
	// Check number of records
	// Return nil + send  email as response length mayve changed
	time.Sleep(time.Duration(rand.Intn(4)) * time.Second)

	message := SearchJSONResponse(data)
	if message != nil {
		SendMail(message, r.Email)
	}

	return
}

func SearchJSONResponse(data []byte) *string {

	results := gjson.GetManyBytes(data, "#.itemDescription", "#.formattedTotal")
	last := (len(results[0].Array()) - 1)

	// When the value of last will be -1 (0 find minus 1) and cause a bounds fail
	//  Possibly eedesign later we already catch non-200 errors above and SendErrMail
	// Though need to be malleable for unknown JSON changes in the future

	if (last != -1) && (strings.EqualFold(results[0].Array()[last].String(), "Total Amount Due")) {

		msg := fmt.Sprintf("The %s is %s from %d tolls", results[0].Array()[last].String(), results[1].Array()[last], last+1)

                // Add support for "-nomail" flag to print to standard output
                println(msg)

		return &msg

	} else {

		return nil

	}

}

func MakePayment() {

}

func SendMail(message *string, emailto []string) {

	var (
		from = "banana@example.net"

		msg = []byte("To: " + from + "\r\n" +
			"Subject: Tolls Alert!\r\n" +
			"\r\n" +
			*message + "\r\n")
	)

	err := smtp.SendMail("127.0.0.1:25", nil, from, emailto, msg)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Message Dispatched: ", *message)
	return
}

func SendErrorMail(errmessage string, emailto []string) {

        println("The '-nomail' flag is set to ->", NoMail)
        if NoMail { return }
	var (
		from = "banana@example.net"

		msg = []byte("To: " + from + "\r\n" +
			"Subject: Tolls Error Issue\r\n" +
			"\r\n" + "Error Occurred in Script:\n\t" +
			errmessage + "\r\n")
	)

	err := smtp.SendMail("127.0.0.1:25", nil, from, emailto, msg)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Error Message Email Dispatched: ", errmessage)
	return
}

func main() {

	// Rudimentary Debug Mode Flag
	println("syntax: gozpassmd [-testdebug] - run in debugmode with sample.json gozpassmd -nomail")
	println("syntax: gozpassmd [-nomail] - run normal but dont send emails")
	if len(os.Args) == 2 {
	    if os.Args[1] == "-testdebug" {
	        TestDebug = true
	    } else if os.Args[1] == "-nomail" {
	        NoMail = true
	    }
	}

	recs := make([]*Record, 0)

	file, err := os.Open("ezpstore.txt")
	defer file.Close()
	if err != nil {
		log.Fatal("Failed to open ezpstore.txt")
	}

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if strings.HasPrefix(line, "#") || len(line) == 0 {
			continue
		}
		recs = append(recs, CreateRecordFromConfig(line))
	}
	fmt.Printf("%d num of Records created from config.\n", len(recs))

	// Concurrency and channels. Esp if you make the method a receiver
	for _, p := range recs {
		QueryNotice(p)
	}

}
