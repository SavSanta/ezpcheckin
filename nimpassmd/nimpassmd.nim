
import random
import times
import strformat
import strutils
import json
import std/httpclient
import std/logging
import std/sequtils
#import std/smtp       # For some reason theyre forcing you to use a compiled with SSL in this language as if.. :eyeroll


type Record = object
  Type: string
  Data: string
  State: string
  Zip: string

proc CreateRecordFromConfig(cfgdata: string): Record =

    var rec = Record()

    rec.Type = "License"
    rec.Data = "KickinDaggers"
    rec.State = "MD"
    rec.Zip = "21227"

    echo "gang gang -->", rec


    return rec



#proc QueryNoticeAPI()


#proc SearchJSONResponse()








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

      recs.add(CreateRecordFromConfig("hihater"))
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



