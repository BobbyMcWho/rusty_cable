// interface Hub {
//   # Subscribe socket to the stream.
//   # We also need a channel_id to sign messages with it (see below)
//   func add(socket_handle, channel_id, stream);

//   # Unsubscribe socket from all streams for the given channel
//   func remove(socket_handle, channel_id);

//   # Broadcast a message to all subscribed sockets
//   func broadcast(stream, msg);
// }

struct Hub {}

impl Hub {
    fn add(socket_handle, channel_id, stream);
    fn remove(socket_handle, channel_id);
    fn broadcast(stream, msg);
}
