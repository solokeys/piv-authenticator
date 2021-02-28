// https://developers.yubico.com/PIV/Introduction/Yubico_extensions.html

pub const RID_LENGTH: usize = 5;

// top nibble of first byte is "category", here "A" = International
// this category has 5 byte "registered application provider identifier"
// (international RID, the other 9 nibbles is between 0x0 and 0x9).
pub const NIST_RID: &[u8; 5] = &hex!("A000000 308");
pub const YUBICO_RID: &[u8; 5] = &hex!("A000000 527");
// our very own RID (847 = 7*11*11 FWIW)
pub const SOLOKEYS_RID: &[u8; 5] = &hex!("A000000 847");

pub const PIV_APP: [u8; 4] = hex!("0000 1000");
pub const DERIVED_PIV_APP: [u8; 4] = hex!("0000 2000");
pub const PIV_VERSION: [u8; 2] = hex!("0100");
pub const PIV_PIX: [u8; 6] = hex!("0000 1000 0100");
pub const DERIVED_PIV_PIX: [u8; 6] = hex!("0000 2000 0100");

pub const PIV_TRUNCATED_AID: [u8; 9] = hex!("A000000308 00001000");

pub const PIV_AID: &[u8] = &hex!("A000000308 00001000 0100");

pub const DERIVED_PIV_AID: [u8; 11] = hex!("A000000308 00002000 0100");

pub const APPLICATION_LABEL: &[u8] = b"SoloKeys PIV v1.0.0-alpha1";
pub const APPLICATION_URL: &[u8] = b"https://piv.codes/SoloKeys/PIV/1.0.0-alpha1";
// pub const APPLICATION_URL: &[u8] = b"https://piv.is/SoloKeys/PIV/1.0.0-alpha1";


// https://git.io/JfWuD
pub const YUBICO_OTP_PIX: [u8; 3] = hex!("200101");
pub const YUBICO_OTP_AID: [u8; 8] = hex!("A000000527 200101");
// they use it to "deauthenticate user PIN and mgmt key": https://git.io/JfWgN
pub const YUBICO_MGMT_PIX: [u8; 3] = hex!("471117");
pub const YUBICO_MGMT_AID: [u8; 8] = hex!("A000000527 471117");

// https://git.io/JfW28
// const (
// 	// https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-78-4.pdf#page=17
// 	algTag     = 0x80
// 	alg3DES    = 0x03
// 	algRSA1024 = 0x06
// 	algRSA2048 = 0x07
// 	algECCP256 = 0x11
// 	algECCP384 = 0x14

// 	// https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-78-4.pdf#page=16
// 	keyAuthentication     = 0x9a
// 	keyCardManagement     = 0x9b
// 	keySignature          = 0x9c
// 	keyKeyManagement      = 0x9d
// 	keyCardAuthentication = 0x9e
// 	keyAttestation        = 0xf9

// 	insVerify             = 0x20
// 	insChangeReference    = 0x24
// 	insResetRetry         = 0x2c
// 	insGenerateAsymmetric = 0x47
// 	insAuthenticate       = 0x87
// 	insGetData            = 0xcb
// 	insPutData            = 0xdb
// 	insSelectApplication  = 0xa4
// 	insGetResponseAPDU    = 0xc0

// 	// https://github.com/Yubico/yubico-piv-tool/blob/yubico-piv-tool-1.7.0/lib/ykpiv.h#L656
// 	insGetSerial     = 0xf8
// 	insAttest        = 0xf9
// 	insSetPINRetries = 0xfa
// 	insReset         = 0xfb
// 	insGetVersion    = 0xfd
// 	insImportKey     = 0xfe
// 	insSetMGMKey     = 0xff
// )

pub const OK: &[u8; 2] = &[0x90, 0x00];

// pub const SELECT: (u8, u8, u8, u8, usize) = (
pub const SELECT: (u8, u8, u8, u8) = (
    0x00, // interindustry, channel 0, no chain, no secure messaging,
    0xa4, // SELECT
    // p1
    0x04, // data is DF name, may be AID, possibly right-truncated
    // p2: i think this is dummy here
    0x00, // b2, b1 zero means "file occurence": first/only occurence,
          // b4, b3 zero means "file control information": return FCI template
    // 256,
);

