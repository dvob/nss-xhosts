# xhosts
`xhosts` is a NSS module which resolves host names. It is similar to the default `files` module which reads records from `/etc/hosts` but it allows to configure wildcard records.

## Example
* `/etc/xhosts`
```
# local k8s test
*.cluster.local 172.18.0.2

# other entry
other.foo.com 192.168.1.1
```

## Installation
* Build the code and install the shard library libnss_xhosts.so.2 in your library path:
```
./install.sh
```

* Add `xhosts` to your `hosts` configuration in  `/etc/nsswitch.conf`:
```
hosts:          files xhosts dns
```

* Create the file `/etc/xhosts` and add your records:
```
*.cluster.local 172.18.0.2
```
