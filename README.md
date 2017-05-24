HwAddr
======
Hardware addresses with access to the producer database.

Building
--------
Building requires either providing a local database or letting the builder
download one.

By default a database is provided and used, otherwise there are a couple options.

- Setting `HWADDR_DATABASE` to a path will use the given path as database.
- Setting `HWADDR_DOWNLOAD` to `1` is going to download it from [IEEE
website](http://standards.ieee.org/develop/regauth/oui/oui.txt) (this is really slow)
- Setting `HWADDR_DOWNLOAD` to a valid URL is going to download it from there.
