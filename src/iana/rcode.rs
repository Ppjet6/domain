//! DNS response codes and extended response codes.
//!
//! The original DNS specification in [RFC 1035] specified four bits of the
//! message header as response code. The type [Rcode] defined herein
//! represents these codes. Later, [RFC 2671] (now [RFC 6891]) added eight
//! bits to the response code to be transmitted as part of the OPT
//! pseudo-resource record. To make matters even worse, the TSIG and TKEY
//! records defined by [RFC 2845] and [RFC 2930] use a 16 bit error code.
//! All of these codes share the same defition space. Even so, we have
//! separate types for each of these.
//!
//! [Rcode]: 
//! [RFC 2671]: https://tools.ietf.org/html/rfc2671
//! [RFC 2845]: https://tools.ietf.org/html/rfc2845
//! [RFC 2930]: https://tools.ietf.org/html/rfc2930
//! [RFC 6891]: https://tools.ietf.org/html/rfc6891
//!

use std::cmp;
use std::fmt;
use std::hash;


//------------ Rcode --------------------------------------------------------

/// DNS Response Codes.
///
/// The response code of a response indicates what happend on the server
/// when trying to answer the query. The code is a 4 bit value and part of
/// the header of a DNS message.
///
/// This response was defined as part of [RFC 1035]. Later, [RFC 2671]
/// defined an extended response code of 12 bits using the lower four bits
/// from the header and eight additional bits stored in the OPT
/// pseudo-record. The type [OptRcode] represents this extended response
/// code. A third response code, now 16 bit wide, was defined for the
/// transaction authentication mechansim (TSIG) in [RFC 2845] and is
/// represented by [TsigRcode].
///
/// All three codes share the same name space. Their values are defined in
/// one registry, [IANA DNS RCODEs].
///
/// [OptRcode]: enum.OptRcode.html
/// [TsigRcode]: enum.TsigRcode.html
/// [IANA DNS RCODEs]: http://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-6
/// [RFC 1035]: https://tools.ietf.org/html/rfc1035
/// [RFC 2671]: https://tools.ietf.org/html/rfc2671
#[derive(Clone, Copy, Debug)]
pub enum Rcode {
    /// No error condition.
    ///
    /// (Otherwise known as success.)
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NoError,

    /// Format error.
    ///
    /// The name server was unable to interpret the query.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    FormErr,

    /// Server failure.
    ///
    /// The name server was unable to process this query due to a problem
    /// with the name server.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    ServFail,

    /// Name error.
    ///
    /// The domain name given in the query does not exist at the name server.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NXDomain,

    /// Not implemented.
    ///
    /// The name server does not support the requested kind of query.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NotImp,

    /// Query refused.
    ///
    /// The name server refused to perform the operation requested by the
    /// query for policy reasons.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    Refused,

    /// Name exists when it should not.
    ///
    /// Returned for an UPDATE query when a domain requested to not exist
    /// does in fact exist.
    ///
    /// Returned when resolving a DNAME redirection when the resulting name
    /// exceeds the length of 255 octets.
    ///
    /// Defined in [RFC 2136] for the UPDATE query and [RFC 6672] for DNAME
    /// redirection.
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    /// [RFC 6672]: https://tools.ietf.org/html/rfc6672
    YXDomain,

    /// RR set exists when it should not.
    ///
    /// Returned for an UPDATE query when an RRset requested to not exist
    /// does in fact exist.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    YXRRSet,

    /// RR set that should exist does not.
    ///
    /// Returned for an UPDATE query when an RRset requested to exist
    /// does not.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    NXRRSet,

    /// Server not authoritative for zone or client not authorized.
    ///
    /// Returned for an UPDATE query when the server is not an authoritative
    /// name server for the requested domain.
    ///
    /// Returned for queries using TSIG when authorisation failed.
    ///
    /// Defined in [RFC 2136] for UPDATE and [RFC 2845] for TSIG.
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    /// [RFC 2845]: https://tools.ietf.org/html/rfc2845
    NotAuth,

