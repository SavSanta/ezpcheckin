#!/usr/bin/env python3

import requests, bs4, base64, functools
from random import randint

# Define constants
URL = base64.decodestring(b"aHR0cHM6Ly93d3cuZXpwYXNzbWQuY29tL3ZlY3Rvci92aW9sYXRpb25zL3Zpb2xOb3RpY2VJbnF1aXJ5LmRvP2xvY2FsZT1lbl9VUyZmcm9tPUhvbWU=")


def check_resp(content):
    pass


def readstore(config="ezpstore.txt"):
    ''' Reads the lines for the EZPass accounts to check and returns as a list variable '''

    with open(config) as lookup:
        items = lookup.readlines()
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
        'formid': frmViolationInquiry,
        'btnSearch.x': randint(0,70),
        'btnSearch.y': randint(0,18),
        'ctokenElem': formtoken,
       }

    basedata.append(cfgdata)
    resp = request('GET', URL, data=basedata)

    return resp


def convcfg(data):
    ''' Convert the data from configuration list into values to use in search violation. '''

    d = {}

    # Separate the values on the config line
    chunks = [ x.strip() for x in data.split("||") ]

    if chunks[0] == 'PLATE':
        # Example values to populate
        # loginNumber = 2DLXEFM or S123456789 or 555 Santa Claus Street | licenstate = MD | zipcode = 47450
        d.update({'loginType': 'plate', 'selectCreditCard':'new', 'loginNumber':chunks[1] , 'zipCode':chunks[2], 'licenseState':chunks[3]})
        return d
    elif chunks[0] == 'MAIL':
        d.update({'loginType': 'violation', 'loginNumber':chunks[1], 'zipCode':chunks[2]})
        return d
    elif chunks[0] == 'DEVICE':
        d.update({'loginType': 'transponder', 'loginNumber':chunks[1], 'zipCode':chunks[2]})
        return d
    elif chunks[0] == 'LIC':
        d.update({'loginType': 'license', 'loginNumber':chunks[1] , 'zipCode':chunks[2], 'licenseState':chunks[3]})
        return d
    else:
        raise Exception("Error While Reading Configuration!! Check for blank lines or incorrect line structure!")


def get_errflash(soup):
    ''' Retrieve the error box flash alert content '''

    errflash = soup.form.div.table.tbody.td.table.tr.td.table.td.next.next.next.text

    if errflash:
        return errflash
    else:
        return False


def find_token(req):
    soup = bs4.BeautifulSoup(req.text)
    try:
        token = soup.form.input.attrs['value']
        return token
    except Exception as e:
        pass


def dispatchemail(item):
    pass


page_resp = request('GET', URL)

# if check_resp call

formtoken = find_token(page_resp)
items = readstore()

for item in items:
    if (item.lstrip.startswith('#')) or (not item):
        continue
    print("Checking config for --> {}".format(item)) 
    page_resp = search_violation(convcfg(item))
   # check_resp call
   # Retrieve the number of billables
   # Maybe alternatively setup a page on webserver for viewing for up to 48hrs if BILLABLES exist
    dispatchemail(item, page_resp)


print("Script Ended.")
