@0xe83067d6ca904cb4;

struct RegisterClientReq {
    clientid @0 :UInt64;
}

enum RegisterClientRes {
    accepted @0;
    denied @1;
}

struct MakeLock {
    lockId @0 :UInt64;
}
