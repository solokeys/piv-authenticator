use core::convert::{TryFrom, TryInto};

use trussed::{
    block,
    Client as TrussedClient,
    syscall, try_syscall,
    types::{KeyId, PathBuf, Location},
};

use crate::constants::*;

use crate::{Pin, Puk};

pub type Result<T> = core::result::Result<T, ()>;

pub enum Key {
    Ed255(KeyId),
    P256(KeyId),
    X255(KeyId),
}
pub enum PinPolicy {
    Never,
    Once,
    Always,
}
pub enum TouchPolicy {
    Never,
    Always,
    Cached,
}

pub struct Slot {
    pub key: Option<KeyId>,
    pub pin_policy: PinPolicy,
    // touch_policy: TouchPolicy,
}

impl Default for Slot {
    fn default() -> Self {
        Self { key: None, pin_policy: PinPolicy::Once, /*touch_policy: TouchPolicy::Never*/ }
    }
}

impl Slot {
    pub fn default(name: SlotName) -> Self {
        use SlotName::*;
        match name {
            // Management => Slot { pin_policy: PinPolicy::Never, ..Default::default() },
            Signature => Slot { pin_policy: PinPolicy::Always, ..Default::default() },
            Pinless => Slot { pin_policy: PinPolicy::Never, ..Default::default() },
            _ => Default::default(),

        }
    }
}

pub struct RetiredSlotIndex(u8);

impl core::convert::TryFrom<u8> for RetiredSlotIndex {
    type Error = u8;
    fn try_from(i: u8) -> core::result::Result<Self, Self::Error> {
        if 1 <= i && i <= 20 {
            Ok(Self(i))
        } else {
            Err(i)
        }
    }
}
pub enum SlotName {
    Identity,
    Management,  // Personalization? Administration?
    Signature,
    Decryption,  // Management after all?
    Pinless,
    Retired(RetiredSlotIndex),
    Attestation,
}

impl SlotName {
    pub fn default_pin_policy(&self) -> PinPolicy {
        use SlotName::*;
        use PinPolicy::*;
        match *self {
            Signature => Always,
            Pinless | Management | Attestation => Never,
            _ => Once,
        }
    }

    pub fn default_slot(&self) -> Slot {
        Slot { key: None, pin_policy: self.default_pin_policy() }
    }

