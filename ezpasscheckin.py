#!/usr/bin/env python3

import requests, bs4
from random import randint


def search_violations(format, goop):
    ''' Preps data out to be sent in POST url form submission"

    basedata = {
        org.apache.struts.taglib.html.TOKEN: formtoken,
        formid: frmViolationInquiry,
        btnSearch.x: randint(0,70),
        btnSearch.y: randint(0,18),
        ctokenElem: formtoken,
       }


    return data


def get_errflash(soup)"

    return soup.form.div.table.tbody.td.table.tr.td.table.td.next.next.next.text

def find_token(soup):

    try:
        token = soup.form.input.attrs['value']
        return token
    except Exception as e:
        pass


USER_AGENT = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Ubuntu Chromium/79.0.3945.79 Chrome/79.0.3945.79 Safari/537.36"
URL = "https://www.ezpassmd.com/vector/violations/violNoticeInquiry.do?locale=en_US&from=Home"


page_resp = requests.request(URL, headers={"User-Agent' : USER_AGENT})
token = find_token(page_resp)
