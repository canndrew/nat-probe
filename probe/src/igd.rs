use priv_prelude::*;
use tokio_io;

#[derive(Debug)]
pub enum IgdResults {
    NoResponse,
    GotResponse(Vec<u8>),
}

#[async]
pub fn check_igd(handle: Handle) -> Result<IgdResults, !> {
    let request = "\
        M-SEARCH * HTTP/1.1\n\
        Host:239.255.255.250:1900\n\
        ST:urn:schemas-upnp-org:device:InternetGatewayDevice:1\n\
        Man:\"ssdp:discover\"\n\
        MX:3\n\
    ";

    let socket = match UdpSocket::bind(&addr!("0.0.0.0:0"), &handle) {
        Err(e) => panic!("unable to bind udp socket: {}", e),
        Ok(socket) => socket,
    };

    let socket = match await!(socket.send_dgram(request, addr!("239.255.255.250:1900"))) {
        Err(e) => panic!("unable to send IGD request"),
        Ok((socket, _req)) => socket,
    };

    let res = await!({
        socket
        .recv_dgram(iter::repeat(0u8).take(1024).collect::<Vec<u8>>())
        .with_timeout(Duration::from_secs(1), &handle)
    });
    let data = match res {
        Err(e) => panic!("error reading on UDP socket: {}", e),
        Ok(None) => {
            return Ok(IgdResults::NoResponse);
        },
        Ok(Some((_socket, mut data, n, _addr))) => {
            data.truncate(n);
            data
        }
    };

    Ok(IgdResults::GotResponse(data))
}