    pub fn reference(&self) -> u8 {
        use SlotName::*;
        match *self {
            Identity => 0x9a,
            Management => 0x9b,
            Signature => 0x9c,
            Decryption => 0x9d,
            Pinless => 0x9e,
            Retired(RetiredSlotIndex(i)) => 0x81 + i,
            Attestation => 0xf9,
        }
    }
    pub fn tag(&self) -> u32 {
        use SlotName::*;
        match *self {
            Identity => 0x5fc105,
            Management => 0,
            Signature => 0x5fc10a,
            Decryption => 0x5fc10b,
            Pinless => 0x5fc101,
            Retired(RetiredSlotIndex(i)) => 0x5fc10c + i as u32,
            Attestation => 0x5fff01,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Keys {
    // 9a "PIV Authentication Key" (YK: PIV Authentication)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_key: Option<KeyId>,
    // 9b "PIV Card Application Administration Key" (YK: PIV Management)
    pub management_key: KeyId,
    // 9c "Digital Signature Key" (YK: Digital Signature)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_key: Option<KeyId>,
    // 9d "Key Management Key" (YK: Key Management)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<KeyId>,
    // 9e "Card Authentication Key" (YK: Card Authentication)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinless_authentication_key: Option<KeyId>,
    // 0x82..=0x95 (130-149)
    pub retired_keys: [Option<KeyId>; 20],
}


#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct State<const C: usize> {
    pub runtime: Runtime<C>,
    // temporary "state", to be removed again
    // pub hack: Hack,
    // trussed: RefCell<Trussed<S>>,
}

impl<const C: usize> State<C> {
    pub fn new() -> Self {
        Default::default()
    }

    // it would be nicer to do this during "board bringup", by using TrussedService as Syscall
    //
    // TODO: it is really not good to overwrite user data on failure to decode old state.
    // To fix this, need a flag to detect if we're "fresh", and/or initialize state in factory.
    pub fn persistent<'t, T>(&mut self, trussed: &'t mut T) -> Persistent<'t, T>
    where T: TrussedClient
        + trussed::client::Tdes
    {
        Persistent::load_or_initialize(trussed)
    }
}

// #[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
// pub struct Pin {
//     // padded_pin: [u8; 8]
//     pin: heapless_bytes::Bytes<heapless::consts::U8>,
// }

// impl Default for Pin {
//     /// Default is "202020"
//     /// But right now we have to use "123456" cause.. Filo
//     fn default() -> Self {
//         // Self::try_new(b"202020\xff\xff").unwrap()
//         Self::try_new(b"123456\xff\xff").unwrap()
//     }
// }

// impl Pin {
//     pub fn try_new(padded_pin: &[u8]) -> Result<Self> {
//         if padded_pin.len() != 8 {
//             return Err(());
//         }
//         let first_pad_byte = padded_pin.iter().position(|&b| b == 0xff);
//         let unpadded_pin = match first_pad_byte {
//             Some(l) => &padded_pin[..l],
//             None => padded_pin,
//         };
//         if unpadded_pin.len() < 6 {
//             return Err(());
//         }
//         let valid_bytes = unpadded_pin.iter().all(|&b| b >= b'0' && b <= b'9');
//         if valid_bytes {
//             Ok(Self {
//                 // padded_pin: padded_pin.try_into().unwrap(),
//                 pin: Bytes::from_slice(padded_pin).unwrap(),//padded_pin.try_into().unwrap(),
//             })
//         } else {
//             Err(())
//         }
//     }
// }

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PersistentState {
    pub keys: Keys,
    consecutive_pin_mismatches: u8,
    consecutive_puk_mismatches: u8,
    // the PIN can be 6-8 digits, padded with 0xFF if <8
    // we just store all of them for now.
    pin: Pin,
    // the PUK should be 8 digits, but it seems Yubico allows 6-8
    // like for PIN
    puk: Puk,
    // pin_hash: Option<[u8; 16]>,
    // Ideally, we'd dogfood a "Monotonic Counter" from `trussed`.
    timestamp: u32,
    // must be a valid RFC 4122 UUID 1, 2 or 4
    guid: [u8; 16],
}

#[derive(Debug, Eq, PartialEq)]
pub struct Persistent<'t, Trussed> {
    trussed: &'t mut Trussed,
    pub(crate) state: PersistentState,
}

