// interface Pinger {
//   # Add socket to the active list
//   func register(socket_handle);

//   # Remove socket from the active list
//   func unregister(socket_handle);
// }

struct Pinger {}

impl Pinger {
    fn register(socket_handle);
    fn unregister(socket_handle);
    fn loop() {
        while(true) {

        }
    }
}

// func loop() {
//   while(true) {
//     var msg = ping_message()
//     for (socket in active_sockets) {
//       socket.transmit(msg)
//     }
//     sleep(INTERVAL)
//   }
// }


// func ping_message() {
//   return json_encode(['type', 'message'], ['ping', time.utc()])
// }