//
// See SP 800-73 Part 1, Table 7
// for list of all objects and minimum container capacity
// - CCC: 287
// - CHUID: 2916
// - discovery: 19
// - key history: 256
// - x5c: 1905B
// - etc.
//
// pub const GET_DATA: (u8, u8, u8, u8, usize) = (
pub const GET_DATA: (u8, u8, u8, u8) = (
    0x00, // as before, would be 0x0C for secure messaging
    0xCB, // GET DATA. There's also `CA`, setting bit 1 here
          // means (7816-4, sec. 5.1.2): use BER-TLV, as opposed
          // to "no indication provided".
    // P1, P2: 7816-4, sec. 7.4.1: bit 1 of INS set => P1,P2 identifies
    // a file. And 0x3FFF identifies current DF
    0x3F,
    0xFF,
    // 256,
);

// SW (SP 800-73 Part 1, Table 6)
// == == == == == == == == == == ==
// 61, xx success, more response data bytes
//
// 63, 00 verification failed
// 63, Cx verification failed, x furtehr retries or resets
//
// 68, 82 secure messaging not supported
//
// 69, 82 security status not satisfied
// 69, 83 authn method blocked
// :      (more secure messaging stuff)
//

//// ISO/IEC 7816-4, 5.1.3 "Status bytes"
//#[derive(Copy, Clone, Debug, Eq, PartialEq)]
//pub enum StatusWord {

////////////////////////////////
//// Normal processing (90, 61)
////////////////////////////////

//    // 9000
//    Success,

//    // 61XX
//    MoreAvailable(u8),

/////////////////////////////////
//// Warning processing (62, 63)
/////////////////////////////////

//    // 62XX: state of non-volatile memory unchanged (cf. SW2)

//    // 63XX: state of non-volatile memory changed (cf. SW2)
//    VerificationFailed,
//    FailedRetries(u8),

//////////////////////////////////
//// Execution error (64, 65, 66)
//////////////////////////////////

//    // 64XX: persistent memory unchanged (cf. SW2)
//    // 65XX: persistent memory changed (cf. SW2)
//    // 66XX: security related issues

/////////////////////////////////
//// Checking error (67 - 6F)
/////////////////////////////////

//    // 6700: wrong length, no further indication

//    // 68XX: functions in CLA not supported (cf. SW2)
//    SecureMessagingNotSupported,
//    CommandChainingNotSupported,

//    // 69xx: command not allowed (cf. SW2)
//    SecurityStatusNotSatisfied,
//    OperationBlocked,

//    // 6Axx: wrong parameters P1-P2 (cf. SW2)
//    IncorrectDataParameter,
//    FunctionNotSupported,
//    NotFound,
//    NotEnoughMemory,
//    IncorrectP1OrP2Parameter,
//    KeyReferenceNotFound,

//    // 6BXX: wrong parameters P1-P2

//    // 6CXX: wrong Le field, SW2 encodes available bytes

//    // 6D00: instruction code not supported or invalid
//    InstructionNotSupportedOrInvalid,

//    // 6E00: class not supported
//    ClassNotSupported,

//    // 6F00: no precise diagnosis
//    UnspecifiedCheckingError,
//}

//impl Into<u16> for StatusWord {
//    #[inline]
//    fn into(self) -> u16 {
//        match self {
//            Self::VerificationFailed => 0x6300,
//            Self::FailedRetries(x) => {
//                assert!(x < 16);
//                u16::from_be_bytes([0x63, 0xc0 + x])
//            }

//            Self::SecureMessagingNotSupported => 0x6882,
//            Self::CommandChainingNotSupported => 0x6884,

//            Self::SecurityStatusNotSatisfied => 0x6982,
//            Self::OperationBlocked => 0x6983,

//            Self::IncorrectDataParameter => 0x6a80,
//            Self::FunctionNotSupported => 0x6a81,
//            Self::NotFound => 0x6a82,
//            Self::NotEnoughMemory => 0x6a84,
//            Self::IncorrectP1OrP2Parameter => 0x6a86,
//            Self::KeyReferenceNotFound => 0x6a88,

//            Self::InstructionNotSupportedOrInvalid => 0x6d00,
//            Self::ClassNotSupported => 0x6e00,
//            Self::UnspecifiedCheckingError => 0x6f00,

