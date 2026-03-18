[![Copr build status](https://copr.fedorainfracloud.org/coprs/mariuszs/rust/package/friendlyid/status_image/last_build.png)](https://copr.fedorainfracloud.org/coprs/mariuszs/rust/package/friendlyid/)

# friendlyid converter
The FriendlyID tool that converts a given UUID to a URL-friendly ID which is based on Base62


## Sample usage

### Convert UUID to FriendlyID
    $ friendlyid c3587ec5-0976-497f-8374-61e0c2ea3da5
    5wbwf6yUxVBcr48AMbz9cb

### Convert FriendlyID to UUID

    $ friendlyid 5wbwf6yUxVBcr48AMbz9cb
    c3587ec5-0976-497f-8374-61e0c2ea3da5

### Generate new FriendlyID

    $ friendlyid
    2IxLZbPQmMuamgy3tapZsM

### Display help

    $ friendlyid -h
    FriendlyID tool — converts UUIDs to URL-friendly Base62 IDs and vice versa

    Usage: friendlyid [UUID|FRIENDLYID]

    Arguments:
      [UUID|FRIENDLYID]  UUID to encode, or FriendlyID to decode. Omit to generate a new FriendlyID

    Options:
      -h, --help     Print help
      -V, --version  Print version

## Installation

### Fedora - install using package manager

    $ sudo dnf copr enable mariuszs/rust
    $ sudo dnf install friendlyid

### Install using cargo

    $ cargo install friendlyid

#### Download binary from github (linux or windows)

Use binary `friendlyid` or `friendlyid.exe` downloaded from https://github.com/mariuszs/rust-friendlyid/releases

#### Use package appropriate for your linux distribution (RPM or DEB)

Download and install DEB or RPM from https://github.com/mariuszs/rust-friendlyid/releases