    /// Name not contained in zone.
    ///
    /// A name used in the prerequisite or update section is not within the
    /// zone given in the zone section.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    NotZone,

    /// A raw, integer rcode value.
    ///
    /// When converting to an `u8`, only the lower four bits are used.
    Int(u8)
}

impl Rcode {
    /// Creates an rcode from an integer.
    ///
    /// Only the lower four bits of `value` are considered.
    pub fn from_int(value: u8) -> Rcode {
        use self::Rcode::*;

        match value & 0x0F {
            0 => NoError,
            1 => FormErr,
            2 => ServFail,
            3 => NXDomain,
            4 => NotImp,
            5 => Refused,
            6 => YXDomain,
            7 => YXRRSet,
            8 => NXRRSet,
            9 => NotAuth,
            10 => NotZone,
            value => Int(value)
        }
    }

    /// Returns the integer value for this rcode.
    pub fn to_int(self) -> u8 {
        use self::Rcode::*;

        match self {
            NoError => 0,
            FormErr => 1,
            ServFail => 2,
            NXDomain => 3,
            NotImp => 4,
            Refused => 5,
            YXDomain => 6,
            YXRRSet => 7,
            NXRRSet => 8,
            NotAuth => 9,
            NotZone => 10,
            Int(value) => value & 0x0F
        }
    }
}


//--- From

impl From<u8> for Rcode {
    fn from(value: u8) -> Rcode { Rcode::from_int(value) }
}

impl From<Rcode> for u8 {
    fn from(value: Rcode) -> u8 { value.to_int() }
}


//--- Display

impl fmt::Display for Rcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Rcode::*;

        match *self {
            NoError => "NOERROR".fmt(f),
            FormErr => "FORMERR".fmt(f),
            ServFail => "SERVFAIL".fmt(f),
            NXDomain => "NXDOMAIN".fmt(f),
            NotImp => "NOTIMP".fmt(f),
            Refused => "REFUSED".fmt(f),
            YXDomain => "YXDOMAIN".fmt(f),
            YXRRSet => "YXRRSET".fmt(f),
            NXRRSet => "NXRRSET".fmt(f),
            NotAuth => "NOAUTH".fmt(f),
            NotZone => "NOTZONE".fmt(f),
            Int(i) => {
                match Rcode::from_int(i) {
                    Rcode::Int(i) => i.fmt(f),
                    value => value.fmt(f)
                }
            }
        }
    }
}


//--- PartialEq and Eq

impl cmp::PartialEq for Rcode {
    fn eq(&self, other: &Rcode) -> bool {
        self.to_int() == other.to_int()
    }
}

impl cmp::PartialEq<u8> for Rcode {
    fn eq(&self, other: &u8) -> bool {
        self.to_int() == *other
    }
}

impl cmp::PartialEq<Rcode> for u8 {
    fn eq(&self, other: &Rcode) -> bool {
        *self == other.to_int()
    }
}

impl cmp::Eq for Rcode { }


//--- PartialOrd and Ord

impl cmp::PartialOrd for Rcode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.to_int().partial_cmp(&other.to_int())
    }
}

impl cmp::PartialOrd<u8> for Rcode {
    fn partial_cmp(&self, other: &u8) -> Option<cmp::Ordering> {
        self.to_int().partial_cmp(other)
    }
}

impl cmp::PartialOrd<Rcode> for u8 {
    fn partial_cmp(&self, other: &Rcode) -> Option<cmp::Ordering> {
        self.partial_cmp(&other.to_int())
    }
}

impl cmp::Ord for Rcode {
    fn cmp(&self, other: &Rcode) -> cmp::Ordering {
        self.to_int().cmp(&other.to_int())
    }
}


//--- Hash

impl hash::Hash for Rcode {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.to_int().hash(state)
    }
}


//------------ OptRcode -----------------------------------------------------

