# HealthKeeper - 医疗档案管理系统

一个基于 AI 的本地医疗档案管理工具，支持 OCR 识别和结构化数据提取。

## 功能特性

- 📋 **多成员管理** - 支持家庭成员档案
- 📎 **文件导入** - 支持图片和 PDF
- 🔍 **OCR 识别** - 使用多模态大模型识别文字
- 🤖 **AI 提取** - 自动提取诊断、药物等信息
- 🔎 **全文搜索** - 快速检索病历内容
- 💾 **本地存储** - 数据完全掌控

## 快速开始

### 安装依赖

```bash
# 安装 Rust (如果未安装)
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh -s -- -y
source $HOME/.cargo/env

# 构建项目
cargo build --release
```

### 配置

复制配置文件模板：

```bash
cp config/config.yaml.example config.yaml
```

编辑 `config.yaml`，配置你的 AI 服务：

```yaml
ocr:
  default: "qwen_vision"
  providers:
    qwen_vision:
      enabled: true
      endpoint: "https://your-api-endpoint"
      model: "qwen3.5-plus"
      api_key: "your-api-key"
      timeout: 300

llm:
  default: "dashscope"
  providers:
    dashscope:
      enabled: true
      endpoint: "https://your-api-endpoint"
      model: "kimi-k2.5"
      api_key: "your-api-key"
      timeout: 120
```

### 使用 CLI

```bash
# 创建人物
./target/release/hk person create --name "张三" --relationship self

# 查看人物列表
./target/release/hk person list

# 创建就诊记录
./target/release/hk visit create \
  --person <person_id> \
  --date 2026-03-22 \
  --hospital "协和医院" \
  --department "内科"

# 导入附件
./target/release/hk import \
  --visit <visit_id> \
  --file ./report.jpg

# OCR 识别
./target/release/hk ocr --attachment <attachment_id>

# AI 提取
./target/release/hk extract --attachment <attachment_id>

# 搜索
./target/release/hk search "头痛"
```

### 启动 Web UI

```bash
./target/release/hk-server
```

浏览器打开 http://localhost:3000

## 项目结构

```
health-keeper/
├── crates/
│   ├── core/           # 核心库 (数据模型、存储、AI)
│   ├── cli/            # 命令行工具
│   ├── server/         # Web 服务器
│   └── ffi/            # FFI 绑定 (供移动端使用)
├── apps/
│   └── web/            # Web 前端
├── migrations/         # 数据库迁移
├── config/             # 配置文件
└── data/               # 数据存储目录
```

## 技术栈

- **后端**: Rust + Axum
- **前端**: HTML + Tailwind CSS
- **数据库**: SQLite
- **AI**: 可插拔的 OCR/LLM 提供者

## 许可证

MIT