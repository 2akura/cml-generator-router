# config list
```
basic config {
  hostname
  banner
  no ip domain
  password
  vty line
  line console
  username
  domain-name
  password-encryption
 }
interface {
  ip assignment
  sub-interface
}
ospf {
  ospf pid
  router id
  <ospf included int>
  pssive int
  def route-injection
  ( int-selection, hello interval & priority )
}
rip
static route
dhcp
```