/// Extended DNS Response Codes for OPT records.
///
/// Originally, the response code of embedded in the header of each DNS
/// message was four bits long. This code, defined in [RFC 1035], is
/// represented by the [Rcode] type. The extension mechanism for DNS
/// initially defined in [RFC 2671] and updated by [RFC 6891] added eight
/// more bits to be stored in the OPT pseudo-resource record. This type
/// represents the complete 12 bit extended response code.
///
/// There is a third, 16 bit wide response code for transaction
/// authentication (TSIG) defined in [RFC 2845] and represented by the
/// [`TsigRcode`] type. The code mostly shares the same name space except
/// for an unfortunate collision in between the BADVERS and BADSIG values.
/// Because of this, we decided to have separate types.
///
/// The values for all three response code types are defined in
/// the [IANA DNS RCODEs] registry.
///
/// [Rcode]: enum.Rcode.html
/// [`TsigRcode`]: enum.TsigRcode.html
/// [IANA DNS RCODEs]: http://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-6
/// [RFC 2671]: https://tools.ietf.org/html/rfc2671
/// [RFC 2845]: https://tools.ietf.org/html/rfc2845
/// [RFC 2930]: https://tools.ietf.org/html/rfc2930
/// [RFC 6891]: https://tools.ietf.org/html/rfc6891
#[derive(Clone, Copy, Debug)]
pub enum OptRcode {
    /// No error condition.
    ///
    /// (Otherwise known as success.)
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NoError,

    /// Format error.
    ///
    /// The name server was unable to interpret the query.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    FormErr,

    /// Server failure.
    ///
    /// The name server was unable to process this query due to a problem
    /// with the name server.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    ServFail,

    /// Name error.
    ///
    /// The domain name given in the query does not exist at the name server.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NXDomain,

    /// Not implemented.
    ///
    /// The name server does not support the requested kind of query.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NotImp,

    /// Query refused.
    ///
    /// The name server refused to perform the operation requested by the
    /// query for policy reasons.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    Refused,

    /// Name exists when it should not.
    ///
    /// Returned for an UPDATE query when a domain requested to not exist
    /// does in fact exist.
    ///
    /// Returned when resolving a DNAME redirection when the resulting name
    /// exceeds the length of 255 octets.
    ///
    /// Defined in [RFC 2136] for the UPDATE query and [RFC 6672] for DNAME
    /// redirection.
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    /// [RFC 6672]: https://tools.ietf.org/html/rfc6672
    YXDomain,

    /// RR set exists when it should not.
    ///
    /// Returned for an UPDATE query when an RRset requested to not exist
    /// does in fact exist.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    YXRRSet,

    /// RR set that should exist does not.
    ///
    /// Returned for an UPDATE query when an RRset requested to exist
    /// does not.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    NXRRSet,

    /// Server not authoritative for zone or client not authorized.
    ///
    /// Returned for an UPDATE query when the server is not an authoritative
    /// name server for the requested domain.
    ///
    /// Returned for queries using TSIG when authorisation failed.
    ///
    /// Defined in [RFC 2136] for UPDATE and [RFC 2845] for TSIG.
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    /// [RFC 2845]: https://tools.ietf.org/html/rfc2845
    NotAuth,

    /// Name not contained in zone.
    ///
    /// A name used in the prerequisite or update section is not within the
    /// zone given in the zone section.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    NotZone,

    /// Bad OPT version.
    ///
    /// A name server does not implement the EDNS version requested in the
    /// OPT record.
    ///
    /// Defined in [RFC 6891].
    ///
    /// [RFC 6891]: https://tools.ietf.org/html/rfc6891
    BadVers,

    // XXX We will not define the values from the TSIG and TKEY RFCs,
    //     unless are used in OPT records, too?

    /// Bad or missing server cookie.
    ///
    /// The request contained a COOKIE option either without a server cookie
    /// or with a server cookie that did not validate.
    ///
    /// Defined in [RFC 7873].
    ///
    /// [RFC 7873]: https://tools.ietf.org/html/rfc7873
    BadCookie,

    /// A raw, integer rcode value.
    ///
    /// When converting to a 12 bit code, the upper four bits are simply
    /// ignored.
    Int(u16)
}


