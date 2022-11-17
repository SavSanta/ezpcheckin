package main


import (

	"os"
	"fmt"
	"strings"
	"encoding/base64"
	"net/http"
	"math/rand"
//	"github.com/tidwall/gjson"
	"log"
	"bufio"
	"net/smtp"

)

const QueryAPI string = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGF5bWVudC9QZW5kaW5nLw=="

type Record struct {
    Type string
    Data string
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

    rand.Intn(100)
    baseURL, _ := base64.StdEncoding.DecodeString(QueryAPI)
    
    /*
    if flags.Debug {
        fmt.Printf("Base URL => %s\n", baseURL)
        fmt.Printf("Zipcode is => %q\n", r.Zipcode)
        fmt.Printf("License Plate is => %q\n", r.Data)
    }
    */

    // Explicitfy the separators for easier
    QueryURL := string(baseURL) + "0/" + r.Zipcode + "/" + r.Data + "/1/25/"

    fmt.Println("Final built string", QueryURL)

    resp, err := http.Get("https://example.com")
    if err != nil {
        panic(err)
    }
    defer resp.Body.Close()

    return
}


func MakePayment() {


}


func SendMail(r *Record) {

    var (
	    from       = "banana@example.net"
	    msg        = []byte("Hey, baby I got ya money!")
	    recipients = r.Email
    )

     hostname := "mail.example.com"
     auth := smtp.PlainAuth("", "user@example.com", "password", hostname)

     err := smtp.SendMail(hostname+":25", auth, from, recipients, msg)
     if err != nil {
        log.Fatal(err)
     }

    return
}


func main() {

    recs := make([]*Record, 0)

    file, err := os.Open("ezpstore.txt")
    defer file.Close()
    if err != nil {
        log.Fatalf("Failed to open ezpstore.txt")
    }

    scanner := bufio.NewScanner(file)

    for scanner.Scan() {
        line := strings.TrimSpace(scanner.Text())
        if (strings.HasPrefix(line, "#") || len(line) == 0) {
            continue
        }
        recs = append(recs, CreateRecordFromConfig(line))      
    }
    fmt.Printf("%d number of Records created.\n\n", len(recs))

    // Concurrency and channels. Esp if you make the method a receiver
    for _, p := range recs {
        QueryNotice(p)
    }

}