//            Self::Success => 0x9000,
//            Self::MoreAvailable(x) => u16::from_be_bytes([0x61, x]),
//        }
//    }
//}

//impl Into<[u8; 2]> for StatusWord {
//    #[inline]
//    fn into(self) -> [u8; 2] {
//        let sw: u16 = self.into();
//        sw.to_be_bytes()
//    }
//}


// 6A, 80 incorrect parameter in command data field
// 6A, 81 function not supported
// 6A, 82 data object not found ( = NOT FOUND for files, e.g. certificate, e.g. after GET-DATA)
// 6A, 84 not enough memory
// 6A, 86 incorrect parameter in P1/P2
// 6A, 88 reference(d) data not found ( = NOT FOUND for keys, e.g. global PIN, e.g. after VERIFY)
//
// 90, 00 SUCCESS!
// == == == == == == == == == == ==

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DataObjects {}
#[allow(non_upper_case_globals)]
impl DataObjects {
    pub const DiscoveryObject: &'static [u8] = &[0x7e];
    pub const BiometricInformationTemplate: &'static [u8] = &[0x7f, 0x61];

    pub const X509CertificateForCardAuthentication: &'static [u8] = &[0x5f, 0xc1, 0x01];
    // CHUID, contains GUID
    pub const CardHolderUniqueIdentifier: &'static [u8] = &[0x5f, 0xc1, 0x02];
    pub const X509CertificateForPivAuthentication: &'static [u8] = &[0x5f, 0xc1, 0x05];
    pub const X509CertificateForDigitalSignature: &'static [u8] = &[0x5f, 0xc1, 0x0a];
    pub const X509CertificateForKeyManagement: &'static [u8] = &[0x5f, 0xc1, 0x0b];

    pub const KeyHistoryObject: &'static [u8] = &[0x5f, 0xc1, 0x0c];
}

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct YubicoObjects {}
#[allow(non_upper_case_globals)]
impl YubicoObjects {
    pub const AttestationCertificate: &'static [u8] = b"\x5f\xff\x01";
}

// https://developers.yubico.com/PIV/Introduction/Yubico_extensions.html
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum YubicoPivExtension {
    SetManagementKey = 0xff,
    ImportAsymmetricKey = 0xfe,
    GetVersion = 0xfd,
    Reset = 0xfb,
    SetPinRetries = 0xfa,
    Attest = 0xf9,
    GetSerial = 0xf8, // also used via 0x01
    GetMetadata = 0xf7,
}

impl core::convert::TryFrom<u8> for YubicoPivExtension {
    type Error = ();
    fn try_from(ins: u8) -> core::result::Result<Self, Self::Error> {
        Ok(match ins {
            // (0x00, 0x01, 0x10, 0x00)
            0x01 => YubicoPivExtension::GetSerial,
            0xff => YubicoPivExtension::SetManagementKey,
            0xfe => YubicoPivExtension::ImportAsymmetricKey,
            0xfd => YubicoPivExtension::GetVersion,
            0xfb => YubicoPivExtension::Reset,
            0xfa => YubicoPivExtension::SetPinRetries,
            // (0x00, 0xf9, 0x9a, 0x00)
            0xf9 => YubicoPivExtension::Attest,
            // (0x00, 0xf8, 0x00, 0x00)
            0xf8 => YubicoPivExtension::GetSerial,
            0xf7 => YubicoPivExtension::GetMetadata,
            _ => return Err(()),
        })
    }
}

