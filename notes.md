### Part 1:

Implmenting a protocol from scratch can be difficult as the OS kernal already implements many of these.


We can use the TUN/TAP feature from the kernel to get around this limitation.


The Network Interface Card (NIC) recieves all packets from the network. The kernel handles the data flow from the NIC.


If something in user space wants to access the network connection you create a socket; this allocates memory in the kernel which the user can point to for reading and writing to the network.


The TUN/TAP creates a virtual network interface that is treated as a NIC that is connected directly to the user space. Allowing for an emulation of a network inside the user space.


0x800 is the IPv4 Protocol header, we need to use the IPv4 header in the rust code to parse the values recieved.


The IPv4 is the link level protocol, the protocol used to send the packet should be something like TCP (the IP level protocol)


Parsing the TCP packet is simply reading bytes from the header according the specified format in the RFC.


A connection is a quad of source IP, source port, destination IP, and destination port.


The first packet sent is the SYN packet. This is the first part of the TCP handshake.
