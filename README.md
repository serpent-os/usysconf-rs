# usysconf-rs

A rewrite of the `usysconf` package trigger management system from [Solus](https://getsol.us) - designed to be shared between Solus and [Serpent OS](https://serpentos.com)

The initial focus is on replacing the aged `usysconf` (C) utility and ensuring it is as safe and battle tested as possible, being a core tenent of the update process.

While the initial focus is on providing the `/usr/sbin/usysconf` replacement, this will be written as a reusable
library to incorporate into [moss-rs](https://github.com/serpent-os/moss-rs) - our next generation package manager.

## License

`usysconf-rs` is available under the terms of the [MPL-2.0](https://spdx.org/licenses/MPL-2.0.html)
