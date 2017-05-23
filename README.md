HwAddr
======
Hardware addresses with access to the producer database.

Building
--------
Building requires either providing a local database or letting the builder
download one.

The database is downloaded from the [IEEE
website](http://standards.ieee.org/develop/regauth/oui/oui.txt), but it doesn't
provide HTTPS, so it may be better to download it and setting the
`HWADDR_DATABASE` environment variable to its path before building.
