#!/usr/bin/env python3

import requests, bs4, base64
from random import randint

def search_violations(format):
    ''' Preps data out to be sent in POST url form submission'''

    basedata = {
        org.apache.struts.taglib.html.TOKEN: formtoken,
        formid: frmViolationInquiry,
        btnSearch.x: randint(0,70),
        btnSearch.y: randint(0,18),
        ctokenElem: formtoken,
       }

    basedata.append()

    return data


def conv_format(data):
    ''' Convert the data from configuration list into values to  use in search violation. '''

    d = {}

    # Separate the values on the config line
    chunks = [ x.strip() for x in data.split("||") ]

    if chunks[0] == 'PLATE':
        # Example values to populate
        # loginNumber = 2DLXEFM | licenstate = MD | zipcode = 47450
        d.update({'loginType': 'plate', 'selectCreditCard':'new', 'loginNumber':chunks[1] , 'zipCode':chunks[2], 'licenseState':chunks[3]})
        return d
    elif chunks[0] == 'ADDR':
        d.update({'loginType': 'violation', 'loginNumber':chunks[1], 'zipCode':chunks[2]})
        return d
    elif chunks[0] == 'DEVICE':
        d.update({'loginType': 'transponder', 'loginNumber':chunks[1], 'zipCode':chunks[2]})
        return d
    elif chunks[0] == 'LIC':
        d.update({'loginType': 'license', 'loginNumber':chunks[1] , 'zipCode':chunks[2], 'licenseState':chunks[3]})
        return d
    else:
        raise Exception("Error While Reading Configuration!")



def get_errflash(soup)"

    return soup.form.div.table.tbody.td.table.tr.td.table.td.next.next.next.text


def find_token(soup):

    try:
        token = soup.form.input.attrs['value']
        return token
    except Exception as e:
        pass


# CREATE LOGGER DECORATOR HERE

# CREATE REQUESTS.REQUEST DECORATOR HERE (NO IM NOT USING SESSION)

USER_AGENT = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/79.0.3945.79 Chrome/79.0.3945.79 Safari/537.36"
URL = base64.decodestring(b"aHR0cHM6Ly93d3cuZXpwYXNzbWQuY29tL3ZlY3Rvci92aW9sYXRpb25zL3Zpb2xOb3RpY2VJbnF1aXJ5LmRvP2xvY2FsZT1lbl9VUyZmcm9tPUhvbWU=")

page_resp = requests.request(URL, headers={"User-Agent' : USER_AGENT})
formtoken = find_token(page_resp)


