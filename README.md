<a id="org048c52b"></a>

# ip2utf8

ip addresses as UTF-8 encoded strings and vice versa

this crate has both a library and two binary packages. The library package exposes
a few basic functions which can be used to convert UTF-8 strings into ipv4
addresses. This is done by first converting each character/grapheme in the
string into its associated Unicode code point as a base 10 u64 and storing the
non-zero bytes in an array. The first four non-zero bytes from that array are
then converted to an ipv4 address. The inverse operation is also possible.


<a id="org253a01b"></a>

## ip2utf8 (library)

the library exposes the following functions:

    utf8_to_ipv4(&str) -> String,
    
    ipv4_to_utf8(&str) -> String,


<a id="org351222a"></a>

## utf8toipv4 (binary package)

the binary package is called &rsquo;utf8toipv4&rsquo; and simply calls the

    utf8_to_ipv4

function on whatever string is supplied as a command line argument. This can be
piped into xargs and used as an argument to whatever networking tool you want.

    utf8toipv4 Isaiah | xargs ping
    PING 73.115.97.105 (73.115.97.105) 56(84) bytes of data.
    64 bytes from 73.115.97.105: icmp_seq=1 ttl=42 time=83.6 ms
    64 bytes from 73.115.97.105: icmp_seq=2 ttl=42 time=82.1 ms
    64 bytes from 73.115.97.105: icmp_seq=3 ttl=42 time=78.2 ms
    64 bytes from 73.115.97.105: icmp_seq=4 ttl=42 time=80.8 ms
    64 bytes from 73.115.97.105: icmp_seq=5 ttl=42 time=83.0 ms
    64 bytes from 73.115.97.105: icmp_seq=6 ttl=42 time=82.0 ms
    ^C
    --- 73.115.97.105 ping statistics ---
    6 packets transmitted, 6 received, 0% packet loss, time 5007ms

    utf8toipv4 🍌😂 | xargs ping
    PING 76.243.1.2 (76.243.1.2) 56(84) bytes of data.
    64 bytes from 76.243.1.2: icmp_seq=1 ttl=53 time=172 ms
    64 bytes from 76.243.1.2: icmp_seq=2 ttl=53 time=66.5 ms
    64 bytes from 76.243.1.2: icmp_seq=3 ttl=53 time=65.8 ms
    ^C
    --- 76.243.1.2 ping statistics ---
    3 packets transmitted, 3 received, 0% packet loss, time 2001ms


<a id="orgffb6284"></a>

## TODO ipv4toutf8 (binary package)

WIP

