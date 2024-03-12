use async_trait::async_trait;
use pingora::prelude::*;
use std::sync::Arc;
use structopt::StructOpt;

use pingora_core::server::configuration::Opt;

pub struct LB(Arc<LoadBalancer<RoundRobin>>) ;

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

        //println!("upstream peer is: {upstream:?}");

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(upstream, false, "backend".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        _session.set_keepalive(Some(30));
        upstream_request.insert_header("Connection", "Keep-Alive").unwrap();
        upstream_request.insert_header("Keep-Alive", "timeout=5, max=1000").unwrap();
        upstream_request.insert_header("Proxy-Connection", "keep-alive").unwrap();
        Ok(())
    }
}

fn main() {
    // read command line arguments
    let opt = Opt::from_args();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let server1_addr = std::env::var("SERVER1_ADDR").expect("SERVER1_ADDR must be set");
    let server2_addr = std::env::var("SERVER2_ADDR").expect("SERVER2_ADDR must be set");

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set");
    let lb_addr = format!("0.0.0.0:{listen_port}");

    let upstreams =
        LoadBalancer::try_from_iter([server1_addr, server2_addr]).unwrap();

    let mut lb = http_proxy_service(&my_server.configuration, LB(Arc::new(upstreams)));
        lb.add_tcp(&lb_addr);

    my_server.add_service(lb);

    println!("Load-balancer running at http://{}/", lb_addr);
    my_server.run_forever();
}