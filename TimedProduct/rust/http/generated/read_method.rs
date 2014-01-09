// This automatically generated file is included in request.rs.
{
    use method::{Connect, Delete, Get, Head, Options, Patch, Post, Put, Trace, ExtensionMethod};
    use server::request::MAX_METHOD_LEN;
    use rfc2616::{SP, is_token_item};

    let (s, next_byte) = match self.stream.read_byte() {
        Some(b) if b == 'C' as u8 => match self.stream.read_byte() {
            Some(b) if b == 'O' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'N' as u8 => match self.stream.read_byte() {
                    Some(b) if b == 'N' as u8 => match self.stream.read_byte() {
                        Some(b) if b == 'E' as u8 => match self.stream.read_byte() {
                            Some(b) if b == 'C' as u8 => match self.stream.read_byte() {
                                Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
                                    Some(b) if b == SP => return Some(Connect),
                                    Some(b) if is_token_item(b) => ("CONNECT", b),
                                    _ => return None,
                                },
                                Some(b) if b == SP => return Some(ExtensionMethod(~"CONNEC")),
                                Some(b) if is_token_item(b) => ("CONNEC", b),
                                _ => return None,
                            },
                            Some(b) if b == SP => return Some(ExtensionMethod(~"CONNE")),
                            Some(b) if is_token_item(b) => ("CONNE", b),
                            _ => return None,
                        },
                        Some(b) if b == SP => return Some(ExtensionMethod(~"CONN")),
                        Some(b) if is_token_item(b) => ("CONN", b),
                        _ => return None,
                    },
                    Some(b) if b == SP => return Some(ExtensionMethod(~"CON")),
                    Some(b) if is_token_item(b) => ("CON", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"CO")),
                Some(b) if is_token_item(b) => ("CO", b),
                _ => return None,
            },
            Some(b) if b == SP => return Some(ExtensionMethod(~"C")),
            Some(b) if is_token_item(b) => ("C", b),
            _ => return None,
        },
        Some(b) if b == 'D' as u8 => match self.stream.read_byte() {
            Some(b) if b == 'E' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'L' as u8 => match self.stream.read_byte() {
                    Some(b) if b == 'E' as u8 => match self.stream.read_byte() {
                        Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
                            Some(b) if b == 'E' as u8 => match self.stream.read_byte() {
                                Some(b) if b == SP => return Some(Delete),
                                Some(b) if is_token_item(b) => ("DELETE", b),
                                _ => return None,
                            },
                            Some(b) if b == SP => return Some(ExtensionMethod(~"DELET")),
                            Some(b) if is_token_item(b) => ("DELET", b),
                            _ => return None,
                        },
                        Some(b) if b == SP => return Some(ExtensionMethod(~"DELE")),
                        Some(b) if is_token_item(b) => ("DELE", b),
                        _ => return None,
                    },
                    Some(b) if b == SP => return Some(ExtensionMethod(~"DEL")),
                    Some(b) if is_token_item(b) => ("DEL", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"DE")),
                Some(b) if is_token_item(b) => ("DE", b),
                _ => return None,
            },
            Some(b) if b == SP => return Some(ExtensionMethod(~"D")),
            Some(b) if is_token_item(b) => ("D", b),
            _ => return None,
        },
        Some(b) if b == 'G' as u8 => match self.stream.read_byte() {
            Some(b) if b == 'E' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
                    Some(b) if b == SP => return Some(Get),
                    Some(b) if is_token_item(b) => ("GET", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"GE")),
                Some(b) if is_token_item(b) => ("GE", b),
                _ => return None,
            },
            Some(b) if b == SP => return Some(ExtensionMethod(~"G")),
            Some(b) if is_token_item(b) => ("G", b),
            _ => return None,
        },
        Some(b) if b == 'H' as u8 => match self.stream.read_byte() {
            Some(b) if b == 'E' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'A' as u8 => match self.stream.read_byte() {
                    Some(b) if b == 'D' as u8 => match self.stream.read_byte() {
                        Some(b) if b == SP => return Some(Head),
                        Some(b) if is_token_item(b) => ("HEAD", b),
                        _ => return None,
                    },
                    Some(b) if b == SP => return Some(ExtensionMethod(~"HEA")),
                    Some(b) if is_token_item(b) => ("HEA", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"HE")),
                Some(b) if is_token_item(b) => ("HE", b),
                _ => return None,
            },
            Some(b) if b == SP => return Some(ExtensionMethod(~"H")),
            Some(b) if is_token_item(b) => ("H", b),
            _ => return None,
        },
        Some(b) if b == 'O' as u8 => match self.stream.read_byte() {
            Some(b) if b == 'P' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
                    Some(b) if b == 'I' as u8 => match self.stream.read_byte() {
                        Some(b) if b == 'O' as u8 => match self.stream.read_byte() {
                            Some(b) if b == 'N' as u8 => match self.stream.read_byte() {
                                Some(b) if b == 'S' as u8 => match self.stream.read_byte() {
                                    Some(b) if b == SP => return Some(Options),
                                    Some(b) if is_token_item(b) => ("OPTIONS", b),
                                    _ => return None,
                                },
                                Some(b) if b == SP => return Some(ExtensionMethod(~"OPTION")),
                                Some(b) if is_token_item(b) => ("OPTION", b),
                                _ => return None,
                            },
                            Some(b) if b == SP => return Some(ExtensionMethod(~"OPTIO")),
                            Some(b) if is_token_item(b) => ("OPTIO", b),
                            _ => return None,
                        },
                        Some(b) if b == SP => return Some(ExtensionMethod(~"OPTI")),
                        Some(b) if is_token_item(b) => ("OPTI", b),
                        _ => return None,
                    },
                    Some(b) if b == SP => return Some(ExtensionMethod(~"OPT")),
                    Some(b) if is_token_item(b) => ("OPT", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"OP")),
                Some(b) if is_token_item(b) => ("OP", b),
                _ => return None,
            },
            Some(b) if b == SP => return Some(ExtensionMethod(~"O")),
            Some(b) if is_token_item(b) => ("O", b),
            _ => return None,
        },
        Some(b) if b == 'P' as u8 => match self.stream.read_byte() {
            Some(b) if b == 'A' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
                    Some(b) if b == 'C' as u8 => match self.stream.read_byte() {
                        Some(b) if b == 'H' as u8 => match self.stream.read_byte() {
                            Some(b) if b == SP => return Some(Patch),
                            Some(b) if is_token_item(b) => ("PATCH", b),
                            _ => return None,
                        },
                        Some(b) if b == SP => return Some(ExtensionMethod(~"PATC")),
                        Some(b) if is_token_item(b) => ("PATC", b),
                        _ => return None,
                    },
                    Some(b) if b == SP => return Some(ExtensionMethod(~"PAT")),
                    Some(b) if is_token_item(b) => ("PAT", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"PA")),
                Some(b) if is_token_item(b) => ("PA", b),
                _ => return None,
            },
            Some(b) if b == 'O' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'S' as u8 => match self.stream.read_byte() {
                    Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
                        Some(b) if b == SP => return Some(Post),
                        Some(b) if is_token_item(b) => ("POST", b),
                        _ => return None,
                    },
                    Some(b) if b == SP => return Some(ExtensionMethod(~"POS")),
                    Some(b) if is_token_item(b) => ("POS", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"PO")),
                Some(b) if is_token_item(b) => ("PO", b),
                _ => return None,
            },
            Some(b) if b == 'U' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
                    Some(b) if b == SP => return Some(Put),
                    Some(b) if is_token_item(b) => ("PUT", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"PU")),
                Some(b) if is_token_item(b) => ("PU", b),
                _ => return None,
            },
            Some(b) if b == SP => return Some(ExtensionMethod(~"P")),
            Some(b) if is_token_item(b) => ("P", b),
            _ => return None,
        },
        Some(b) if b == 'T' as u8 => match self.stream.read_byte() {
            Some(b) if b == 'R' as u8 => match self.stream.read_byte() {
                Some(b) if b == 'A' as u8 => match self.stream.read_byte() {
                    Some(b) if b == 'C' as u8 => match self.stream.read_byte() {
                        Some(b) if b == 'E' as u8 => match self.stream.read_byte() {
                            Some(b) if b == SP => return Some(Trace),
                            Some(b) if is_token_item(b) => ("TRACE", b),
                            _ => return None,
                        },
                        Some(b) if b == SP => return Some(ExtensionMethod(~"TRAC")),
                        Some(b) if is_token_item(b) => ("TRAC", b),
                        _ => return None,
                    },
                    Some(b) if b == SP => return Some(ExtensionMethod(~"TRA")),
                    Some(b) if is_token_item(b) => ("TRA", b),
                    _ => return None,
                },
                Some(b) if b == SP => return Some(ExtensionMethod(~"TR")),
                Some(b) if is_token_item(b) => ("TR", b),
                _ => return None,
            },
            Some(b) if b == SP => return Some(ExtensionMethod(~"T")),
            Some(b) if is_token_item(b) => ("T", b),
            _ => return None,
        },
        Some(b) if is_token_item(b) => ("", b),
        _ => return None,
    };
    // OK, that didn't pan out. Let's read the rest and see what we get.
    let mut s = s.to_owned();
    s.push_char(next_byte as char);
    loop {
        match self.stream.read_byte() {
            Some(b) if b == SP => return Some(ExtensionMethod(s)),
            Some(b) if is_token_item(b) => {
                if s.len() == MAX_METHOD_LEN {
                    // Too long; bad request
                    return None;
                }
                s.push_char(b as char);
            },
            _ => return None,
        }
    }
}
