# Wireshark HTTP

## The Basic HTTP GET/response interaction

1. Is your browser running HTTP version 1.0 or 1.1?
    What version of HTTP is the server running?

    My browser runs HTTP 1.1.
    The server runs HTTP 1.1.
1. What languages (if any) does your browser indicate that it can accept to the
    server?

    "en-US,en;q=0.9": meaning that my browser indicates that it accepts US
    English or English with weight 0.9.
1. What is the IP address of your computer?

    "10.200.97.250"

    Of the gaia.cs.umass.edu server?

    "128.119.245.12"
1. What is the status code returned from the server to your browser?

    "200 OK"
1. When was the HTML file that you are retrieving last modified at the server?

    It says "Sun, 30 Apr 2023 05:59:01 GMT," but I believe the server is lying.
1. How many bytes of content are being returned to your browser?

    "81"
1. By inspecting the raw data in the packet content window,
    do you see any headers within the data that are not displayed in the
    packet-listing window?
    If so, name one.

    Yes. For example, the IP header "Protocol: TCP (6)" is not shown in the
    packet-listing window.

## The HTTP CONDITIONAL GET/response interaction

(Questions 8 ~ 11)

1. Inspect the contents of the first HTTP GET request from your browser to the
    server.
    Do you see an “IF-MODIFIED-SINCE” line in the HTTP GET?

    No.
1. Inspect the contents of the server response.
    Did the server explicitly return the contents of the file?

    Yes.

    How can you tell?

    I can tell because I get 10 lines of "Line-based text data."
1. Now inspect the contents of the second HTTP GET request from your browser to
    the server.
    Do you see an “IF-MODIFIED-SINCE:” line in the HTTP GET?

    Yes.

    If so, what information follows the “IF-MODIFIED-SINCE:” header?

    "Sun, 30 Apr 2023 05:59:01 GMT"
1. What is the HTTP status code and phrase returned from the server in response
    to this second HTTP GET?

    "304 Not Modified"

    Did the server explicitly return the contents of the file?

    No.

    Explain.

    When I refreshed the page, Firefox sent a new request with the
    "If-Modified-Since" header that contains the time in "Last-Modified" in the
    last server response.
    The server got the request, checked the "If-Modified-Since" and found out
    that the content does not need to be updated,
    so it returned "304 Not Modified."
    Firefox got the 304 response and kept using the old cached page.

*Note*: Safari does not send "If-Modified-Since" on refresh.
See <https://stackoverflow.com/questions/13789855/safari-not-sending-if-modified-since-and-if-none-match-headers>.

## Retrieving Long Documents

(Questions 12 ~ 15)

1. How many HTTP GET request messages did your browser send?

    One.

    Which packet number in the trace contains the GET message for the Bill or
    Rights?

    "101"
1. Which packet number in the trace contains the status code and phrase
    associated with the response to the HTTP GET request?

    "111"
1. What is the status code and phrase in the response?

    "200 OK"
1. How many data-containing TCP segments were needed to carry the single HTTP
    response and the text of the Bill of Rights?

    4: Frame 108, 109, 110, and 111.
