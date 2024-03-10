# Pingora Load Balancer
A simple *blazing fast* load balancer using pingora, uses round-robin between 2 servers.

## Environment Variables Needed
- **SERVER1_ADDR** - First backend host/ip and port.                Example: SERVER1_ADDR="1.1.1.1:443"
- **SERVER2_ADDR** - Second backend host/ip and port.               Example: SERVER2_ADDR="1.0.1.0:443"
- **LISTEN_PORT** - The port this load-balancer will listen to.     Example: LISTEN_PORT="9999"

## Docker
With **docker + docker-compose** installed, run:
```
git clone https://github.com/not4rt/pingora-lb.git
cd pingora-lb
docker compose up
```