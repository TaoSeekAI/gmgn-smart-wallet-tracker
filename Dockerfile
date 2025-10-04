# 多阶段构建 Dockerfile
FROM rust:1.90-slim as builder

# 安装编译依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 创建工作目录
WORKDIR /app

# 复制项目文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 构建发布版本
RUN cargo build --release

# 运行时镜像
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -u 1000 gmgn && \
    mkdir -p /app/.cache && \
    chown -R gmgn:gmgn /app

# 切换到非 root 用户
USER gmgn
WORKDIR /app

# 从构建阶段复制可执行文件
COPY --from=builder --chown=gmgn:gmgn /app/target/release/gmgn-tracker /usr/local/bin/

# 复制默认配置 (可选)
COPY --chown=gmgn:gmgn config.yaml.example /app/config.yaml

# 暴露日志目录作为 volume
VOLUME ["/app/logs", "/app/.cache"]

# 默认命令
ENTRYPOINT ["gmgn-tracker"]
CMD ["--help"]
