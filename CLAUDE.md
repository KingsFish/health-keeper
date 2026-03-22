# HealthKeeper 项目说明

医疗档案管理系统 - 本地存储 + AI OCR/提取

## 技术栈
- Rust (core + cli + server)
- Axum (Web 框架)
- SQLite (存储)
- Tailwind CSS (前端)

## AI 配置
- **OCR**: qwen3.5-plus (多模态，阿里云 DashScope)
- **LLM**: kimi-k2.5 (结构化提取)
- API endpoint: `https://coding.dashscope.aliyuncs.com/apps/anthropic`
- 需要 `User-Agent: curl/8.0` header 才能调用

## 常用命令
```bash
# 构建
cargo build --release

# CLI
./target/release/hk person create --name "张三" --relationship self
./target/release/hk visit create --person <id> --date 2026-03-22
./target/release/hk import --visit <id> --file ./report.jpg
./target/release/hk ocr --attachment <id>
./target/release/hk extract --attachment <id>

# Web UI
./target/release/hk-server  # http://localhost:3000
```

## 关键文件
- `crates/core/` - 核心库
- `crates/cli/` - 命令行工具
- `crates/server/` - Web 服务器
- `config.yaml` - 配置文件
- `ROADMAP.md` - 开发路线图

## 当前进度
MVP 完成，详见 ROADMAP.md