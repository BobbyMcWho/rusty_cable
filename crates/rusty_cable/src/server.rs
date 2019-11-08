// interface Server {
//  # Invoked on socket connection.
//  # socket_handle is an entity (object/record/whatever) representing connection socket
//  func socket_conn(socket_handle);

//  # Invoked on socket disconnection
//  func socket_disconn(socket_handle);

//  # Invoker on incoming message
//  func socket_data(socket_handle, msg);
// }

#[derive(Debug)]
struct Server {}

impl Server {
    fn socket_conn(socket_handle);
    fn socket_disconn(socket_handle);
    fn socket_data(socket_handle, msg)
}
