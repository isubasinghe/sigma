@0xe83067d6ca904cb4;

using Rust = import "rust.capnp";
$Rust.parentModule("shared");

struct RegisterClientReq {
    id @0 :Pair;
}

enum RegisterClientRes {
    accepted @0;
    denied @1;
}

struct MakeLockReq {
    lockId @0 :UInt64;
}

struct MakeLockRes {
    success @0 :Bool;
}

struct Pair {
    id @0 :UInt64;
    timestamp @1 :UInt64;
}

struct Request {
    union {
        registerClient @0 :RegisterClientReq;
        makeLock @1 :MakeLockReq;
    }
}

struct Response {
    union {
        registerClient @0 :RegisterClientRes;
        makeLock @1 :MakeLockRes;
    }
}

struct Message {
    traceId @0 :UInt64;
    union {
      request @1 :Request;
      response @2 :Response;
    }
}

