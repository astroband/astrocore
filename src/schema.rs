table! {
    peers (ip, port) {
        ip -> Text,
        port -> Integer,
        nextattempt -> Timestamp,
        numfailures -> Integer,
    }
}
