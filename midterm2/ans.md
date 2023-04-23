# Midterm 2

## Part 1

Consider the network topology below where each node represents an Autonomous System (AS) in the Internet and each edge represents an interconnection link. Vertical links indicate the relationship between a provider (above) and a client (below), where the client pays the provider proportional to the volume of traffic traversing the link. Horizontal links indicate a peer-to-peer relationship, where traffic is exchanged free of charge between the two networks. Assume that networks incur costs to forward packets internally (for example, from one border router to the next). Assume also that networks configure BGP in an attempt to maximize profits.

For each of the four routes below between AS A and AS B, discuss if the route could be used in practice or not. Justify your answer.

---

1. Route A-E-C-D-H-B.

    Yes, could be used.

    The way A knows about this route to B is as follows,
    the way B knows about this route to A would be similar:
    - For B, H, and D:
        H exports its client route H-B to its provider D.
        So, D knows route D-H-B.
    - D exports all its client routes including D-H-B to its peer C,
        so C knows route C-D-H-B
    - For A, E, and C:
        C exports all its routes including C-D-H-B to its client E.
        E exports all its routes including E-C-D-H-B, to A.
        A knows A-E-C-D-H-B.

2. Route A-E-F-D-H-B.

    No, could not be used.

    F would not export the route to its provider D, F-D, to its peer E,
    so A would not know about any route including F-D including this route.

    F would not export the route to its peer E, E-F, to its provider D,
    so D would not know about any route including E-F including this route,
    so B would not know about any route that includes E-F and passes through D
    including this route to A.

3. Route A-E-F-G-B.

    No, could not be used.

    F would not export the route to its peer G, F-G, to its peer E,
    so E would not know about any route including F-G including this route,
    so A would not know about any route that includes F-G and passes through E
    including this route to B.

    F would not export the route to its peer E, E-F, to its peer G,
    so G would not know about any route including E-F including this route,
    so B would not know about any route that includes E-F and passes through G
    including this route to A.

4. Route A-F-G-B.

    Yes, could be used.

    The way A knows about this route to B is as follows,
    the way B knows about this route to A would be similar:
    - G exports its client route G-B to its peer F,
        so F knows F-G-B.
    - F exports all its routes including F-G-B to A,
        so A knows A-F-G-B.

## Part 2

Answer the questions below. Justify your answers.

1. Why are different intradomain and interdomain routing protocols used in the Internet? (Why not a single type of routing protocol?)
2. The Border Gateway Protocol (BGP) is classified as a path vector protocol. The AS-path field contains the vector with the sequence of autonomous systems traversed by a route. How is the AS-path field used by BGP?
3. What are the differences between a network switch (layer-2) forwarding a packet vs a network router (layer-3) routing a packet?

---

1. The internet has too many routers and endpoints,
    so intradomain protocols would not scale to fit interdomain routing needs.

    Interdomain routing involves more than one provider,
    so other monetary factors need to be considered.

2. Each router periodically transmits its AS-path to all its neighbors.
    (Feedback: Not periodic. BGP is stateful.)

    Each router uses the AS-path for forwarding.
    (Feedback: Only the next hop is needed for forwarding.
    The AS-path is useful in loop detection and policies.)

3. Network switches connect links of the same type,
    so they use link-layer address.
    They simply keep forwarding tables to map destination to outgoing port.

    Network routers can connect links of different types,
    so they need to translate IP to link-layer address.
    They also keep a routing table and periodically merge the routes into
    their forwarding table.

## Part 3

Consider the network below with three switches and four computers. Consider that the network uses virtual switching and that the circuit identifier has only one bit. Answer the following questions:

1. Show the forwarding tables for all the three switches to implement unidirectional circuits A → C, C → B, B → D e D → A. The circuits should be created in this order and the smallest circuit identifier available should be used when a new identifier needs to be allocated.

2. Is it possible to create another circuit in this network? If yes, provide one example.

---

1. Forwarding tables:

    |Switch|input port|input circuit|output port|output circuit|route for|
    |-|-|-|-|-|-|
    |Left|2|0|3|0|A → C|
    |Left|3|0|1|0|C → B|
    |Left|1|1|3|1|B → D|
    |Left|3|0|2|0|D → A|
    |Middle|1|0|2|0|A → C|
    |Middle|2|0|1|0|C → B|
    |Middle|1|1|2|1|B → D|
    |Middle|2|1|1|0|D → A|
    |Right|3|0|1|0|A → C|
    |Right|2|0|3|0|C → B|
    |Right|3|1|1|0|B → D|
    |Right|1|0|3|0|D → A|

    *Feedback*:

    |Switch|input port|input circuit|output port|output circuit|route for|
    |-|-|-|-|-|-|
    |Left|2|0|3|0|A → C|
    |Left|3|0|1|0|C → B|
    |Left|1|0|3|1|B → D|
    |Left|3|1|2|1|D → A|
    |Middle|1|0|2|0|A → C|
    |Middle|2|0|1|0|C → B|
    |Middle|1|1|2|1|B → D|
    |Middle|2|1|1|1|D → A|
    |Right|3|0|2|0|A → C|
    |Right|2|0|3|0|C → B|
    |Right|3|1|1|0|B → D|
    |Right|1|0|3|1|D → A|

2. Example: A → B

    |Switch|input port|input circuit|output port|output circuit|route for|
    |-|-|-|-|-|-|
    |Left|2|1|1|0|A → B|

## Part 4

Suppose network provider BigTelecom needs to delegate some of its IP address space to its client MiniTelecom. BigTelecom has identified two options:

- Delegate prefixes 200.128.30.0/24, 200.128.41.0/24, 200.128.44.0/24 and 200.128.47.0/24, or
- Delegate prefix 200.128.48.0/21 to MiniTelecom.

Discuss which factors BigTelecom should consider when deciding which allocation is more adequate

---

200.128.30.0/24, 200.128.41.0/24, 200.128.44.0/24 and 200.128.47.0/24
are $4\times 2^{32-24}=2^{10}$ addresses.

200.128.48.0/21 are $2^{32-21}=2^{11}$ addresses.

So, 200.128.48.0/21 has two times more addresses than the former option,
and therefore should be more adequate.
(Feedback: But this will waste address as 1024 are enough.)

On the flip side,
since the latter is in one large chunk instead of 4 smaller chunks,
MiniTelecom might more likely divide the allocation into larger chunks and
run out of IPs sooner, ironically.
(Feedback: Ok. Depends on the management policies.

One large chunk avoids bloating the global routing table.)
