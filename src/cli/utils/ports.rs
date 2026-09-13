use crate::cli::utils::terminal::warning;
use std::net::TcpListener;

/// Bind the first port at or above `start_port` that is free on `host`.
///
/// The returned listener is handed back to the caller so it can keep holding the
/// port until every service port has been picked, which stops two services from
/// resolving to the same port.
pub fn bind_available_port(host: &str, start_port: u16, port_label: &str) -> (u16, TcpListener) {
    let mut port = start_port;
    let mut listener = TcpListener::bind(format!("{}:{}", host, port));

    while listener.is_err() {
        warning(format!("{} {} is not available", port_label, port).as_str());
        port += 1;
        listener = TcpListener::bind(format!("{}:{}", host, port));
    }

    // The loop only exits once the bind succeeded.
    (port, listener.expect("port listener should be bound"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_available_port_returns_the_requested_port_when_free() {
        // Ask the OS for a free port, release it, then claim it back.
        let free_port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();

        let (port, listener) = bind_available_port("127.0.0.1", free_port, "Port");

        assert_eq!(port, free_port);
        assert_eq!(listener.local_addr().unwrap().port(), free_port);
    }

    #[test]
    fn test_bind_available_port_skips_a_busy_port() {
        // Hold a port so the helper is forced to walk past it.
        let occupied = TcpListener::bind("127.0.0.1:0").unwrap();
        let occupied_port = occupied.local_addr().unwrap().port();

        let (port, listener) = bind_available_port("127.0.0.1", occupied_port, "Port");

        assert!(
            port > occupied_port,
            "expected a port above the occupied {}, got {}",
            occupied_port,
            port
        );
        assert_eq!(listener.local_addr().unwrap().port(), port);
    }
}
