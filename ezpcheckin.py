#!/usr/bin/env python3

# ezpcheckin.py
# Author: Savsanta (Ru Uba)
# Version: 2.0
# This script checks for kickback-tolls that may be (not) mailed to you in this godforsaken expensive state and country.
# This progam uses f-strings and requires Python 3.6 or greater.


import requests, bs4, base64, functools
from random import randint

# Define constants
URL = base64.decodestring(b"aHR0cHM6Ly93d3cuZXpwYXNzbWQuY29tL3ZlY3Rvci92aW9sYXRpb25zL3Zpb2xOb3RpY2VJbnF1aXJ5LmRvP2xvY2FsZT1lbl9VUyZmcm9tPUhvbWU=")


def check_resp(webpage):
    if webpage.status_code == 200:
        print("Response Check returned OK!\n")
        return
    elif (400 <= webpage.status_code < 500):
        print("Response Check returned failing 400 series.\n")
        raise Exception("400 Server Error Code. Aborting")
    elif (500 <= webpage.status_code < 600):
        print("Response Check returned failing 500 series. Site is having issues.\n")
        raise Exception("500 Server Error Code. Aborting")
    else:
        print(f"Unknown Error Code: {webpage.status_code}.\n")
        raise Exception("Unknown Response Code. Aborting")


def readstore(config="ezpstore.txt"):
    ''' Reads the lines for the EZPass accounts to check and returns non-blank/non-commented lines as a list variable '''

    items = []
    with open(config) as lookup:
        while True:
            line = lookup.readline()

            # Strip out newlines and comments
            if (line.lstrip().startswith('#')) or (line == '\n'):
                continue
            elif (not line):
               break
            else:
                items.append(line)

    return items


def request(*args, **kwargs):
    ''' Shadows over the requests.Request function just to provide consistent header '''

    USER_AGENT = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/79.0.3945.79 Chrome/79.0.3945.79 Safari/537.36"
    headers = {'User-Agent' : USER_AGENT}

    resp = requests.request(*args, **kwargs, headers=headers)
    return resp


def exceptmail(function):
    ''' Decorator that wraps and emails caught exceptions '''

    @functools.wraps(function)
    def wrapper(*args, **kwargs):
        try:
            # try to run fumoction
            return function(*args, **kwargs)
        except:
            # email the exception
            err = "Error: Exception in "
            err += function.__name__
            print(err)
            dispatchemail(err)
            raise
    return wrapper


def search_violation(cfgdata):
    ''' Preps data out to be sent in POST url form submission'''

    basedata = {
        'org.apache.struts.taglib.html.TOKEN': formtoken,
        'formid': 'frmViolationInquiry',
        'btnSearch.x': randint(0,70),
        'btnSearch.y': randint(0,18),
        'ctokenElem': formtoken,
       }

    basedata.update(cfgdata)
    resp = request('POST', URL, data=basedata)

    return resp


def convcfg(data):
    ''' Convert the data from configuration list into values to use in the violation search and email dispatching. '''

    d = {}

    # Separate the values on the config line
    chunks = [ x.strip() for x in data.split("||") ]

    if chunks[0] == 'PLATE':
        # Example values to populate
        # loginNumber = 2DLXEFM or S123456789 or L7564334578 | licenstate = MD | zipcode = 47450
        d.update({'loginType': 'plate', 'selectCreditCard':'new', 'loginNumber':chunks[1] , 'licenseState':chunks[2], 'zipCode':chunks[3]})
        return d
    elif chunks[0] == 'MAIL':
        d.update({'loginType': 'violation', 'loginNumber':chunks[1], 'zipCode':chunks[3]})
        return d
    elif chunks[0] == 'DEVICE':
        d.update({'loginType': 'transponder', 'loginNumber':chunks[1], 'zipCode':chunks[3]})
        return d
    elif chunks[0] == 'LIC':
        d.update({'loginType': 'plate', 'loginNumber':chunks[1] , 'licenseState':chunks[2], 'zipCode':chunks[3]})
        return d
    else:
        raise Exception("Error While Reading Configuration!! Check for blank lines or incorrect line structure!")


def get_errflash(content):
    ''' Retrieve the error flash alert text that appears at top of webpage and provids info hints '''

    # In future work in better alternative for multiple errflash messages with soup.find_all("td", "data")
    soup = bs4.BeautifulSoup(content.text, features='lxml')

    try:
       errflash = None
       errflash = soup.form.div.table.tbody.td.table.tr.td.table.td.next.next.next.text
    except AttributeError:
        pass

    # Test if the existence of an errflash is there and return it. Otherwise return False.
    if errflash:
        return errflash
    else:
        return None


def find_token(webpage):
    '''Locate page token for sending requests '''

    soup = bs4.BeautifulSoup(webpage.text, features='lxml')
    try:
        token = soup.form.input.attrs['value']
        return token
    except Exception as e:
        pass


def get_totals(webpage):
    '''Locate toll total charges and number of notices on the page.'''

    soup = bs4.BeautifulSoup(webpage.text, features='lxml')

    # Should retrieve spans for Total Amount due and Total Number of Notices
    overall_totals = soup.select("td > span")
    # Should retrieve anchor links withh spans for each notice
    violation_notices = soup.select("td.cl > a span")

    total_amt = overall_totals[0].text + overall_totals[1].text
    total_bills = overall_totals[3].text.replace('\xa0', '').replace('\n', ' ')

    print("\t|------ Found Totals ------|")
    print("\t\t",total_amt)
    print("\t\t",total_bills)
    print("\t|------  End Totals  ------|")
    print('\n')


def check_endable(message):
    ''' Check errflash message against known end-of-operation messages. End program gracefully if required. '''

    # Following should be benign indications that we can end the program
    endable = [
                "Multiple sessions have been detected. For your protection we are logging out of all sessions.",
                "Your query has not returned any results.",
                "No Data Matched the Search Criteria"
               ]


    # Not used.
    # Following should be indicator that the program needs to be fixed or the configuration information is wrong.
    recoverable = [
                "Please fix the errors below.",
                "Please Select License Plate State",
                "Please enter either Transponder Number or License Plate",
                "Please enter 13 digit Violation Number",
                "Please enter valid Zip Code"
                ]

    print(f"Error Flash Messagebox Content: {message}")

    if message in endable:
        print("Error Flash Message is designated as endable. ")
        return True
    else:
        print("Error Flash Message not designated to be endable. Continuing program execution.")
        return False


def get_pdf_page(link):
    # probably should post to /vector/violations/violTransactionList.do?violationNumber=<T082023063482>
    pass


def dispatchemail(item, content):
    pass




page_resp = request('GET', URL)
check_resp(page_resp)
formtoken = find_token(page_resp)
items = readstore()

for item in items:

    print(f"\tChecking config for --> {item} \n")
    page_resp = search_violation(convcfg(item))

    # Check for an error flash message on the page.
    check_resp(page_resp)
    message = get_errflash(page_resp)


    if check_endable(message):
        continue
    else:
        get_totals(page_resp)

    # Maybe alternatively setup a page on webserver for viewing for up to 48hrs if BILLABLES exist
    #dispatchemail(item, page_resp)



print("Script Ended. Godspeed.")
