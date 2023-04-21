# Midterm 1

## Part 1

Consider the topology in the attached figure, with two endpoints (A and D), two switches (B and C) and three links. Consider that the propagation delay in each link is Pi (in seconds), and that the bandwidth in each link is Bi (in MB/s), where i is the link number.

(a) Provide a formula for the one-way latency of a frame sent from A to D. Consider the one-way latency to be the time interval between the instant when A starts transmitting the frame on link 1 and the instant when D starts receiving the frame on link 3. Consider that the frame has T bytes. Assume that the switches incur no queuing or processing delay.

(b) Provide a formula for the one-way latency for a sequence of S packets sent from A to D. Consider the one-way latency for a sequence of packets to be the time interval between the instant when A starts transmitting the first frame on link 1 and the instant when D starts receiving the last frame on link 3. Consider that A starts transmission of a frame immediately after finishing transmission of the previous frame. Consider that all packets have the same number of bytes T.

---

(*Feedback*: assumes that switches start forwarding bits immediately,
in practice need to receive whole frame to forward.)

(a) *Revised*

The transmission time for each node are

$$
T_i=\frac{T}{B_i}\frac{1\text{B}}{1\text{MB/s}}
=\frac{T}{10^6\times B_i}\text{s}
$$

The latency between when each node starts either transmitting or receiving
are their propagation delays.
So, the one-way latency of a frame sent from A to D is

$$
\begin{align*}
    L_1&=P_1+P_2+P_3+T_1+T_2\\&=
    P_1+P_2+P_3+\frac{T}{10^6}\left(
        \frac{1}{B_1}+\frac{1}{B_2}
    \right)
\end{align*}
$$

(b)

The transmission time for each frame is bottlenecked by the slowest link

$$
T_0=\frac{T}{\min\{B_1,B_2,B_3\}}\frac{1\text{B}}{1\text{MB/s}}
=\frac{T}{10^6\times\min\{B_1,B_2,B_3\}}\text{s}
$$

The transmission time between when D starts to receive the first frame and
when D starts receiving the last frame is

$$
(S-1)T_0
$$

The one-way latency for a sequence of S packets sent from A to D is the sum of
transmission time and propagation delay

$$
\begin{align*}
    L_S&=(S-1)T_0+L_1\\[12pt]
    &=(S-1)\frac{T}{10^6\times\min\{B_1,B_2,B_3\}}+
    P_1+P_2+P_3+\frac{T}{10^6}\left(
        \frac{1}{B_1}+\frac{1}{B_2}
    \right)
\end{align*}
$$

## Part 2

Consider a link layer technology that uses a sending window of 4 frames and a reception window of 4 frames. Consider that frames are identified by the numbers between 0 and 15, that the first frame transmitted is frame zero, and that the sliding windows do not use optimizations such as duplicate ACKs, negative ACKs, or selective ACKs. Identify what frames are in the sending and receive window after each sequence of events below:

(a) Transmission of frames 0, 1, 2, and 3. Transmission errors occur in frames 1 and 2; and a transmission error occurs in the transmission of the ACK for frame 0.

(b) Transmission of frames 0, 1, 2, and 3. Transmission errors occur in frames 2 and 3; and a transmission error occurs in the transmission of the ACK for frame 1.

(c\) Transmission of frames 0, 1, 2, and 3. A transmission error occurs in the transmission of the ACK for frame 2.

---

*Original chart (disaster to type & could not display on Sakai)*:

|   | | |s|e|n|d|┊| | | | |r|e|c|v|
|---|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|
|(a)| |0|1|2|3| |┊| | | |0| | | | |
|(b)| |0|1|2|3| |┊| | |0|1| | | | |
|(c\)|0|1|2|3| | |┊|0|1|2|3| | | | |

Numbers marked are frames sent/received.

The left of `s` is LAR.

The left of `r` is LFR.

*Revised and displayed with code blocks*:

(a)
The receiver receives 3 after 0, which is out of order,
so it only sends ACK for 0.

```
0 1 2 3 4
^       ^
LFR    LFA
```

The sender does not get any ACK.

```
_ 0 1 2 3
^       ^
LAR    LFS
```

(b)
The receiver receives and sends ACK for 0 and 1.

```
0 1 2 3 4 5
  ^       ^
 LFR     LFA
```

The sender gets ACK for 0 and move forward to 1.

```
0 1 2 3 4
^     ^->
LAR  LFS
```

(c\)
The receiver receives and sends ACK for 0, 1, 2 and 3.

```
0 1 2 3 4 5 6 7
      ^       ^
     LFR     LFA
```

The sender gets ACK for 0, 1, 3 where 3 is out of order,
so it only moves forward to 2.

```
0 1 2 3 4 5
  ^   ^ -->
 LAR LFS
```

## Part 3

We could say that our NetMaze implementation uses sentinel-based framing for messages, where the newline character is used as the sentinel to mark the end of a message. Propose an alternate protocol that uses count-based framing for messages. Describe what messages look like in your new protocol, and how the receiver knows when it has finished receiving a message. (Assume that the message type and message content are the same as in the current protocol.)

---

- LEN:
    A `u8` number for the length of the message in bytes,
    including the message type and message content.
    - Allows a message size from 0 to 127 bytes.
- Sender:

        LEN + message + LEN

    - Count the length LEN of the message in bytes as an `u8`.
    - Prepend and append LEN to the message, and send it.

- Receiver:

        LEN + message + LEN  +  …
         ^    ^     ^    ^      ^
        1st 2nd LEN+1st LEN+2nd LEN+3rd

    - Read the 1st byte received as LEN.
    - Read the 2nd to LEN + 1st bytes as the message payload.
    - Validate that the LEN + 2nd byte is the same as LEN.
        - If validation fails, the message is corrupted, drop the connection.
    - Start the above process again from the LEN + 3rd byte.

## Part 4

Answer the questions below about Ethernet and Wi-Fi:

(a) Describe one cause for collisions in Ethernet and Wi-Fi networks.

(b) Describe how Ethernet and Wi-Fi attempt to minimize the number of collisions.

(c\) Describe how Ethernet and Wi-Fi detect collisions.

---

(a)
Ethernet:
Node A and node B on the same link L start to transmit via L to each other
at the same time so their signals interfere.

Wi-Fi:
Node A is sending signal which shadows the incoming weaker signal from
Node B.

(b)
Ethernet:
Collision detection.

- Nodes watch for signal interference.
- Use a minimum packet size that has a longer transmission time than
    propagation delay.
- Send jamming sequence and stop after collisions are detected.

(*Feedback*: This is detection.
Avoidance uses exponential backoff, carrier sense.)

Wi-Fi:
Collision avoidance.

- Nodes send RTS before sending packets.
- Nodes send packets only after receiving CTS for themselves.
- Nodes stay quiet for some time after receiving CTS for other nodes.

(c\)
Ethernet:
There is signal coming in to the node when it is sending.

Wi-Fi:
Collision detection is hard so collision avoidance is used instead.
