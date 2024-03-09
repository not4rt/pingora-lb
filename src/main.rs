use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {

    /// For this small example, we don't need context storage
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self.0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, false, "one.one.one.one".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request.insert_header("Host", "one.one.one.one").unwrap();
        Ok(())
    }
}

fn main() {
    let mut my_server = Server::new(None).unwrap();
    my_server.bootstrap();

    let server1_host = std::env::var("SERVER1_HOST").expect("SERVER1_HOST must be set");
    let server2_host = std::env::var("SERVER2_HOST").expect("SERVER2_HOST must be set");
    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set");
    
    let server1_addr = format!("{server1_host}:{server_port}");
    let server2_addr = format!("{server2_host}:{server_port}");
    let lb_addr = format!("0.0.0.0:{listen_port}");

    let upstreams =
        LoadBalancer::try_from_iter([server1_url, server2_url]).unwrap();

    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
        lb.add_tcp(&lb_addr);

    my_server.add_service(lb);

    my_server.run_forever();
}