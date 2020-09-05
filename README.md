# friendlyid converter
The FriendlyID application that converts a given UUID to a URL-friendly ID which is based on Base62


## Sample usage

### Convert UUID to FriendlyID
    $ friendlyid c3587ec5-0976-497f-8374-61e0c2ea3da5                                                                                                      ✔ 
    5wbwf6yUxVBcr48AMbz9cb
  
### Convert FriendlyID to UUID

    $ friendlyid 5wbwf6yUxVBcr48AMbz9cb                                                                                                                    ✔ 
    c3587ec5-0976-497f-8374-61e0c2ea3da5

### Display help

    $ friendlyid -h                                                                                                                                        ✔ 
    FriendlyId Converter 0.1.0
    Mariusz Smykula <mariuszs@gmail.com>
    The FriendlyID library converts a given UUID to a URL-friendly ID which is based on Base62

    USAGE:
        friendlyid [FLAGS] <ID>

    ARGS:
        <ID>    ID to convert

    FLAGS:
        -d, --decode     Decode friendlyId
        -h, --help       Prints help information
        -V, --version    Prints version information

## Installation

### Linux

#### Build from the source using cargo and rustc

    cargo build --release
    
#### Download binary from github    

Use binary `friendlyid` downloaded from https://github.com/mariuszs/rust-friendlyid/releases

#### Use package appropriate for your linux distribution (RPM or DEB)

Download and install DEB or RPM from https://github.com/mariuszs/rust-friendlyid/releases
   
    
    