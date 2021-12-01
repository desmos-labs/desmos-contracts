/// ChainLink contains the data representing either an inter- or cross- chain
/// link
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainLink {
    /// User defines the destination profile address to link
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    /// Address contains the data of the external chain address to be connected
    /// with the Desmos profile
    #[prost(message, optional, tag="2")]
    pub address: ::core::option::Option<::prost_types::Any>,
    /// Proof contains the ownership proof of the external chain address
    #[prost(message, optional, tag="3")]
    pub proof: ::core::option::Option<Proof>,
    /// ChainConfig contains the configuration of the external chain
    #[prost(message, optional, tag="4")]
    pub chain_config: ::core::option::Option<ChainConfig>,
    /// CreationTime represents the time in which the link has been created
    #[prost(message, optional, tag="5")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// ChainConfig contains the data of the chain with which the link is made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChainConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Proof contains all the data used to verify a signature when linking an
/// account to a profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proof {
    /// PubKey represents the public key associated with the address for which to
    /// prove the ownership
    #[prost(message, optional, tag="1")]
    pub pub_key: ::core::option::Option<::prost_types::Any>,
    /// Signature represents the hex-encoded signature of the PlainText value
    #[prost(string, tag="2")]
    pub signature: ::prost::alloc::string::String,
    /// PlainText represents the hex-encoded value signed in order to produce the Signature
    #[prost(string, tag="3")]
    pub plain_text: ::prost::alloc::string::String,
}
/// Bech32Address represents a Bech32-encoded address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bech32Address {
    /// Value represents the Bech-32 encoded address value
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
    /// Prefix represents the HRP of the Bech32 address
    #[prost(string, tag="2")]
    pub prefix: ::prost::alloc::string::String,
}
/// Base58Address represents a Base58-encoded address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base58Address {
    /// Value contains the Base58-encoded address
    #[prost(string, tag="1")]
    pub value: ::prost::alloc::string::String,
}
// ___________________________________________________________________________________________________________________

