# friendlyid converter
The FriendlyID application that converts a given UUID to a URL-friendly ID which is based on Base62


### Sample usage

    $ friendlyid c3587ec5-0976-497f-8374-61e0c2ea3da5                                                                                                      ✔ 
    5wbwf6yUxVBcr48AMbz9cb

    $ friendlyid 5wbwf6yUxVBcr48AMbz9cb                                                                                                                    ✔ 
    c3587ec5-0976-497f-8374-61e0c2ea3da5

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
