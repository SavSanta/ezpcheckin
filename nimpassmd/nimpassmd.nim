# Nim Compiler Version 1.6.6 [Linux: amd64]
# Compiled at 2022-05-05
# Copyright (c) 2006-2021 by Andreas Rumpf
# git hash: 0565a70eab02122ce278b98181c7d1170870865c
# nim -c -d:ssl -d:strip -d:release nimpassmd       # Smallest Linux Binary 
# nim -c -d:ssl -d:strip nimpassmd        # Second smallest Linux binary
# nim c -d:ssl -d=mingw -d:danger -d:strip --opt:size --passL:-Wl,--dynamicbase --app:console -o:nimpasswin nimpassmd  # Windows binary

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

var testDebug = false

proc SearchJSONResponse(jdata: string)
let queryAPI = "aHR0cHM6Ly9jc2MuZHJpdmVlem1kLmNvbS9hcGkvUGF5VG9sbHMvUGVuZGluZ1BheW1lbnRzVG90YWwv"

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
      echo "Record Creating: -->", rec

    return rec



proc QueryNoticeAPI(r: Record) =

    # Currently built to use only the LIC type
    var
      baseURL: string
      QueryURL: string
      jsondata: string

    baseURL = decode(queryAPI)

    if testDebug == true:
      echo "Base URL => ", baseURL
      echo "Zipcode is => ", r.Zip
      echo "License Plate is => ", r.Data

    # Explicitfy the separators for ease
    QueryURL = baseURL & "0/" & r.Zip & "/" & r.Data & "/1/25/" & "0/"
    echo "Target URL: ", QueryURL


    if testDebug == true:
      # TODO Clean this junk up eventually to use a try/except/finally
      var f: File
      discard open(f, "sample.json")
      jsondata = readFile("sample.json")
      f.close()
    else:
      # TODO Also clean this crap up eventually to use a try/except/finally
      var client = newHttpClient(userAgent="Mozilla/5.0 (Linux; Android 11; Pixel 5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3485.133 Mobile Safari/537.36", timeout=27000)
      var resp = httpClient.Response()
      resp = client.get(QueryURL)
      client.close()

      if testDebug == true:
        echo "STATUS CODE: ", code(resp)
        echo "BODY: ", jsondata

      # TODO Eventually catch other error/status codes
      if resp.code() == HTTPCode(200):
        echo "Received status 200 response code."
        jsondata = resp.body()
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

    # TODO - So this works even if the arrSize is only 1 but can prob decide If i want to switchout to a 'arrSize > -1' compare. So far doesnt seem necessary
    if arrSize-1 > 0:
      echo "Final ", pObj[arrSize-1]["itemDescription"].getStr()," ",pObj[arrSize-1]["formattedTotal"].getStr()
      msg = "The " & pObj[arrSize-1]["itemDescription"].getStr() & " is " & pObj[arrSize-1]["formattedTotal"].getStr() & " from " & repr(arrSize) & "tolls."


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


QueryNoticeAPI(recs[0])
