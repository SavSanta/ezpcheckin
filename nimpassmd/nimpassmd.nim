
import random
import times
import strformat
import strutils
import json
import std/httpclient
import std/logging
import std/sequtils
from std/base64 import decode
#import std/smtp       # For some reason theyre forcing you to use a compiled with SSL in this language as if.. :eyeroll

var testDebug = true

proc SearchJSONResponse(jdata: string)
let queryAPI = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGF5bWVudC9QZW5kaW5nLw"

type Record = object
  Type: string
  Data: string
  State: string
  Zip: string
  Email: string

proc CreateRecordFromConfig(cfgdata: seq[string]): Record =

    var rec = Record()

    rec.Type = cfgdata[0]
    rec.Data = cfgdata[1]
    rec.State = cfgdata[2]
    rec.Zip = cfgdata[3]
    rec.Email = cfgdata[4]

    if testDebug == true:
      echo "gang gang -->", rec

    return rec



proc QueryNoticeAPI(r: Record) =

    # Currently built to use only the LIC type   
    var
      baseURL: string
      QueryURL: string 
    
    baseURL = decode(queryAPI)

    if testDebug == true:
      echo "Base URL => ", baseURL
      echo "Zipcode is => ", r.Zip
      echo "License Plate is => ", r.Data

    # Explicitfy the separators for ease
    QueryURL = baseURL & "0/" & r.Zip & "/" & r.Data & "/1/25/"
    echo "Target URL: ", QueryURL

    var
      f: File
      jdata: string


    discard open(f, "sample.json")
    let jsondata = readFile("sample.json")
    f.close()
    SearchJSONResponse(jsondata)


proc SearchJSONResponse(jdata: string) =

    var cnt: int
    var msg: string

    let pObj = parseJson(jdata)
    let arrSize = pObj.len()
    
    echo "Array Object Size: ", arrSize

    while cnt < arrSize:
      echo "- ", pObj[cnt]["itemDescription"].getStr(), " ",pObj[cnt]["formattedTotal"].getStr()
      inc(cnt)

    if arrSize-1 > 0:
      echo "Final ", pObj[arrSize-1]["itemDescription"].getStr()," ",pObj[arrSize-1]["formattedTotal"].getStr()


when isMainModule:

# Open File For Reads Cookbook
  var
    f: File
    recs: seq[Record]
    line: string
    chunks: seq[string]
    numRecs: int

if open(f, "ezpstore.txt"):
  try:
    echo "=== Reading ezpstore.txt ==="

    while f.readLine(line):
      if line.strip().startswith("#") or line.strip() == "":
        continue
      chunks = line.strip(leading = true).split("||")
      if chunks.len() != 5:
        raise newException(CatchableError, "Insufficient chunks derived from config file. Verify Construction.")
      apply(chunks, proc(x: string): string = x.strip())
      
      recs.add(CreateRecordFromConfig(chunks))
      numRecs += 1

  except IOError:
    echo "Input/Output Error while reading ezpstore.txt"
  except CatchableError as e:
    echo "Unexpected Error: " & e.msg
  finally:
    echo "Number of successful records ingested from config: ",numRecs
    close(f)
else:
  raise newException(CatchableError, "Couldnt not read/access ezpstore.txt file.")


if testDebug == true:
  echo recs

QueryNoticeAPI(recs[0])
