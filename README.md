### ipcalc
`ipcalc` is a tool for calculating ip information, written in rust. The program takes an ip address, in CIDR format, as its first and only argument, and reports back information about it.

#### Examples
Get information on an ip:
```
$ ipcalc 192.168.116.22/24
Address:   192.168.116.22   11000000.10101000.01110100.00010110
Netmask:   255.255.255.0    11111111.11111111.11111111.00000000
Wildcard:  0.0.0.255        00000000.00000000.00000000.11111111
=>
Network:   192.168.116.0    11000000.10101000.01110100.00000000
HostMin:   192.168.116.1    11000000.10101000.01110100.00000001
HostMax:   192.168.116.254  11000000.10101000.01110100.11111110
Broadcast: 192.168.116.255  11000000.10101000.01110100.11111111
```

#### Flags
```
USAGE:
    ipcalc <ip-address>

ARGS:
    <ip-address>    IP address to operate on

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