pub const YUBICO_PIV_AUTHENTICATION_CERTIFICATE: &'static [u8; 351] = &[
    0x53, 0x82, 0x01, 0x5b, 0x70, 0x82, 0x01, 0x52, 0x30, 0x82, 0x01, 0x4e, 0x30, 0x81, 0xf5, 0xa0,
    0x03, 0x02, 0x01, 0x02, 0x02, 0x11, 0x00, 0x8e, 0x46, 0x32, 0xd8, 0xf0, 0xc1, 0xf7, 0xc1, 0x4d,
    0x67, 0xd1, 0x4b, 0xfd, 0xe3, 0x64, 0x8e, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x04, 0x03, 0x02, 0x30, 0x2a, 0x31, 0x16, 0x30, 0x14, 0x06, 0x03, 0x55, 0x04, 0x0a, 0x13, 0x0d,
    0x79, 0x75, 0x62, 0x69, 0x6b, 0x65, 0x79, 0x2d, 0x61, 0x67, 0x65, 0x6e, 0x74, 0x31, 0x10, 0x30,
    0x0e, 0x06, 0x03, 0x55, 0x04, 0x0b, 0x13, 0x07, 0x28, 0x64, 0x65, 0x76, 0x65, 0x6c, 0x29, 0x30,
    0x20, 0x17, 0x0d, 0x32, 0x30, 0x30, 0x35, 0x30, 0x39, 0x31, 0x32, 0x30, 0x30, 0x34, 0x39, 0x5a,
    0x18, 0x0f, 0x32, 0x30, 0x36, 0x32, 0x30, 0x35, 0x30, 0x39, 0x31, 0x33, 0x30, 0x30, 0x34, 0x39,
    0x5a, 0x30, 0x12, 0x31, 0x10, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x04, 0x03, 0x13, 0x07, 0x53, 0x53,
    0x48, 0x20, 0x6b, 0x65, 0x79, 0x30, 0x59, 0x30, 0x13, 0x06, 0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d,
    0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03, 0x01, 0x07, 0x03, 0x42, 0x00, 0x04,
    0x83, 0x2a, 0x92, 0x47, 0x8b, 0x4e, 0xb5, 0x7a, 0x46, 0x1b, 0x2a, 0x5e, 0x0e, 0x44, 0x25, 0x03,
    0x9b, 0xfb, 0x27, 0x94, 0x56, 0x78, 0xed, 0x48, 0x2b, 0x1c, 0xf2, 0x21, 0x61, 0x6d, 0xda, 0xbd,
    0x3d, 0x8f, 0xb6, 0x2b, 0x75, 0xc6, 0xac, 0x3f, 0x83, 0x4a, 0x59, 0x4e, 0x5a, 0xdf, 0xed, 0xe7,
    0x3a, 0xe4, 0x99, 0x1a, 0xe7, 0x33, 0x2f, 0x61, 0x2b, 0xcf, 0x6c, 0x0e, 0xd6, 0x78, 0x72, 0xeb,
    0xa3, 0x12, 0x30, 0x10, 0x30, 0x0e, 0x06, 0x03, 0x55, 0x1d, 0x0f, 0x01, 0x01, 0xff, 0x04, 0x04,
    0x03, 0x02, 0x03, 0x88, 0x30, 0x0a, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x04, 0x03, 0x02,
    0x03, 0x48, 0x00, 0x30, 0x45, 0x02, 0x20, 0x03, 0x09, 0xe2, 0x84, 0x47, 0xdc, 0xb7, 0xc5, 0x32,
    0xee, 0x97, 0x5b, 0x9e, 0x44, 0xfa, 0x42, 0x06, 0xf2, 0x26, 0x67, 0xc1, 0xa6, 0xc6, 0x4a, 0xdc,
    0x6a, 0x0b, 0x5d, 0xa9, 0x87, 0x63, 0x8b, 0x02, 0x21, 0x00, 0xbb, 0x4e, 0xcb, 0x18, 0x72, 0xcc,
    0x12, 0x39, 0xd3, 0xd4, 0x18, 0x36, 0x14, 0x18, 0xe4, 0xa9, 0xf3, 0x83, 0x81, 0x4b, 0x74, 0x0f,
    0x93, 0x33, 0xb8, 0x47, 0xa9, 0x73, 0xc2, 0x82, 0x92, 0x3e, 0x71, 0x01, 0x00, 0xfe, 0x00,
];

