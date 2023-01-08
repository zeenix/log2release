# **log2release**

Template creator for release notes from git log.

At the moment, it's a bit specific to zbus and hence the following is assumed:

* The repo has subcrates/subprojects.
* Tags are named `<subproject>-VERSION`.

## Usage

```sh
cargo run /home/zeenix/checkout/dbus/zbus zbus
```
