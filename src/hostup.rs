use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::thread::sleep;
use std::time::Duration;
use std::iter::Iterator;

fn check<A: ToSocketAddrs>(addr: A) -> bool {
  match TcpStream::connect(addr) {
    Ok(_) => true,
    Err(_) => false,
  }
}

pub fn wait_on_connect<A: ToSocketAddrs>(addr: A) {
  let remote = addr.to_socket_addrs().unwrap().next().unwrap();
  info!("Waiting for host to come up on: {}", remote);
  loop {
    match check(&addr) {
      true => {
        info!("Remote host is up!: {}", remote);
        return;
      }
      false => (),
    };
    sleep(Duration::new(30, 0));
  }
}
