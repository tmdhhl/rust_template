# ---- 构建阶段 ----
FROM rust:1.87 AS builder

WORKDIR /opt/kuai_saver

# 预先拷贝依赖文件，加速依赖缓存
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/kuai_saver*

# 拷贝源码并编译
COPY . .
RUN cargo build --release

# ---- 运行阶段 ----
FROM debian:bookworm-slim

WORKDIR /opt/kuai_saver
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

# 拷贝可执行文件
COPY --from=builder /opt/kuai_saver/target/release/kuai_saver .

# 如有静态资源或配置文件，按需拷贝
COPY configuration ./configuration

# 暴露端口（如有需要）
EXPOSE 8000

# 启动命令
CMD ["./kuai_saver"]