impl OptRcode {
    /// Creates an rcode from an integer.
    ///
    /// Only the lower twelve bits of `value` are considered.
    pub fn from_int(value: u16) -> OptRcode {
        use self::OptRcode::*;

        match value & 0x0FFF {
            0 => NoError,
            1 => FormErr,
            2 => ServFail,
            3 => NXDomain,
            4 => NotImp,
            5 => Refused,
            6 => YXDomain,
            7 => YXRRSet,
            8 => NXRRSet,
            9 => NotAuth,
            10 => NotZone,
            16 => BadVers,
            23 => BadCookie,
            value => Int(value)
        }
    }

    /// Returns the integer value for this rcode.
    pub fn to_int(self) -> u16 {
        use self::OptRcode::*;

        match self {
            NoError => 0,
            FormErr => 1,
            ServFail => 2,
            NXDomain => 3,
            NotImp => 4,
            Refused => 5,
            YXDomain => 6,
            YXRRSet => 7,
            NXRRSet => 8,
            NotAuth => 9,
            NotZone => 10,
            BadVers => 16,
            BadCookie => 23,
            Int(value) => value & 0x0F
        }
    }

    /// Creates an extended rcode value from its parts.
    pub fn from_parts(rcode: Rcode, ext: u8) -> OptRcode {
        if ext == 0 {
            rcode.into()
        }
        else {
            OptRcode::from_int((ext as u16) << 4 | (rcode.to_int() as u16))
        }
    }

    /// Returns the two parts of an extended rcode value.
    pub fn to_parts(self) -> (Rcode, u8) {
        let res = self.to_int();
        (Rcode::from_int(res as u8), (res >> 8) as u8)
    }

    /// Returns the rcode part of the extended rcode.
    pub fn rcode(&self) -> Rcode {
        self.to_parts().0
    }

    /// Returns the extended octet of the extended rcode.
    pub fn ext(&self) -> u8 {
        self.to_parts().1
    }
}


//--- From

impl From<u16> for OptRcode {
    fn from(value: u16) -> OptRcode { OptRcode::from_int(value) }
}

impl From<OptRcode> for u16 {
    fn from(value: OptRcode) -> u16 { value.to_int() }
}

impl From<Rcode> for OptRcode {
    fn from(value: Rcode) -> OptRcode { OptRcode::from_parts(value, 0) }
}


//--- Display

impl fmt::Display for OptRcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::OptRcode::*;

        match *self {
            NoError => "NOERROR".fmt(f),
            FormErr => "FORMERR".fmt(f),
            ServFail => "SERVFAIL".fmt(f),
            NXDomain => "NXDOMAIN".fmt(f),
            NotImp => "NOTIMP".fmt(f),
            Refused => "REFUSED".fmt(f),
            YXDomain => "YXDOMAIN".fmt(f),
            YXRRSet => "YXRRSET".fmt(f),
            NXRRSet => "NXRRSET".fmt(f),
            NotAuth => "NOAUTH".fmt(f),
            NotZone => "NOTZONE".fmt(f),
            BadVers => "BADVER".fmt(f),
            BadCookie => "BADCOOKIE".fmt(f),
            Int(i) => {
                match OptRcode::from_int(i) {
                    Int(i) => i.fmt(f),
                    value => value.fmt(f)
                }
            }
        }
    }
}



//------------ TsigRcode ----------------------------------------------------

/// Response codes for transaction authentication (TSIG).
///
/// TSIG and TKEY resource records contain a 16 bit wide error field whose
/// values are an extension of the standard DNS [`Rcode`]. While it was
/// intended to also share the same space with the extended response codes
/// used by EDNS (see [`OptRcode`]), both used the value 16. To allow
/// distinguish between the two uses of this value, we have two separate
/// types.
///
/// The values for all three response code types are defined in
/// the [IANA DNS RCODEs] registry.
///
/// [`Rcode?]: enum.Rcode.html
/// [`OptRcode`]: enum.OptRcode.html
/// [IANA DNS RCODEs]: http://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-6
#[derive(Clone, Copy, Debug)]
pub enum TsigRcode {
    /// No error condition.
    ///
    /// (Otherwise known as success.)
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NoError,