pub const YUBICO_ATTESTATION_CERTIFICATE: &'static [u8; 754] = &[
    0x53, 0x82, 0x02, 0xee, 0x70, 0x82, 0x02, 0xea, 0x30, 0x82, 0x02, 0xe6, 0x30, 0x82, 0x01, 0xce,
    0xa0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x09, 0x00, 0xa4, 0x85, 0x22, 0xaa, 0x34, 0xaf, 0xae, 0x4f,
    0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b, 0x05, 0x00, 0x30,
    0x2b, 0x31, 0x29, 0x30, 0x27, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x20, 0x59, 0x75, 0x62, 0x69,
    0x63, 0x6f, 0x20, 0x50, 0x49, 0x56, 0x20, 0x52, 0x6f, 0x6f, 0x74, 0x20, 0x43, 0x41, 0x20, 0x53,
    0x65, 0x72, 0x69, 0x61, 0x6c, 0x20, 0x32, 0x36, 0x33, 0x37, 0x35, 0x31, 0x30, 0x20, 0x17, 0x0d,
    0x31, 0x36, 0x30, 0x33, 0x31, 0x34, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x18, 0x0f, 0x32,
    0x30, 0x35, 0x32, 0x30, 0x34, 0x31, 0x37, 0x30, 0x30, 0x30, 0x30, 0x30, 0x30, 0x5a, 0x30, 0x21,
    0x31, 0x1f, 0x30, 0x1d, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x16, 0x59, 0x75, 0x62, 0x69, 0x63,
    0x6f, 0x20, 0x50, 0x49, 0x56, 0x20, 0x41, 0x74, 0x74, 0x65, 0x73, 0x74, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x30, 0x82, 0x01, 0x22, 0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01,
    0x01, 0x01, 0x05, 0x00, 0x03, 0x82, 0x01, 0x0f, 0x00, 0x30, 0x82, 0x01, 0x0a, 0x02, 0x82, 0x01,
    0x01, 0x00, 0xab, 0xa9, 0x0b, 0x16, 0x9b, 0xef, 0x31, 0xcc, 0x3e, 0xac, 0x18, 0x5a, 0x2d, 0x45,
    0x80, 0x75, 0x70, 0xc7, 0x58, 0xb0, 0x6c, 0x3f, 0x1b, 0x59, 0x0d, 0x49, 0xb9, 0x89, 0xe8, 0x6f,
    0xce, 0xbb, 0x27, 0x6f, 0xd8, 0x3c, 0x60, 0x3a, 0x85, 0x00, 0xef, 0x5c, 0xbc, 0x40, 0x99, 0x3d,
    0x41, 0xee, 0xea, 0xc0, 0x81, 0x7f, 0x76, 0x48, 0xe4, 0xa9, 0x4c, 0xbc, 0xd5, 0x6b, 0xe1, 0x1f,
    0x0a, 0x60, 0x93, 0xc6, 0xfe, 0xaa, 0xd2, 0x8d, 0x8e, 0xe2, 0xb7, 0xcd, 0x8b, 0x2b, 0xf7, 0x9b,
    0xdd, 0x5a, 0xab, 0x2f, 0xcf, 0xb9, 0x0e, 0x54, 0xce, 0xec, 0x8d, 0xf5, 0x5e, 0xd7, 0x7b, 0x91,
    0xc3, 0xa7, 0x56, 0x9c, 0xdc, 0xc1, 0x06, 0x86, 0x76, 0x36, 0x44, 0x53, 0xfb, 0x08, 0x25, 0xd8,
    0x06, 0xb9, 0x06, 0x8c, 0x81, 0xfd, 0x63, 0x67, 0xca, 0x3c, 0xa8, 0xb8, 0xea, 0x1c, 0xa6, 0xca,
    0xdb, 0x44, 0x7b, 0x12, 0xca, 0xb2, 0x34, 0x01, 0x7e, 0x73, 0xe4, 0x36, 0x83, 0xdf, 0xeb, 0xf9,
    0x23, 0x00, 0x07, 0x01, 0x6a, 0x07, 0x19, 0x8a, 0x64, 0x56, 0x9d, 0x10, 0x8a, 0xc5, 0x73, 0x02,
    0x3d, 0x18, 0x6e, 0xaf, 0x3f, 0xc3, 0x02, 0xa7, 0xc0, 0xf7, 0xa2, 0xfd, 0x6d, 0x5a, 0x42, 0x76,
    0x4e, 0xd6, 0xc0, 0x1e, 0xd6, 0xc0, 0xc6, 0xaa, 0x5d, 0xa7, 0x1a, 0x9f, 0x10, 0xdb, 0x30, 0x57,
    0x18, 0x5c, 0xb5, 0xb5, 0xfd, 0x0c, 0xbe, 0x49, 0x24, 0x22, 0xaf, 0x1e, 0x56, 0x4a, 0x34, 0x44,
    0xd4, 0xaa, 0xd4, 0xe1, 0xae, 0x95, 0x4c, 0x75, 0xc0, 0x88, 0x61, 0xf4, 0x8c, 0x7e, 0x54, 0xf3,
    0x13, 0xeb, 0x0f, 0xe5, 0x2b, 0x52, 0x60, 0x5a, 0x6e, 0xba, 0xd7, 0xe5, 0x8c, 0x63, 0xda, 0x51,
    0x1a, 0xbb, 0x22, 0x5c, 0x37, 0x2b, 0xd7, 0xd1, 0x70, 0x57, 0x4c, 0x2e, 0xdc, 0x35, 0x3c, 0x22,
    0x98, 0x9b, 0x02, 0x03, 0x01, 0x00, 0x01, 0xa3, 0x15, 0x30, 0x13, 0x30, 0x11, 0x06, 0x0a, 0x2b,
    0x06, 0x01, 0x04, 0x01, 0x82, 0xc4, 0x0a, 0x03, 0x03, 0x04, 0x03, 0x04, 0x03, 0x03, 0x30, 0x0d,
    0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b, 0x05, 0x00, 0x03, 0x82, 0x01,
    0x01, 0x00, 0x52, 0x80, 0x5a, 0x6d, 0xc3, 0x9e, 0xdf, 0x47, 0xa8, 0xf1, 0xb2, 0xa5, 0x9c, 0xa3,
    0x80, 0x81, 0x3b, 0x1d, 0x6a, 0xeb, 0x6a, 0x12, 0x62, 0x4b, 0x11, 0xfd, 0x8d, 0x30, 0xf1, 0x7b,
    0xfc, 0x71, 0x10, 0xc9, 0xb2, 0x08, 0xfc, 0xd1, 0x4e, 0x35, 0x7f, 0x45, 0xf2, 0x10, 0xa2, 0x52,
    0xb9, 0xd4, 0xb3, 0x02, 0x1a, 0x01, 0x56, 0x07, 0x6b, 0xfa, 0x64, 0xa7, 0x08, 0xf0, 0x03, 0xfb,
    0x27, 0xa9, 0x60, 0x8d, 0x0d, 0xd3, 0xac, 0x5a, 0x10, 0xcf, 0x20, 0x96, 0x4e, 0x82, 0xbc, 0x9d,
    0xe3, 0x37, 0xda, 0xc1, 0x4c, 0x50, 0xe1, 0x3d, 0x16, 0xb4, 0xca, 0xf4, 0x1b, 0xff, 0x08, 0x64,
    0xc9, 0x74, 0x4f, 0x2a, 0x3a, 0x43, 0xe0, 0xde, 0x42, 0x79, 0xf2, 0x13, 0xae, 0x77, 0xa1, 0xe2,
    0xae, 0x6b, 0xdf, 0x72, 0xa5, 0xb6, 0xce, 0xd7, 0x4c, 0x90, 0x13, 0xdf, 0xde, 0xdb, 0xf2, 0x8b,
    0x34, 0x45, 0x8b, 0x30, 0xdc, 0x51, 0xab, 0xa9, 0x34, 0xf8, 0xa9, 0xe5, 0x0c, 0x47, 0x29, 0xaa,
    0x2f, 0x42, 0x54, 0xf2, 0xf8, 0x19, 0x5a, 0xb4, 0x89, 0xfe, 0x1b, 0x9f, 0x19, 0x7a, 0x16, 0xc8,
    0xc8, 0xba, 0x8f, 0x18, 0x17, 0x7a, 0x07, 0xa9, 0x97, 0xa1, 0x56, 0xb9, 0x52, 0x5d, 0xa1, 0x21,
    0xc0, 0x81, 0x67, 0x2d, 0xe8, 0x0e, 0xa6, 0x51, 0xb9, 0x08, 0xb0, 0x9d, 0xd3, 0x60, 0x1c, 0x70,
    0xa3, 0x0f, 0xfa, 0xd8, 0x62, 0xd8, 0x79, 0x2b, 0x0a, 0xe6, 0x42, 0xfc, 0xf8, 0x2d, 0xf5, 0xe4,
    0xcd, 0xfb, 0x15, 0x96, 0x23, 0xff, 0xb6, 0xc0, 0xa7, 0xa7, 0xe2, 0x85, 0x83, 0xf9, 0x70, 0xc8,
    0x19, 0x6b, 0xf3, 0xc1, 0x3f, 0x37, 0x44, 0x65, 0x27, 0xfb, 0x67, 0x88, 0xc8, 0x83, 0xb7, 0x2f,
    0x85, 0x1f, 0x80, 0x44, 0xbb, 0x72, 0xce, 0x06, 0x82, 0x59, 0x2d, 0x83, 0x00, 0xe1, 0x94, 0x8d,
    0xa0, 0x85,
];

