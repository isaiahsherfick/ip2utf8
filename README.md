# utf8toipv4

UTF-8 strings as ip addresses and vice versa

this crate has both a library and two binary packages. The library package
exposes a few basic functions which can be used to convert UTF-8 strings into
ipv4 addresses. This is done by first converting each character/grapheme in the
string into its associated Unicode code point as a base 10 u64 and storing the
non-zero bytes in an array. The first four non-zero bytes from that array are
then converted to an ipv4 address. This functionality is accessible from the
binary crate &rsquo;utf8toipv4&rsquo;.

If you want to figure out what input to utf8toipv4 would result in a given ip
address, you can use the &rsquo;ipv4toutf8&rsquo; binary crate.

The question of which UTF-8 strings will yield a given ip address when fed into
utf8toipv4 is more difficult to answer, as there are thousands of potential
solutions given the way UTF-8 and utf8toipv4 both work. As such, ipv4toutf8
prints out a list of a few UTF-8 strings that will work. Many of these are
hideous and feature a lot of unprintable characters, but that&rsquo;s just the nature
of these things.


<a id="org391e953"></a>

## utf8toipv4 (library)

the library exposes the following functions:

    utf8_to_ipv4(&str) -> Result<Vec<String>,Error>,
    
    ipv4_to_utf8(&str) -> Vec<String>,


<a id="org8ed21cd"></a>

## utf8toipv4 (binary package)

the utf8toipv4 binary package simply calls the utf8_to_ipv4 function on whatever
string is supplied as a command line argument. This can be piped into xargs and
used as an argument to whatever networking tool you want.

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


<a id="org046fc0c"></a>

## ipv4toutf8 (binary package)

the ipv4toutf8 binary package simply calls the ipv4_to_utf8 function on whatever
string is supplied as a command line argument. If the string is not a valid ipv4
address, it prints an error. If the string is a valid ipv4 address, it prints
each of the computed solution strings on its own line like so:

    ipv4toutf8 192.168.2.1
    \u{c0}\u{a8}\u{2}\u{1}
    \u{c0}𐊨
    𪣀\u{1}
    ꣀ\u{2}\u{1}
    ꣀĂ
    \u{c0}\u{a8}Ă

By default, most non-printable characters are represented as their ASCII escape
codes. I might add an option to print the raw output in the future, but I don&rsquo;t
want to right now.