    /// Format error.
    ///
    /// The name server was unable to interpret the query.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    FormErr,

    /// Server failure.
    ///
    /// The name server was unable to process this query due to a problem
    /// with the name server.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    ServFail,

    /// Name error.
    ///
    /// The domain name given in the query does not exist at the name server.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NXDomain,

    /// Not implemented.
    ///
    /// The name server does not support the requested kind of query.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    NotImp,

    /// Query refused.
    ///
    /// The name server refused to perform the operation requested by the
    /// query for policy reasons.
    ///
    /// Defined in [RFC 1035].
    ///
    /// [RFC 1035]: https://tools.ietf.org/html/rfc1035
    Refused,

    /// Name exists when it should not.
    ///
    /// Returned for an UPDATE query when a domain requested to not exist
    /// does in fact exist.
    ///
    /// Returned when resolving a DNAME redirection when the resulting name
    /// exceeds the length of 255 octets.
    ///
    /// Defined in [RFC 2136] for the UPDATE query and [RFC 6672] for DNAME
    /// redirection.
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    /// [RFC 6672]: https://tools.ietf.org/html/rfc6672
    YXDomain,

    /// RR set exists when it should not.
    ///
    /// Returned for an UPDATE query when an RRset requested to not exist
    /// does in fact exist.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    YXRRSet,

    /// RR set that should exist does not.
    ///
    /// Returned for an UPDATE query when an RRset requested to exist
    /// does not.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    NXRRSet,

    /// Server not authoritative for zone or client not authorized.
    ///
    /// Returned for an UPDATE query when the server is not an authoritative
    /// name server for the requested domain.
    ///
    /// Returned for queries using TSIG when authorisation failed.
    ///
    /// Defined in [RFC 2136] for UPDATE and [RFC 2845] for TSIG.
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    /// [RFC 2845]: https://tools.ietf.org/html/rfc2845
    NotAuth,

    /// Name not contained in zone.
    ///
    /// A name used in the prerequisite or update section is not within the
    /// zone given in the zone section.
    ///
    /// Defined in [RFC 2136].
    ///
    /// [RFC 2136]: https://tools.ietf.org/html/rfc2136
    NotZone,

    /// TSIG signature failure.
    ///
    /// The TSIG signature fails to verify.
    ///
    /// Defined in [RFC 2845].
    ///
    /// [RFC 2845]: https://tools.ietf.org/html/rfc2845
    BadSig,

    /// Key not recognized.
    ///
    /// The server did not recognize the key used for generating the
    /// signature.
    ///
    /// Defined in [RFC 2845].
    ///
    /// [RFC 2845]: https://tools.ietf.org/html/rfc2845
    BadKey,

    /// Signature out of time window.
    ///
    /// The server time was outside the time interval specified by the
    /// request.
    ///
    /// Defined in [RFC 2845].
    ///
    /// [RFC 2845]: https://tools.ietf.org/html/rfc2845
    BadTime,

    /// Bad TKEY mode.
    ///
    /// The mode field in a TKEY resource record contained a mode not
    /// supported by the server.
    ///
    /// Defined in [RFC 2930].
    ///
    /// [RFC 2930]: https://tools.ietf.org/html/rfc2930
    BadMode,

    /// Duplicate key name.
    ///
    /// In TKEY records, when establishing a new key, the name used already
    /// exists at the server or when deleting a key, a key of this name does
    /// not exist.
    ///
    /// Defined in [RFC 2930].
    ///
    /// [RFC 2930]: https://tools.ietf.org/html/rfc2930
    BadName,

    /// Algorithm not supported.
    ///
    /// The value is defined in [RFC 2930] but never actually explained.
    /// Presumably, it will be returned when the algorithm field of a TKEY
    /// record contains a value not supported by the server.
    ///
    /// [RFC 2930]: https://tools.ietf.org/html/rfc2930
    BadAlg,