pub const YUBICO_ATTESTATION_CERTIFICATE_FOR_9A: &'static [u8; 584] = &[
    0x30, 0x82, 0x02, 0x44, 0x30, 0x82, 0x01, 0x2c, 0xa0, 0x03, 0x02, 0x01, 0x02, 0x02, 0x11, 0x00,
    0xc6, 0x36, 0xe7, 0xb3, 0xa5, 0xa5, 0xa4, 0x98, 0x5d, 0x13, 0x6e, 0x43, 0x36, 0x2d, 0x13, 0xf7,
    0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x0b, 0x05, 0x00, 0x30,
    0x21, 0x31, 0x1f, 0x30, 0x1d, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x16, 0x59, 0x75, 0x62, 0x69,
    0x63, 0x6f, 0x20, 0x50, 0x49, 0x56, 0x20, 0x41, 0x74, 0x74, 0x65, 0x73, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x30, 0x20, 0x17, 0x0d, 0x31, 0x36, 0x30, 0x33, 0x31, 0x34, 0x30, 0x30, 0x30, 0x30,
    0x30, 0x30, 0x5a, 0x18, 0x0f, 0x32, 0x30, 0x35, 0x32, 0x30, 0x34, 0x31, 0x37, 0x30, 0x30, 0x30,
    0x30, 0x30, 0x30, 0x5a, 0x30, 0x25, 0x31, 0x23, 0x30, 0x21, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c,
    0x1a, 0x59, 0x75, 0x62, 0x69, 0x4b, 0x65, 0x79, 0x20, 0x50, 0x49, 0x56, 0x20, 0x41, 0x74, 0x74,
    0x65, 0x73, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x39, 0x61, 0x30, 0x59, 0x30, 0x13, 0x06,
    0x07, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x02, 0x01, 0x06, 0x08, 0x2a, 0x86, 0x48, 0xce, 0x3d, 0x03,
    0x01, 0x07, 0x03, 0x42, 0x00, 0x04, 0x83, 0x2a, 0x92, 0x47, 0x8b, 0x4e, 0xb5, 0x7a, 0x46, 0x1b,
    0x2a, 0x5e, 0x0e, 0x44, 0x25, 0x03, 0x9b, 0xfb, 0x27, 0x94, 0x56, 0x78, 0xed, 0x48, 0x2b, 0x1c,
    0xf2, 0x21, 0x61, 0x6d, 0xda, 0xbd, 0x3d, 0x8f, 0xb6, 0x2b, 0x75, 0xc6, 0xac, 0x3f, 0x83, 0x4a,
    0x59, 0x4e, 0x5a, 0xdf, 0xed, 0xe7, 0x3a, 0xe4, 0x99, 0x1a, 0xe7, 0x33, 0x2f, 0x61, 0x2b, 0xcf,
    0x6c, 0x0e, 0xd6, 0x78, 0x72, 0xeb, 0xa3, 0x3c, 0x30, 0x3a, 0x30, 0x11, 0x06, 0x0a, 0x2b, 0x06,
    0x01, 0x04, 0x01, 0x82, 0xc4, 0x0a, 0x03, 0x03, 0x04, 0x03, 0x04, 0x03, 0x04, 0x30, 0x13, 0x06,
    0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0xc4, 0x0a, 0x03, 0x07, 0x04, 0x05, 0x02, 0x03, 0x52,
    0xf7, 0x43, 0x30, 0x10, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0xc4, 0x0a, 0x03, 0x08,
    0x04, 0x02, 0x02, 0x02, 0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01,
    0x0b, 0x05, 0x00, 0x03, 0x82, 0x01, 0x01, 0x00, 0x02, 0x17, 0x38, 0xa8, 0xf6, 0x1d, 0x17, 0x35,
    0xe1, 0x30, 0x9d, 0xd2, 0xd5, 0xc4, 0xd4, 0xd0, 0x0d, 0xe1, 0x9f, 0x37, 0x9a, 0xbe, 0xcf, 0x63,
    0x6a, 0x0e, 0x2b, 0xd0, 0xd7, 0xa4, 0x04, 0x5c, 0x40, 0x7d, 0xf7, 0x43, 0x9b, 0xe4, 0xee, 0x7d,
    0x96, 0x55, 0xd2, 0x91, 0xdc, 0x32, 0x82, 0x54, 0xfe, 0x2d, 0x9f, 0x19, 0x23, 0x54, 0xbb, 0xdd,
    0x7d, 0x6b, 0xe9, 0x61, 0x2a, 0x1d, 0xc8, 0x13, 0x65, 0xe2, 0x04, 0x9f, 0xa2, 0x87, 0xde, 0x61,
    0x92, 0xd5, 0xde, 0x46, 0xd4, 0xa4, 0xc2, 0xa6, 0xb4, 0x80, 0x5d, 0x4a, 0xa4, 0xd1, 0x1b, 0xa7,
    0x34, 0xf2, 0x97, 0x7b, 0x7a, 0x5a, 0xad, 0x9a, 0xa8, 0x5d, 0x2a, 0xd4, 0x7f, 0xb1, 0x57, 0xbf,
    0x26, 0x1d, 0x3d, 0xa6, 0xb3, 0xea, 0x3d, 0x3d, 0xf7, 0x94, 0xcd, 0x16, 0x36, 0x40, 0x24, 0xcd,
    0x7c, 0x8e, 0x7a, 0xdb, 0x2d, 0xf9, 0x22, 0xda, 0x26, 0xb3, 0xc1, 0xc8, 0x00, 0xa3, 0x47, 0x97,
    0x52, 0x10, 0x12, 0x73, 0x4b, 0xaf, 0x12, 0xfe, 0xb7, 0x0d, 0x9e, 0x91, 0x30, 0xa7, 0x52, 0xcf,
    0x12, 0xd8, 0x2b, 0xdf, 0x12, 0x6a, 0xb6, 0x2f, 0x39, 0x24, 0xc6, 0x04, 0xa2, 0x6f, 0xed, 0x70,
    0xb5, 0xf2, 0x0d, 0x2a, 0x73, 0xe3, 0x38, 0xa9, 0x9c, 0xfe, 0x35, 0x3e, 0xdc, 0x17, 0x40, 0x55,
    0xd5, 0x95, 0x7f, 0x05, 0x8e, 0x24, 0xc2, 0xb3, 0xb1, 0x05, 0x2d, 0x69, 0x0c, 0xcf, 0x5b, 0xf7,
    0x06, 0x40, 0x17, 0x36, 0x0a, 0xc3, 0xa5, 0xdb, 0x3c, 0xda, 0x62, 0xf8, 0x53, 0x2d, 0xf1, 0x3f,
    0x04, 0x55, 0x70, 0x0c, 0x43, 0x7b, 0x1f, 0xa3, 0x63, 0xb1, 0xa0, 0x5e, 0x89, 0x28, 0x5b, 0x4f,
    0x76, 0xa7, 0x05, 0xe1, 0x4c, 0x45, 0x55, 0x14, 0xff, 0x10, 0x10, 0x89, 0x69, 0x6a, 0x13, 0x3d,
    0x89, 0xf2, 0xca, 0xfd, 0x14, 0x9a, 0xc4, 0xd0,
];
// pub const YUBICO_DEFAULT_MANAGEMENT_KEY: &'static [u8; 24] = b"123456781234567812345678";
pub const YUBICO_DEFAULT_MANAGEMENT_KEY: &'static [u8; 24] = &[
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
];

// stolen from le yubico
pub const DISCOVERY_OBJECT: &'static [u8; 20] = b"~\x12O\x0b\xa0\x00\x00\x03\x08\x00\x00\x10\x00\x01\x00_/\x02@\x00";

// import secrets; secrets.token_bytes(16)
pub const GUID: &'static [u8; 16] = b"\x0c\x92\xc9\x04\xd0\xdeL\xd9\xf6\xd1\xa2\x9fE3\xca\xeb";
