# nu_plugin_pnet

This is taken from https://github.com/fennewald/nu_plugin_net and modernized a tiny bit. Thanks! Be sure to read the changelog for tips for Windows users.

A nushell plugin to list system network interfaces

A simple and straightforward plugin. All of the heavy lifting is done by pnet. This package just translates the datatypes into a nu-accepatble format.

Format may be subject to change.

# Examples

```
~> net
╭───┬──────────┬─────────────┬───────────────────┬────────────────┬───────────────────╮
│ # │   name   │ description │        mac        │      ips       │       flags       │
├───┼──────────┼─────────────┼───────────────────┼────────────────┼───────────────────┤
│ 1 │ lo       │             │ 00:00:00:00:00:00 │ [table 2 rows] │ {record 5 fields} │
│ 2 │ enp2s0f0 │             │ 8c:8c:aa:1f:a5:2a │ [table 2 rows] │ {record 5 fields} │
│ 3 │ wlp3s0   │             │ c8:e2:65:c3:09:42 │ [table 2 rows] │ {record 5 fields} │
╰───┴──────────┴─────────────┴───────────────────┴────────────────┴───────────────────╯
```

```
~> net | flatten flags
╭───┬──────────┬─────────────┬───────────────────┬────────────────┬───────┬──────────────┬─────────────┬───────────────────┬──────────────╮
│ # │   name   │ description │        mac        │      ips       │ is_up │ is_broadcast │ is_loopback │ is_point_to_point │ is_multicast │
├───┼──────────┼─────────────┼───────────────────┼────────────────┼───────┼──────────────┼─────────────┼───────────────────┼──────────────┤
│ 1 │ lo       │             │ 00:00:00:00:00:00 │ [table 2 rows] │ true  │ false        │ true        │ false             │ false        │
│ 2 │ enp2s0f0 │             │ 8c:8c:aa:1f:a5:2a │ [table 2 rows] │ true  │ true         │ false       │ false             │ true         │
│ 3 │ wlp3s0   │             │ c8:e2:65:c3:09:42 │ [table 2 rows] │ true  │ true         │ false       │ false             │ true         │
╰───┴──────────┴─────────────┴───────────────────┴────────────────┴───────┴──────────────┴─────────────┴───────────────────┴──────────────╯
```

```
~> net | select ips | flatten | flatten
╭───┬──────┬──────────────────────────────┬────────╮
│ # │ type │             addr             │ prefix │
├───┼──────┼──────────────────────────────┼────────┤
│ 0 │ v4   │ 127.0.0.1/8                  │      8 │
│ 1 │ v6   │ ::1/128                      │    128 │
│ 2 │ v4   │ 192.168.1.232/24             │     24 │
│ 3 │ v6   │ fe80::8e8c:aaff:fe1f:a52a/64 │     64 │
│ 4 │ v4   │ 192.168.4.189/24             │     24 │
│ 5 │ v6   │ fe80::cae2:65ff:fec3:942/64  │     64 │
╰───┴──────┴──────────────────────────────┴────────╯
```

# Installing

This plugin uses the `msgpack` encoding. To register:

```
register <path to plugin binary>
```

# Changelog

Version 1.1.0

Use `if_index` instead of `index`, fixing the way table indexes are displayed

Version 1.2.0

Renamed from `net` to `pnet` as to not cause a conflict on Windows.

The biggest mistake I made was not realizing that:

1. You have to download Packet.lib from Winpcap Developers Pack from https://www.winpcap.org/devel.htm, making sure to use the x64 version vs the x86 version. You have to put Packet.lib in the root of the nu_plugin_pnet folder for linking.
2. You have to install npcap installer from https://npcap.com/#download

Otherwise you get these errors when trying to register

```
register c:\CarTar\debug\nu_plugin_pnet.exe
  × Error getting signatures
   ╭─[entry #1:1:1]
 1 │ register c:\CarTar\debug\nu_plugin_pnet.exe
   · ────┬───
   ·     ╰── Plugin failed to load: unable to get encoding from plugin: failed to fill whole buffer
   ╰────
```
