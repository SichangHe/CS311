# Midterm 3

## Question 1

Consider a TCP connection that uses AI/MD (additive-increase multiplicative-decrease) and TCP fast retransmit to control the congestion window. To simplify, consider (i) that the congestion window is maintained at the packet granularity (instead of bytes), (ii) that all transmitted packets are the same size, (iii) that the one-way propagation delay is 1 second, (iv) that transmissions are instantaneous (there is infinite bandwidth), (v) that the congestion window starts with one packet, and (vi) that the retransmission timeout is 4 seconds. Draw three diagrams for the scenarios described below:

1. Draw a diagram describing the behavior of the congestion window over time for the transmission of the first 50 packets. Consider no packets are lost.

    Assume that the destination sends the ACKs back immediately upon reception.
    Assume that the sender computes the new cwnd instantly and send immediately
    afterwards.

2. Draw a diagram describing the behavior of the congestion window over time for the transmission of the first 50 packets. Consider that the first transmission of packet 23 is lost. Indicate any retransmission timeouts.

    cwnd uses integer division.

3. Draw a diagram describing the behavior of the congestion window over time for the transmission of the first 50 packets. Consider that the first transmission of packets 23 and 24 are lost. Indicate any retransmission timeouts.

    Fast retransmit on 4 duplicated ACKs.

State any assumptions of simplifications you make.

## Question 2

UDP provides more flexibility to programmers than TCP, and may be more efficient for some application classes. One broad class of application where UDP can be beneficial compared to TCP is interactive applications such as videoconferencing and online games. Discuss why UDP can be more adequate than TCP for these applications, give one example where UDP behavior would be better than that of TCP.

---

UDP has advantages over TCP:

- Lower overhead.
    - Smaller packet size.
        UDP may be better for utilizing the bandwidth.
    - No three-way handshake.
        Instant resume on unstable network connection.
- Statelessness.
    - No in-order delivery guarantee.
        Ensure information is timely instead of waiting.
    - No connection.
        No need to wait for reconnection.

Consider videoconferencing.
One user could have unstable connection (DKU Wi-Fi) and lose connection
for several seconds.
Since TCP is stateful and has in-order delivery guarantee,
they would need to wait for the previous video and sound track to be
retransmitted before they can see or hear the current video and sound.
With UDP, they would only see the videoconference freeze for those few seconds
and immediately get up-to-date video and sound after the internet connection
resume.

## Question 3

Draw a time diagram showing TCP segments exchanged between a client and a server considering the events below. Show each individual packet. For each packet, indicate what TCP flags are set and the TCP sequence number.

1. The client establishes a connection to the server

    ---

    client --segment1--> server

    SequenceNum: x\
    Flags: SYN

    ---

    client <--segment2-- server

    SequenceNum: y\
    Acknowledgment: x + 1\
    Flags: SYN + ACK

    ---

    client --segment3--> server

    SequenceNum: x + 1\
    Acknowledgment: y + 1\
    Flags: ACK

2. The client sends a single-byte request to the server

    ---

    client --segment4--> server

    SequenceNum: x + 2

    ---

    client <--segment5-- server

    SequenceNum: y + 1\
    Acknowledgment: x + 2\
    Flags: ACK
3. The server sends a single-byte response to the client

    ---

    client <--segment6-- server

    SequenceNum: y + 2

    ---

    client --segment7--> server

    SequenceNum: x + 3

    Acknowledgment: y + 2\
    Flags: ACK
4. The client terminates the connection

    ---

    client --segment8--> server

    SequenceNum: x + 4\
    Flags: FIN

    ---

    client <--segment9-- server

    SequenceNum: y + 3\
    Acknowledgment: x + 4\
    Flags: FIN + ACK

    ---

    client --segment10--> server

    SequenceNum: x + 5\
    Acknowledgment: y + 3\
    Flags: ACK

    ---

    client <--segment11-- server

    SequenceNum: y + 4\
    Acknowledgment: x + 5\
    Flags: ACK

## Question 4

Describe a network scenario where the Jacobson/Karels algorithm to compute the TCP retransmission timeout outperforms the Karn/Partridge algorithm. (Include detailed properties of the network round-trip time.)

---

Consider a network scenario where RTT has a normal distribution
with mean 10 and standard deviation 1 (sec).

For both algorithms, EstimatedRTT should converge to about 10.
For the Karn/Partridge algorithm, Timeout is about 20.
For the Jacobson/Karels algorithm, Timeout is about 14.

Since the RTT has a normal distribution, the probability that any RTT is
outside the range of 4 standard deviations is basically 0.
This means that the time that the Karn/Partridge algorithm wait for packets to
arrive after 14sec is almost always in vain, and therefore wasted.
So, the Jacobson/Karels algorithm should outperform it on average.
