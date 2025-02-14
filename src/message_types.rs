/// These are SSH message type numbers. They are scattered around several
/// documents but many were taken from [SSH-PARAMETERS].
pub struct MessageTypeNumber;

impl MessageTypeNumber {
    /// RFC 4253 section 12.
    pub const MSG_DISCONNECT: u8 = 1;
    /// RFC 4253 section 12.
    pub const MSG_IGNORE: u8 = 2;
    /// RFC 4253 section 12.
    pub const MSG_UNIMPLEMENTED: u8 = 3;
    /// RFC 4253 section 12.
    pub const MSG_DEBUG: u8 = 4;
    /// See RFC 4253, section 10.
    pub const MSG_SERVICE_REQUEST: u8 = 5;
    /// See RFC 4253, section 10.
    pub const MSG_SERVICE_ACCEPT: u8 = 6;
    /// See RFC 8308, section 2.3
    pub const MSG_EXT_INFO: u8 = 7;
    /// See RFC 4253, section 7.1.
    pub const MSG_KEX_INIT: u8 = 20;
    /// RFC 4253 section 12.
    pub const MSG_NEW_KEYS: u8 = 21;
    /// See RFC 4253, section 8.
    pub const MSG_KEX_DH_INIT: u8 = 30;
    // See RFC 5656, section 7.
    pub const MSG_KEX_ECDH_INIT: u8 = 30;
    /// See RFC 5656, section 7.
    pub const MSG_KEX_ECDH_REPLY: u8 = 31;
    /// See RFC 4253, section 8.
    pub const MSG_KEX_DH_REPLY: u8 = 31;
    pub const MSG_KEX_DH_GEX_GROUP: u8 = 31;
    pub const MSG_KEX_DH_GEX_INIT: u8 = 32;
    pub const MSG_KEX_DH_GEX_REPLY: u8 = 33;
    pub const MSG_KEX_DH_GEX_REQUEST: u8 = 34;
    /// See RFC 4252, section 5.
    pub const MSG_USER_AUTH_REQUEST: u8 = 50;
    /// See RFC 4252, section 5.1
    pub const MSG_USER_AUTH_FAILURE: u8 = 51;
    /// See RFC 4252, section 5.1
    pub const MSG_USER_AUTH_SUCCESS: u8 = 52;
    /// See RFC 4252, section 5.4
    pub const MSG_USER_AUTH_BANNER: u8 = 53;
    /// See RFC 4256, section 3.2
    pub const MSG_USER_AUTH_INFO_REQUEST: u8 = 60;
    /// See RFC 4256, section 3.2
    pub const MSG_USER_AUTH_INFO_RESPONSE: u8 = 61;
    /// See RFC 4252, section 7
    pub const MSG_USER_AUTH_PUB_KEY_OK: u8 = 60;
    /// See RFC 4462, section 3
    pub const MSG_USER_AUTH_GSSAPI_RESPONSE: u8 = 60;
    pub const MSG_USER_AUTH_GSSAPI_TOKEN: u8 = 61;
    pub const MSG_USER_AUTH_GSSAPI_MIC: u8 = 66;
    /// See RFC 4462, section 3.9
    pub const MSG_USER_AUTH_GSSAPI_ERR_TOK: u8 = 64;
    // See RFC 4462, section 3.8
    pub const MSG_USER_AUTH_GSSAPI_ERROR: u8 = 65;
    /// See RFC 4254, section 4
    pub const MSG_GLOBAL_REQUEST: u8 = 80;
    /// See RFC 4254, section 4
    pub const MSG_REQUEST_SUCCESS: u8 = 81;
    /// See RFC 4254, section 4
    pub const MSG_REQUEST_FAILURE: u8 = 82;
    /// See RFC 4254, section 5.1.
    pub const MSG_CHANNEL_OPEN: u8 = 90;
    /// See RFC 4254, section 5.1.
    pub const MSG_CHANNEL_OPEN_CONFIRM: u8 = 91;
    /// See RFC 4254, section 5.1
    pub const MSG_CHANNEL_OPEN_FAILURE: u8 = 92;
    /// See RFC 4254, section 5.2
    pub const MSG_CHANNEL_WINDOW_ADJUST: u8 = 93;
    pub const MSG_CHANNEL_DATA: u8 = 94;
    pub const MSG_CHANNEL_EXTENDED_DATA: u8 = 95;
    /// See RFC 4254, section 5.3
    pub const MSG_CHANNEL_EOF: u8 = 96;
    /// See RFC 4254, section 5.3
    pub const MSG_CHANNEL_CLOSE: u8 = 97;
    pub const MSG_CHANNEL_REQUEST: u8 = 98;
    /// See RFC 4254, section 5.4.
    pub const MSG_CHANNEL_SUCCESS: u8 = 99;
    /// See RFC 4254, section 5.4.
    pub const MSG_CHANNEL_FAILURE: u8 = 100;
    /// Transport layer OpenSSH extension. See [PROTOCOL], section 1.9
    pub const MSG_PING: u8 = 192;
    /// Transport layer OpenSSH extension. See [PROTOCOL], section 1.9
    pub const MSG_PONG: u8 = 193;
}
