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

    No.
    If so, what information follows the “IF-MODIFIED-SINCE:” header?
1. What is the HTTP status code and phrase returned from the server in response
    to this second HTTP GET?

    "200 OK"
    Did the server explicitly return the contents of the file?

    Yes.
    Explain.

    When I refresh, I force the browser to fetch the HTML file again.
    Since the browser sent the same request as the first time,
    the server responded similarly.
