/* Reccommend build with "go build -tags -netgo"
 Reference: https://www.arp242.net/static-go.html
 Reference: https://community.tmpdir.org/t/problem-with-go-binary-portability-lib-x86-64-linux-gnu-libc-so-6-version-glibc-2-32-not-found/123

 Slightly smaller binary go build -ldflags="-w -s"
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

const TestDebug = true

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
		log.Fatal("Configuration file has insufficient chunks.")
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

	/*
	   if TestDebug == True {
	       fmt.Printf("Base URL => %s\n", baseURL)
	       fmt.Printf("Zipcode is => %q\n", r.Zipcode)
	       fmt.Printf("License Plate is => %q\n", r.Data)
	   }
	*/

	// Explicitfy the separators for easier
	QueryURL := string(baseURL) + "0/" + r.Zipcode + "/" + r.Data + "/1/25/"
	fmt.Println("Target URL", QueryURL)

	var data []byte

	if TestDebug == false {
		var err error
		resp, err := http.Get(QueryURL)
		if err != nil {
			log.Fatalf("Error on URL request.", err)
		}

		data, err = io.ReadAll(resp.Body)
		if err != nil {
			log.Fatalf("Error on io read. ", err)
		}

		if resp.StatusCode > 299 {
			log.Printf("Response failed with StatusCode: %d\n Body: %s\n\n", resp.StatusCode, data)
		}
		resp.Body.Close()

	} else {

		// Read in sample.json since no current tolls exist
		file, err := os.Open("sample.json")
		defer file.Close()
		if err != nil {
			log.Fatal("Failed to open sample.json")
		}

		data, err = io.ReadAll(file)

	}

	// Check length of bytes here.
	// Check number of records
	// Return nil + send  email as response length mayve changed
	fmt.Println("Data Retrieved as JSON: ")
	time.Sleep(time.Duration(rand.Intn(4)) * time.Second)

	message := SearchJSONResponse(data)
	if message != nil {
		SendMail(message, r.Email)
	}

	return
}

func SearchJSONResponse(data []byte) *string {

	// OLD Alternative versions of attempting to use GJSON to get values
	/*
	   result := gjson.ParseBytes(data)
	   fmt.Println("The number of notices is ", len(result))
	   result.ForEach(func(key, value gjson.Result) bool {
	       println("key ->", key.String())
	       fmt.Println("val ->", gjson.Get(value.String(), "itemDescription"))
	       fmt.Println("val ->", gjson.Get(value.String(), "formattedTotal"))
	       return true // iterate to the next
	   })
	*/
	/*
	   // discard the first which is the ordinal/key, retain the list(s) in the next
	    for _, b := range results {
	       fmt.Println()
	       fmt.Printf("Len of b:  \n", b.Indexes)
	       //fmt.Printf("Type of b: %T \n", b)
	       //fmt.Println("Explanation of Results -> ", b)
	   }
	*/

	results := gjson.GetManyBytes(data, "#.itemDescription", "#.formattedTotal")
	last := (len(results[0].Array()) - 1)

	if strings.EqualFold(results[0].Array()[last].String(), "Total Amount Due") {

		msg := fmt.Sprintf("The %s is %s from %d tolls", results[0].Array()[last].String(), results[1].Array()[last], last+1)

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
			"Subject: Tolls Due!\r\n" +
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

func main() {

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
