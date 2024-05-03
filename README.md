# Retoro
Decentralized communications solution

Overall vision and more information on the project in https://github.com/wrx75752/projekt-nowatorski

builder required fields
profile name: string
profile secret key: bytes 
interface types: enum or addrs: multiaddrs
bootnodes: miltiaddrrs



interface of retoro client api

you create retoro node with builder
(set profile, network stuff, keys)

then you have struct of type retoro with methods:
send message(content, target id)
join channel(name)
save address
get channels
get nodes in channel
recv messages(target id)? Stream<Message>





UI abstraction
Node{
    name: String,
    id: PeerId
    addr: Multiaddr
}
Message{
    ts: u64
    author: PeerId
    content: String
}
map of channels:
    name/id -> {
        chat: Vec<Message>
        nodes: Vec<Node>
    }


Vec for currently open chat
input -> string

