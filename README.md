# balance_this_load

A learning project to build a small TCP load balancer in Rust, with room to add an HTTP reverse proxy later.

The goal is to understand how a load balancer accepts connections, chooses an application server, forwards traffic, checks backend health, and reports what is happening. The project is being built in small pieces so each part is easier to read and understand.

## Planned features

When the project is finished, the TCP load balancer should:

- Read its listener, backend servers, scheduling choice, health checks, and limits from a TOML config file.
- Accept client TCP connections and choose a healthy backend for each connection.
- Support round robin and least connections scheduling.
- Forward bytes between the client and backend without changing or inspecting them.
- Handle connection setup failures, disconnects, half-closes, timeouts, and graceful shutdown.
- Keep connection counts accurate and stop accepting work when configured limits are reached.
- Check backend health in the background and stop sending new connections to unhealthy servers.
- Write structured logs and expose useful metrics, such as accepted, active, and failed connections and bytes forwarded.
- Include tests with local backend servers for normal traffic, distribution, backend failures, health changes, and connection cleanup.
- Include instructions for running the load balancer and its test backends locally.

A later HTTP mode may parse HTTP requests and add host or path routing, request-level metrics, and TLS termination. The first version will remain a TCP proxy; it will treat the payload as data to pass through.

## Architecture planned

The code is split into small modules:

- `backend` stores backend addresses and state.
- `scheduler` chooses an eligible backend using the selected algorithm.
- `health` checks backend availability and updates health state.
- `config` reads and validates the TOML settings.
- `proxy` accepts connections and forwards data to and from backends.
- `metrics` tracks connection and byte counts.
- `shutdown` coordinates stopping the listener and active connections.
- `main` loads configuration and starts the pieces.

The backend picker will not handle network traffic. The connection handler will not decide health policy. Keeping these jobs separate should make it easier to understand and change one part at a time.

## Current status

The project is under construction. The backend address type and round-robin scheduler are the first building blocks. The listener, TCP forwarding, configuration, health checks, metrics, and integration tests have not been added yet.

## Running

Run instructions will be added once the first working TCP proxy milestone is complete.
