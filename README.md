# Genet

Genet is a graphical network analyzer powered by web technologies.

## Features

- Cross-Platform (macOS, Linux, Windows)
- Web-based UI (Electron)
- Built-in Package Manager
- SDK for JavaScript and Rust
- Concurrency Support

<img src="https://raw.githubusercontent.com/deplug/images/master/screenshot.png" alt="Screenshot in Ubuntu" width="560"/>

## Download

https://deplug.net/#/inst?id=download

## Why Genet?

### Package Management
__Everything up-to-date.__

Decoding latest protocol specs without updating Genet itself.

__Custom protocols.__

You can easily publish & install extensions for custom protocols.

### Extensible
__No protocol-dependent features.__

Genet core provides only fundamental functionality and 
every protocol is implemented as an independent package.

For example, display filter does not have a syntax for IPv4 address like `ipv4.src === 127.0.0.1`.
Instead, there is an extensible macro expression: `ipv4.src === @127.0.0.1` (Defined in `ipv4` package).

### Security
__Writing a secure dissector in C/C++ is hard.__ 

Genet provides a safer & modern SDK for JavaScript and Rust.

__Enabling packages only what you need.__

Decreases the risk of loading vulnerable modules.

__Sandboxing.__

Dissectors can run under seccomp. (Linux only)

## Built-in Supported Protocols

- Ethernet
- IPv4
- IPv6
- UDP
- TCP
- HTTP
- NTP

## Import / Export

Genet supports following formats by default.

- Pcap File (*.pcap)

## Preferences

Configuration files are located in `$HOME/.genet/profile/default`.

- `config.yml` General and Package-specific Configs
- `keybind.yml` Keybind Configs
- `workspace.yml` Workspace Configs (Window Size, Layout, Filter History, etc.)

## Packages

Installed packages are located in `$HOME/.genet/package`.

## Display Filter

Genet has a simple DSL(Domain Specific Language) to filter packets. 

It is very similar to JavaScript.
(And actually, it will be transpiled into JavaScript and executed on V8 JIT!)

Display filter applies the filter program to each frame and shows them only if the result value is _truthy_. Therefore, following filters do nothing: always show all frames.

- `true`
- `!0`
- `1`
- `1 + 2 == 3`
- `'ok!'`

There are some language extensions suitable for the packet filtering:

- Extended operators
- Pipeline syntax
- Macro syntax

### Extended operators

Some operators (`==`, `===`, `!=`, `!==`, `<`, `<=`, `>`, `>=`) can take an iterable object as the operand.

```js
ipv4.src == [127, 0, 0, 1] // IPv4 source address equals 127.0.0.1
```

### Pipeline syntax

Pipeline syntax provides a convenient way to transfrom the returned value.

```js
http.path lower == '/login'

// almost equivalent to:
('toLowerCase' in Object(http.path)) 
  ? http.path.toLowerCase()
  : http.path
    === '/login'
```

Use `:` to pass arguments:

```js
http.path slice:0:6 == '/login'

// almost equivalent to:
('slice' in Object(http.path)) 
  ? http.path.slice(0, 6)
  : http.path
    == '/login'
```

<p class="warning">
Note that you can not write a method call like `http.path.toLowerCase()` because Genet resolves `http.path.toLowerCase` as a layer attribute named `http.path.toLowerCase`.
</p>

#### Builtin Manipulators

There are some built-in manipulator functions:

- `length`
  - Return the `length` property if it exists. Otherwise, return `0`.
- `slice(begin, end)`
  - Apply the `slice` function if it exists.
- `lower`
  - Apply the `toLowerCase` function if it exists.
- `upper`
  - Apply the `toUpperCase` function if it exists.

### Macro syntax

Macro provides a familiar way to write compound constants.

For example, an IPv4 address is represented as just an array of integers but you are also able to write it as `@127.0.0.1`. That will be converted into `[127, 0, 0, 1]` before parsing.

```js
ipv4.src == @127.0.0.1 // Same as ipv4.src == [127, 0, 0, 1]
```

A macro expression starts with `@` and ends with whitespace or the line ending. You can create an extension package to add custom macros, of course.

There are some macros defined in the built-in packages:

| Name | Format | Example | Expanded |
|------|--------|---------|----------|
|MAC Address|`@XX:XX:XX:XX:XX:XX`|`@11:22:33:44:55:66`|`[0x11, 0x22, 0x33, 0x44, 0x55, 0x66]`|
|IPv4 Address|`@X.X.X.X`|`@127.0.0.1`|`[127, 0, 0, 1]`|
|IPv6 Address|`@XXXX:XXXX:...:XXXX`|`@::ffff`|`[0, 0, ..., 0, 0, 0xff, 0xff]`|
|DateTime|`@YYYY-MM-DDThh:mm:ss`|`@2018-03-01T00:00:00`|`1519862400000` (Depends on timezone)|


### Examples

#### Protocol

| Expression | Description |
|------------|-------------|
| `tcp` | TCP only |
| `udp` | UDP only |
| `udp && ipv6` | UDP over IPv6 only |
| <code>udp &#124;&#124; arp</code> | UDP or ARP |
| `!arp` | All frames except ARP |

#### Attributes

| Expression | Description |
|------------|-------------|
| `tcp.flags.ack` | TCP frames with the ACK flag |
| `tcp.flags & 0b000010000` | TCP frames with the ACK flag |
| `tcp.flags.ack && tcp.flags.fin` | TCP frames with the ACK and the FIN flag |
| `tcp.flags == 16` | TCP frames with only the ACK flag |
| <code>tcp.src < 1024 &#124;&#124; tcp.dst < 1024</code> | TCP with well-known ports |
| `http.method.get` | HTTP GET Requests |

#### Frame Metadata

| Expression | Description |
|------------|-------------|
| `$.actualLength > 1024` | Actual frame length is larger than 1024 |
| `$.timestamp < @2018-03-15T22:00:00` | Frame timestamp is before `2018-03-15T22:00:00` |

## Confidence

Confidence is a 2-bit value represents an accuracy of the dissected layer.

| Confidence | Value | Description | Example |
|------------|-------|-------------|---------|
| Exact | 3 | Protocol is specified in the lower layer. | IPv4 over Ethernet |
| Probable | 2 | Protocol could not be determined precisely, but a usual port or parameter is used. | SSH on TCP port 20 |
| Possible | 1 | Protocol is decodable but an unusual port or parameter is used. | SSH on TCP port 5555 |
| Error | 0 | Protocol has an invalid parameter. | - |

Increasing the confidence threshold level produces cleaner results.
On the other hand, the lower level is a better choice for catching broken/camouflaged packets.