impl<T> AsRef<PersistentState> for Persistent<'_, T> {
    fn as_ref(&self) -> &PersistentState {
        &self.state
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Runtime<const C: usize> {
    // aid: Option<
    // consecutive_pin_mismatches: u8,

    pub global_security_status: GlobalSecurityStatus,
    // pub currently_selected_application: SelectableAid,
    pub app_security_status: AppSecurityStatus,
    pub command_cache: Option<CommandCache>,
    pub chained_command: Option<iso7816::Command<C>>,
}

// pub trait Aid {
//     const AID: &'static [u8];
//     const RIGHT_TRUNCATED_LENGTH: usize;

//     fn len() -> usize {
//         Self::AID.len()
//     }

//     fn full() -> &'static [u8] {
//         Self::AID
//     }

//     fn right_truncated() -> &'static [u8] {
//         &Self::AID[..Self::RIGHT_TRUNCATED_LENGTH]
//     }

//     fn pix() -> &'static [u8] {
//         &Self::AID[5..]
//     }

//     fn rid() -> &'static [u8] {
//         &Self::AID[..5]
//     }
// }

// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
// pub enum SelectableAid {
//     Piv(PivAid),
//     YubicoOtp(YubicoOtpAid),
// }

// impl Default for SelectableAid {
//     fn default() -> Self {
//         Self::Piv(Default::default())
//     }
// }

// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
// pub struct PivAid {}

// impl Aid for PivAid {
//     const AID: &'static [u8] = &PIV_AID;
//     const RIGHT_TRUNCATED_LENGTH: usize = 9;
// }

// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
// pub struct YubicoOtpAid {}

// impl Aid for YubicoOtpAid {
//     const AID: &'static [u8] = &YUBICO_OTP_AID;
//     const RIGHT_TRUNCATED_LENGTH: usize = 8;
// }

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GlobalSecurityStatus {
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SecurityStatus {
    JustVerified,
    Verified,
    NotVerified,
}

impl Default for SecurityStatus {
    fn default() -> Self {
        Self::NotVerified
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AppSecurityStatus {
    pub pin_verified: bool,
    pub puk_verified: bool,
    pub management_verified: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CommandCache {
    GetData(GetData),
    AuthenticateManagement(AuthenticateManagement),
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetData {
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthenticateManagement {
    pub challenge: [u8; 8],
}

impl<'t, T> Persistent<'t, T>
where
    T: TrussedClient + trussed::client::Tdes,
{

    pub const PIN_RETRIES_DEFAULT: u8 = 3;
    // hmm...!
    pub const PUK_RETRIES_DEFAULT: u8 = 5;
    const FILENAME: &'static [u8] = b"persistent-state.cbor";
    const DEFAULT_PIN: &'static [u8] = b"123456\xff\xff";
    const DEFAULT_PUK: &'static [u8] = b"12345678";

    pub fn guid(&self) -> [u8; 16] {
        self.state.guid
    }

    pub fn remaining_pin_retries(&self) -> u8 {
        if self.state.consecutive_pin_mismatches >= Self::PIN_RETRIES_DEFAULT {
            0
        } else {
            Self::PIN_RETRIES_DEFAULT - self.state.consecutive_pin_mismatches
        }
    }

    pub fn remaining_puk_retries(&self) -> u8 {
        if self.state.consecutive_puk_mismatches >= Self::PUK_RETRIES_DEFAULT {
            0
        } else {
            Self::PUK_RETRIES_DEFAULT - self.state.consecutive_puk_mismatches
        }
    }

    pub fn verify_pin(&self, other_pin: &Pin) -> bool {
        // hprintln!("verifying pin {:?} against {:?}", other_pin, &self.pin).ok();
        self.state.pin == *other_pin
    }

    pub fn verify_puk(&self, other_puk: &Puk) -> bool {
        // hprintln!("verifying puk {:?} against {:?}", other_puk, &self.puk).ok();
        self.state.puk == *other_puk
    }

    pub fn set_pin(&mut self, new_pin: Pin) {
        self.state.pin = new_pin;
        self.save();
    }

    pub fn set_puk(&mut self, new_puk: Puk) {
        self.state.puk = new_puk;
        self.save();
    }

    pub fn reset_pin(&mut self) {
        self.set_pin(Pin::try_from(Self::DEFAULT_PIN).unwrap());
        self.reset_consecutive_pin_mismatches();
    }

    pub fn reset_puk(&mut self) {
        self.set_puk(Puk::try_from(Self::DEFAULT_PUK).unwrap());
        self.reset_consecutive_puk_mismatches();
    }

    pub fn increment_consecutive_pin_mismatches(&mut self) -> u8 {
        if self.state.consecutive_pin_mismatches >= Self::PIN_RETRIES_DEFAULT {
            return 0;
        }

        self.state.consecutive_pin_mismatches += 1;
        self.save();
        Self::PIN_RETRIES_DEFAULT - self.state.consecutive_pin_mismatches
    }

    pub fn increment_consecutive_puk_mismatches(&mut self) -> u8 {
        if self.state.consecutive_puk_mismatches >= Self::PUK_RETRIES_DEFAULT {
            return 0;
        }

        self.state.consecutive_puk_mismatches += 1;
        self.save();
        Self::PUK_RETRIES_DEFAULT - self.state.consecutive_puk_mismatches
    }

    pub fn reset_consecutive_pin_mismatches(&mut self) -> u8 {
        if self.state.consecutive_pin_mismatches != 0 {
            self.state.consecutive_pin_mismatches = 0;
            self.save();
        }

        Self::PIN_RETRIES_DEFAULT
    }

    pub fn reset_consecutive_puk_mismatches(&mut self) -> u8 {
        if self.state.consecutive_puk_mismatches != 0 {
            self.state.consecutive_puk_mismatches = 0;
            self.save();
        }

        Self::PUK_RETRIES_DEFAULT
    }

    pub fn reset_management_key(&mut self) {
        self.set_management_key(YUBICO_DEFAULT_MANAGEMENT_KEY);
    }

    pub fn set_management_key(&mut self, management_key: &[u8; 24]) {
        // let new_management_key = syscall!(self.trussed.unsafe_inject_tdes_key(
        let new_management_key = syscall!(self.trussed.unsafe_inject_shared_key(
            management_key,
            trussed::types::Location::Internal,
        )).key;
        let old_management_key = self.state.keys.management_key;
        self.state.keys.management_key = new_management_key;
        self.save();
        syscall!(self.trussed.delete(old_management_key));
    }

    pub fn initialize(trussed: &'t mut T) -> Self {
        info_now!("initializing PIV state");
        let management_key = syscall!(trussed.unsafe_inject_shared_key(
            YUBICO_DEFAULT_MANAGEMENT_KEY,
            trussed::types::Location::Internal,
        )).key;

        let mut guid: [u8; 16] = syscall!(trussed.random_bytes(16))
            .bytes
            .as_ref()
            .try_into()
            .unwrap();

        guid[6] = (guid[6] & 0xf) | 0x40;
        guid[8] = (guid[8] & 0x3f) | 0x80;

        let keys = Keys {
            authentication_key: None,
            management_key: management_key,
            signature_key: None,
            encryption_key: None,
            pinless_authentication_key: None,
            retired_keys: Default::default(),
        };

        let mut state = Self {
            trussed,
            state: PersistentState {
                keys,
                consecutive_pin_mismatches: 0,
                consecutive_puk_mismatches: 0,
                pin: Pin::try_from(Self::DEFAULT_PIN).unwrap(),
                puk: Puk::try_from(Self::DEFAULT_PUK).unwrap(),
                timestamp: 0,
                guid,
            }
        };
        state.save();
        state
    }

    pub fn load(trussed: &'t mut T) -> Result<Self> {
        let data = block!(trussed.read_file(
                Location::Internal,
                PathBuf::from(Self::FILENAME),
            ).unwrap()
        ).map_err(|e| {
            info!("loading error: {:?}", &e);
            drop(e)
        })?.data;

        let previous_state: PersistentState = trussed::cbor_deserialize(&data).map_err(|e| {
            info!("cbor deser error: {:?}", e);
            info!("data: {:X?}", &data);
            drop(e)
        })?;
        // horrible deser bug to forget Ok here :)
        Ok(Self { trussed, state: previous_state })
    }

    pub fn load_or_initialize(trussed: &'t mut T) -> Self {
        // todo: can't seem to combine load + initialize without code repetition
        let data = try_syscall!(trussed.read_file(Location::Internal, PathBuf::from(Self::FILENAME)));
        if let Ok(data) = data {
            let previous_state = trussed::cbor_deserialize(&data.data).map_err(|e| {
                info!("cbor deser error: {:?}", e);
                info!("data: {:X?}", &data);
                drop(e)
            });
            if let Ok(state) = previous_state {
            // horrible deser bug to forget Ok here :)
                return Self { trussed, state }
            }
        }

        Self::initialize(trussed)
    }

    pub fn save(&mut self) {
        let data: trussed::types::Message = trussed::cbor_serialize_bytes(self.as_ref()).unwrap();

        syscall!(self.trussed.write_file(
            Location::Internal,
            PathBuf::from(Self::FILENAME),
            data,
            None,
        ));
    }

    pub fn timestamp(&mut self) -> u32 {
        self.state.timestamp += 1;
        self.save();
        self.state.timestamp
    }

}

