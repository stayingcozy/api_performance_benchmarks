# API Performance Benchmarks
Create api servers across the most popular backend programming languages with REST API, PostgreSQL, and Redis capabilities. Compare each api server and their respective most popular libraries for speed and memory performance.

Inspired from [TheOptimizationKing](https://medium.com/@optimzationking2) on medium in his article: [We Threw 1 Million Concurrent Users at Go, Rust, and Node — The Results Hurt](https://medium.com/@optimzationking2/we-threw-1-million-concurrent-users-at-go-rust-and-node-the-results-hurt-6cfa7ff6a4d0)


# Programming Languages to Test
- **Python**
    - FastAPI + asyncpg + redis-py
- **Node.js**
    - Express.js + pg + ioredis
- **Java**
    - Spring Boot + org.postgresql + Lettuce
- **Go**
    - Gin + pgx + go-redis
- **Rust**
    - Axum + sqlx + redis-rs
- **C++**
    - Oat++ + oatpp-postgresql + hiredis


# Architecture
coding_lang/
├── requestor/
│   └── notify (timing) + for loop with post
├── server/
│   └── server rest, sql, redis
└── docker/
    ├── docker-compose.yml
    ├── Dockerfile.requestor
    └── Dockerfile.server


# TODO
- [ ] Add benchmark framework (docker, timing/memory/energy) (great example: [kostya/benchmarks](https://github.com/kostya/benchmarks))
- [ ] Customize Node.js implementation
- [ ] Customize Go implementation
- [ ] Customize Rust implementation
- [ ] Create Python Server
- [ ] Create Java Server
- [ ] Create C++ Server