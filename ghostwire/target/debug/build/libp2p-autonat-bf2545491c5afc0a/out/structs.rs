#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(enumeration="message::MessageType", optional, tag="1")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(message, optional, tag="2")]
    pub dial: ::core::option::Option<message::Dial>,
    #[prost(message, optional, tag="3")]
    pub dial_response: ::core::option::Option<message::DialResponse>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PeerInfo {
        #[prost(bytes="vec", optional, tag="1")]
        pub id: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
        #[prost(bytes="vec", repeated, tag="2")]
        pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Dial {
        #[prost(message, optional, tag="1")]
        pub peer: ::core::option::Option<PeerInfo>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DialResponse {
        #[prost(enumeration="ResponseStatus", optional, tag="1")]
        pub status: ::core::option::Option<i32>,
        #[prost(string, optional, tag="2")]
        pub status_text: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(bytes="vec", optional, tag="3")]
        pub addr: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        Dial = 0,
        DialResponse = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseStatus {
        Ok = 0,
        EDialError = 100,
        EDialRefused = 101,
        EBadRequest = 200,
        EInternalError = 300,
    }
}
