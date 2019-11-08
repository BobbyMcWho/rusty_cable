// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait RPC {
    fn connect(&self, o: ::grpc::RequestOptions, p: super::anycable::ConnectionRequest) -> ::grpc::SingleResponse<super::anycable::ConnectionResponse>;

    fn command(&self, o: ::grpc::RequestOptions, p: super::anycable::CommandMessage) -> ::grpc::SingleResponse<super::anycable::CommandResponse>;

    fn disconnect(&self, o: ::grpc::RequestOptions, p: super::anycable::DisconnectRequest) -> ::grpc::SingleResponse<super::anycable::DisconnectResponse>;
}

// client

pub struct RPCClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_Connect: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::anycable::ConnectionRequest, super::anycable::ConnectionResponse>>,
    method_Command: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::anycable::CommandMessage, super::anycable::CommandResponse>>,
    method_Disconnect: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::anycable::DisconnectRequest, super::anycable::DisconnectResponse>>,
}

impl ::grpc::ClientStub for RPCClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        RPCClient {
            grpc_client: grpc_client,
            method_Connect: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/anycable.RPC/Connect".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Command: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/anycable.RPC/Command".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_Disconnect: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/anycable.RPC/Disconnect".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl RPC for RPCClient {
    fn connect(&self, o: ::grpc::RequestOptions, p: super::anycable::ConnectionRequest) -> ::grpc::SingleResponse<super::anycable::ConnectionResponse> {
        self.grpc_client.call_unary(o, p, self.method_Connect.clone())
    }

    fn command(&self, o: ::grpc::RequestOptions, p: super::anycable::CommandMessage) -> ::grpc::SingleResponse<super::anycable::CommandResponse> {
        self.grpc_client.call_unary(o, p, self.method_Command.clone())
    }

    fn disconnect(&self, o: ::grpc::RequestOptions, p: super::anycable::DisconnectRequest) -> ::grpc::SingleResponse<super::anycable::DisconnectResponse> {
        self.grpc_client.call_unary(o, p, self.method_Disconnect.clone())
    }
}

// server

pub struct RPCServer;


impl RPCServer {
    pub fn new_service_def<H : RPC + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/anycable.RPC",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/anycable.RPC/Connect".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.connect(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/anycable.RPC/Command".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.command(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/anycable.RPC/Disconnect".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.disconnect(o, p))
                    },
                ),
            ],
        )
    }
}
