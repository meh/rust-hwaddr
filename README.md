HwAddr
======
Hardware addresses with optional access to the producer database.

Building
--------
Optionally the database of producers to find out who produces a certain MAC
address can be compiled in by enabling the `database` feature. Keep in mind
this database adds up to the final binary around 7MB.

By default a database is provided and used, otherwise there are a couple options.

- Setting `HWADDR_DATABASE` to a path will use the given path as database.
- Setting `HWADDR_DOWNLOAD` to `1` is going to download it from [IEEE
website](http://standards.ieee.org/develop/regauth/oui/oui.txt) (this is really slow)
- Setting `HWADDR_DOWNLOAD` to a valid URL is going to download it from there.