/// Profile represents a generic first on Desmos, containing the information of a
/// single user
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// Account represents the base Cosmos account associated with this profile
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<::prost_types::Any>,
    /// DTag represents the unique tag of this profile
    #[prost(string, tag="2")]
    pub dtag: ::prost::alloc::string::String,
    /// Nickname contains the custom human readable name of the profile
    #[prost(string, tag="3")]
    pub nickname: ::prost::alloc::string::String,
    /// Bio contains the biography of the profile
    #[prost(string, tag="4")]
    pub bio: ::prost::alloc::string::String,
    /// Pictures contains the data about the pictures associated with he profile
    #[prost(message, optional, tag="5")]
    pub pictures: ::core::option::Option<Pictures>,
    /// CreationTime represents the time in which the profile has been created
    #[prost(message, optional, tag="6")]
    pub creation_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Pictures contains the data of a user profile's related pictures
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pictures {
    /// Profile contains the URL to the profile picture
    #[prost(string, tag="1")]
    pub profile: ::prost::alloc::string::String,
    /// Cover contains the URL to the cover picture
    #[prost(string, tag="2")]
    pub cover: ::prost::alloc::string::String,
}
/// ApplicationLink contains the data of a link to a centralized application
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationLink {
    ///  User to which the link is associated
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    /// Data contains the details of this specific link
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<Data>,
    /// State of the link
    #[prost(enumeration="ApplicationLinkState", tag="3")]
    pub state: i32,
    /// OracleRequest represents the request that has been made to the oracle
    #[prost(message, optional, tag="4")]
    pub oracle_request: ::core::option::Option<OracleRequest>,
    /// Data coming from the result of the verification.
    /// Only available when the state is STATE_SUCCESS
    #[prost(message, optional, tag="5")]
    pub result: ::core::option::Option<Result>,
    /// CreationTime represents the time in which the link was created
    #[prost(message, optional, tag="6")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Data contains the data associated to a specific user of a
/// generic centralized application
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// The application name (eg. Twitter, GitHub, etc)
    #[prost(string, tag="1")]
    pub application: ::prost::alloc::string::String,
    /// Username on the application (eg. Twitter tag, GitHub profile, etc)
    #[prost(string, tag="2")]
    pub username: ::prost::alloc::string::String,
}
/// OracleRequest represents a generic oracle request used to
/// verify the ownership of a centralized application account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleRequest {
    /// ID is the ID of the request
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// OracleScriptID is ID of an oracle script
    #[prost(uint64, tag="2")]
    pub oracle_script_id: u64,
    /// CallData contains the data used to perform the oracle request
    #[prost(message, optional, tag="3")]
    pub call_data: ::core::option::Option<oracle_request::CallData>,
    /// ClientID represents the ID of the client that has called the oracle script
    #[prost(string, tag="4")]
    pub client_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OracleRequest`.
pub mod oracle_request {
    /// CallData contains the data sent to a single oracle request in order to
    /// verify the ownership of a centralized application by a Desmos profile
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallData {
        /// The application for which the ownership should be verified
        #[prost(string, tag="1")]
        pub application: ::prost::alloc::string::String,
        /// The hex encoded call data that should be used to verify the application
        /// account ownership
        #[prost(string, tag="2")]
        pub call_data: ::prost::alloc::string::String,
    }
}
/// Result represents a verification result
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Result {
    /// sum is the oneof that specifies whether this represents a success or
    /// failure result
    #[prost(oneof="result::Sum", tags="1, 2")]
    pub sum: ::core::option::Option<result::Sum>,
}
/// Nested message and enum types in `Result`.
pub mod result {
    /// Success is the result of an application link that has been successfully
    /// verified
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Success {
        /// Hex-encoded value that has be signed by the profile
        #[prost(string, tag="1")]
        pub value: ::prost::alloc::string::String,
        /// Hex-encoded signature that has been produced by signing the value
        #[prost(string, tag="2")]
        pub signature: ::prost::alloc::string::String,
    }
    /// Failed is the result of an application link that has not been verified
    /// successfully
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Failed {
        /// Error that is associated with the failure
        #[prost(string, tag="1")]
        pub error: ::prost::alloc::string::String,
    }
    /// sum is the oneof that specifies whether this represents a success or
    /// failure result
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Sum {
        /// Success represents a successful verification
        #[prost(message, tag="1")]
        Success(Success),
        /// Failed represents a failed verification
        #[prost(message, tag="2")]
        Failed(Failed),
    }
}
/// ApplicationLinkState defines if an application link is in the following
/// states: STARTED, ERRORED, SUCCESSFUL, TIMED_OUT
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ApplicationLinkState {
    /// A link has just been initialized
    InitializedUnspecified = 0,
    /// A link has just started being verified
    VerificationStarted = 1,
    /// A link has errored during the verification process
    VerificationError = 2,
    /// A link has being verified successfully
    VerificationSuccess = 3,
    /// A link has timed out while waiting for the verification
    TimedOut = 4,
}
/// DTagTransferRequest represent a DTag transfer request between two users
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DTagTransferRequest {
    /// DTagToTrade contains the value of the DTag that should be transferred from
    /// the receiver of the request to the sender
    #[prost(string, tag="1")]
    pub dtag_to_trade: ::prost::alloc::string::String,
    /// Sender represents the address of the account that sent the request
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    /// Receiver represents the receiver of the request that, if accepted, will
    /// give to the sender their DTag
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
}
/// Relationship is the struct of a relationship.
/// It represent the concept of "follow" of traditional social networks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relationship {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subspace: ::prost::alloc::string::String,
}
/// UserBlock represents the fact that the Blocker has blocked the given Blocked
/// user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserBlock {
    /// Blocker represents the address of the user blocking another one
    #[prost(string, tag="1")]
    pub blocker: ::prost::alloc::string::String,
    /// Blocked represents the address of the blocked user
    #[prost(string, tag="2")]
    pub blocked: ::prost::alloc::string::String,
    /// Reason represents the optional reason the user has been blocked for.
    #[prost(string, tag="3")]
    pub reason: ::prost::alloc::string::String,
    /// Subspace contains the ID of the subspace inside which the user should be
    /// blocked
    #[prost(string, tag="4")]
    pub subspace: ::prost::alloc::string::String,
}
/// QueryProfileRequest is the request type for the Query/Profile RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProfileRequest {
    /// Address or DTag of the user to query the profile for
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
}
/// QueryProfileResponse is the response type for the Query/Profile RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProfileResponse {
    #[prost(message, optional, tag="1")]
    pub profile: ::core::option::Option<::prost_types::Any>,
}