    /// Bad truncation.
    ///
    /// A TSIG record was received with a MAC too short for the local
    /// policy in force.
    ///
    /// Defined in [RFC 4635].
    ///
    /// [RFC 4635]: https://tools.ietf.org/html/rfc4635
    BadTrunc,

    /// Bad or missing server cookie.
    ///
    /// The request contained a COOKIE option either without a server cookie
    /// or with a server cookie that did not validate.
    ///
    /// Defined in [RFC 7873].
    ///
    /// [RFC 7873]: https://tools.ietf.org/html/rfc7873
    BadCookie,

    /// A raw, integer rcode value.
    ///
    /// When converting to a 12 bit code, the upper four bits are simply
    /// ignored.
    Int(u16)
}


impl TsigRcode {
    /// Creates an rcode from an integer.
    pub fn from_int(value: u16) -> TsigRcode {
        use self::TsigRcode::*;

        match value {
            0 => NoError,
            1 => FormErr,
            2 => ServFail,
            3 => NXDomain,
            4 => NotImp,
            5 => Refused,
            6 => YXDomain,
            7 => YXRRSet,
            8 => NXRRSet,
            9 => NotAuth,
            10 => NotZone,
            16 => BadSig,
            17 => BadKey,
            18 => BadTime,
            19 => BadMode,
            20 => BadName,
            21 => BadAlg,
            22 => BadTrunc,
            23 => BadCookie,
            value => Int(value)
        }
    }

    /// Returns the integer value for this rcode.
    pub fn to_int(self) -> u16 {
        use self::TsigRcode::*;

        match self {
            NoError => 0,
            FormErr => 1,
            ServFail => 2,
            NXDomain => 3,
            NotImp => 4,
            Refused => 5,
            YXDomain => 6,
            YXRRSet => 7,
            NXRRSet => 8,
            NotAuth => 9,
            NotZone => 10,
            BadSig => 16,
            BadKey => 17,
            BadTime => 18,
            BadMode => 19,
            BadName => 20,
            BadAlg => 21,
            BadTrunc => 22,
            BadCookie => 23,
            Int(value) => value & 0x0F
        }
    }
}


//--- From

impl From<u16> for TsigRcode {
    fn from(value: u16) -> TsigRcode { TsigRcode::from_int(value) }
}

impl From<TsigRcode> for u16 {
    fn from(value: TsigRcode) -> u16 { value.to_int() }
}

impl From<Rcode> for TsigRcode {
    fn from(value: Rcode) -> TsigRcode {
        TsigRcode::from_int(value.to_int() as u16)
    }
}

impl From<OptRcode> for TsigRcode {
    fn from(value: OptRcode) -> TsigRcode {
        TsigRcode::from_int(value.to_int())
    }
}


//--- Display

impl fmt::Display for TsigRcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TsigRcode::*;

        match *self {
            NoError => "NOERROR".fmt(f),
            FormErr => "FORMERR".fmt(f),
            ServFail => "SERVFAIL".fmt(f),
            NXDomain => "NXDOMAIN".fmt(f),
            NotImp => "NOTIMP".fmt(f),
            Refused => "REFUSED".fmt(f),
            YXDomain => "YXDOMAIN".fmt(f),
            YXRRSet => "YXRRSET".fmt(f),
            NXRRSet => "NXRRSET".fmt(f),
            NotAuth => "NOAUTH".fmt(f),
            NotZone => "NOTZONE".fmt(f),
            BadSig => "BADSIG".fmt(f),
            BadKey => "BADKEY".fmt(f),
            BadTime => "BADTIME".fmt(f),
            BadMode => "BADMODE".fmt(f),
            BadName => "BADNAME".fmt(f),
            BadAlg => "BADALG".fmt(f),
            BadTrunc => "BADTRUNC".fmt(f),
            BadCookie => "BADCOOKIE".fmt(f),
            Int(i) => {
                match TsigRcode::from_int(i) {
                    Int(i) => i.fmt(f),
                    value => value.fmt(f)
                }
            }
        }
    }
}

