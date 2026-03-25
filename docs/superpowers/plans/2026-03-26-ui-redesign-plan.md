# HealthKeeper UI 重设计实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 HealthKeeper 从灰白朴素风格重设计为温暖安心的仪表盘式布局

**Architecture:** 单页 HTML + Tailwind CSS，保持所有 API 接口和功能不变，仅重构视觉呈现层。采用仪表盘式布局：问候区 → 快速操作 → 家庭成员概览 → 时间线

**Tech Stack:** HTML5, Tailwind CSS (CDN), Noto Sans SC 字体, 原生 JavaScript

---

## 文件结构

| 文件 | 操作 | 说明 |
|------|------|------|
| `apps/web/index.html` | 重写 | 全新 UI 设计，保持 JS 逻辑不变 |

---

## Task 1: 基础框架和样式

**Files:**
- Modify: `apps/web/index.html:1-100` (head 部分)

- [ ] **Step 1: 添加 Google 字体和 CSS 变量**

在 `<head>` 中添加：
- Google Fonts: Noto Sans SC (weights: 400, 500, 600, 700)
- CSS 变量定义主题色 (--primary: #3b82f6 等)
- 基础 body 样式设置 font-family

- [ ] **Step 2: 验证字体加载**

启动服务器，在浏览器中打开 http://localhost:3000，确认 Noto Sans SC 字体生效

- [ ] **Step 3: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): add Noto Sans SC font and CSS variables"
```

---

## Task 2: 首页问候区

**Files:**
- Modify: `apps/web/index.html` (body 开始部分)

- [ ] **Step 1: 实现问候区 HTML 结构**

创建渐变背景问候区：
- 线性渐变 (135deg, #3b82f6 → #1d4ed8)
- 问候时间文字（早上好/下午好/晚上好）
- 用户名显示
- 健康提示文字
- 通知图标按钮

- [ ] **Step 2: 添加动态问候逻辑**

JavaScript 函数：
- `updateGreeting()`: 根据当前时间设置问候语
- 从 persons 数组获取当前选中用户名

- [ ] **Step 3: 在 init() 中调用问候函数**

- [ ] **Step 4: 测试问候区**

确认渐变背景显示正确，问候语随时间变化

- [ ] **Step 5: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): add greeting section with gradient background"
```

---

## Task 3: 快速操作区

**Files:**
- Modify: `apps/web/index.html` (问候区下方)

- [ ] **Step 1: 实现快速操作区 HTML**

三个操作按钮卡片：
- 快速录入（突出显示，蓝色边框）
- 搜索记录
- 家庭成员（显示成员数量）

- [ ] **Step 2: 添加辅助函数**

- `focusSearch()`: 聚焦搜索框
- `updateMemberCount()`: 更新成员数量显示

- [ ] **Step 3: 在 loadPersons() 中更新成员数量**

- [ ] **Step 4: 测试快速操作区**

确认按钮点击正常，成员数量显示正确

- [ ] **Step 5: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): add quick action buttons section"
```

---

## Task 4: 家庭成员概览

**Files:**
- Modify: `apps/web/index.html` (快速操作区下方)

- [ ] **Step 1: 实现家庭成员概览容器**

白色卡片容器：
- 标题 "家庭成员"
- 添加成员链接
- 横向滚动的成员卡片容器

- [ ] **Step 2: 重写 renderPersons() 函数**

新样式：
- 横向卡片布局
- 选中成员：渐变蓝色背景
- 未选中成员：浅灰背景 + 边框
- 圆形头像图标
- 姓名 + 关系标签

- [ ] **Step 3: 测试成员概览**

确认卡片显示正确，选中状态高亮，点击切换正常

- [ ] **Step 4: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): redesign member cards with horizontal scroll"
```

---

## Task 5: 就诊记录时间线

**Files:**
- Modify: `apps/web/index.html` (成员概览下方)

- [ ] **Step 1: 实现时间线容器**

- 标题 "最近就诊"
- 成员筛选标签（全部/各成员）
- 时间线容器（左侧圆点连线）

- [ ] **Step 2: 重写 renderVisits() 为时间线样式**

- 左侧渐变时间轴线
- 圆点标记（今天用蓝色高亮）
- 记录卡片：日期、医院、科室、诊断
- 附件状态标签（已提取/待处理）
- 点击进入详情

- [ ] **Step 3: 测试时间线**

确认时间线显示正确，记录卡片样式美观

- [ ] **Step 4: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): redesign visits as timeline view"
```

---

## Task 6: 模态框样式重构

**Files:**
- Modify: `apps/web/index.html` (所有模态框)

- [ ] **Step 1: 添加模态框基础 CSS**

- `.modal`: 全屏遮罩 + flex 居中
- `.modal-overlay`: 半透明背景 + 模糊效果
- `.modal-content`: 白色圆角卡片 + 阴影
- `.modal-bottom-sheet`: 底部上滑面板

- [ ] **Step 2: 重构添加成员弹窗**

- 标题栏（标题 + 关闭按钮）
- 表单区：
  - 姓名输入框
  - 关系选择按钮组
  - 出生日期选择器
  - 性别选择按钮组
- 底部操作栏（取消/保存按钮）

- [ ] **Step 3: 添加选择函数**

- `selectRelation(el, value)`: 关系按钮选择
- `selectGender(el, value)`: 性别按钮选择

- [ ] **Step 4: 测试模态框**

确认弹窗显示正确，交互动画流畅

- [ ] **Step 5: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): redesign modal with new style"
```

---

## Task 7: 成员详情页重构

**Files:**
- Modify: `apps/web/index.html` (personDetailModal)

- [ ] **Step 1: 重构成员详情头部**

- 渐变背景头部
- 圆形头像
- 姓名 + 基本信息（关系、性别、年龄）
- 标签（血型、BMI）
- 操作按钮（删除、编辑、关闭）

- [ ] **Step 2: 重写 renderDetailView()**

健康卡片区域：
- 慢性病卡片（黄色背景）
- 过敏卡片（红色背景）
- 长期用药卡片（蓝色背景）

就诊历史区域：
- 标题 + 数量统计
- 记录列表（最近5条）
- 点击进入详情

- [ ] **Step 3: 添加辅助函数**

- `calculateAge(birthDate)`: 计算年龄
- `loadPersonVisits(personId)`: 加载成员就诊记录

- [ ] **Step 4: 测试成员详情**

确认头部渐变显示正确，健康卡片颜色区分

- [ ] **Step 5: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): redesign person detail with gradient header"
```

---

## Task 8: 就诊记录详情重构

**Files:**
- Modify: `apps/web/index.html` (visitDetailModal)

- [ ] **Step 1: 重构就诊详情模态框结构**

- 头部信息区：
  - 医院、科室、医生
  - 患者名、日期
  - 编辑/关闭按钮
- 内容区：滚动容器

- [ ] **Step 2: 重写 renderVisitDetailView()**

信息卡片：
- 诊断卡片（蓝色标题）
- 主诉/治疗双列卡片
- 附件缩略图网格
- 删除按钮

- [ ] **Step 3: 测试就诊详情**

确认信息展示清晰，附件缩略图显示正确

- [ ] **Step 4: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): redesign visit detail with card layout"
```

---

## Task 9: 快速录入流程优化

**Files:**
- Modify: `apps/web/index.html` (visitModal)

- [ ] **Step 1: 重构快速录入弹窗**

- 标题栏
- 上传区域：
  - 渐变背景 + 虚线边框
  - 图标 + 提示文字
  - 隐藏的文件输入
- 进度条区域：
  - 旋转加载图标
  - 进度条 + 文字提示
- 归档选项复选框
- 表单字段：
  - 患者/日期 双列
  - 医院/科室 双列
  - 诊断文本框
- 底部操作栏

- [ ] **Step 2: 测试快速录入**

确认上传区域显示正确，进度条动画流畅

- [ ] **Step 3: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): redesign quick import modal"
```

---

## Task 10: 搜索页面样式

**Files:**
- Modify: `apps/web/index.html` (搜索相关)

- [ ] **Step 1: 添加搜索栏样式**

- 白色卡片容器
- 搜索图标 + 输入框
- 筛选按钮

- [ ] **Step 2: 筛选面板**

- 成员筛选按钮组
- 横向滚动

- [ ] **Step 3: 测试搜索功能**

确认搜索栏样式美观，筛选功能正常

- [ ] **Step 4: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): redesign search bar with modern style"
```

---

## Task 11: Toast 提示组件

**Files:**
- Modify: `apps/web/index.html`

- [ ] **Step 1: 添加 Toast 组件 HTML**

- 固定定位在顶部中央
- 初始状态在屏幕外上方
- 包含：图标、标题、消息

- [ ] **Step 2: 实现 showToast() 函数**

参数：
- title: 标题
- message: 消息
- type: success/error/warning/info

功能：
- 设置图标和颜色
- 动画滑入显示
- 3秒后自动滑出隐藏

- [ ] **Step 3: 在操作成功时调用 Toast**

- 保存成员成功
- 保存就诊记录成功
- 其他操作反馈

- [ ] **Step 4: 测试 Toast**

确认显示和隐藏动画流畅

- [ ] **Step 5: Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): add toast notification component"
```

---

## Task 12: 最终测试和清理

- [ ] **Step 1: 构建并启动服务器**

```bash
cargo build --release && ./target/release/hk-server
```

- [ ] **Step 2: 功能测试清单**

- [ ] 首页问候区显示正确
- [ ] 快速操作按钮点击正常
- [ ] 家庭成员卡片显示和选择正常
- [ ] 就诊记录时间线显示正确
- [ ] 成员详情弹窗正常
- [ ] 就诊详情弹窗正常
- [ ] 快速录入流程正常
- [ ] 搜索功能正常
- [ ] 所有模态框样式正确
- [ ] 移动端响应式显示正常

- [ ] **Step 3: 清理无用代码**

删除旧的 CSS 样式和 HTML 结构

- [ ] **Step 4: 最终 Commit**

```bash
git add apps/web/index.html
git commit -m "feat(ui): complete UI redesign with dashboard layout"
```

---

## 执行顺序

按 Task 1 → Task 12 顺序执行，每个 Task 完成后提交 commit。

## 预计工作量

约 12 个 commit，每个 Task 约 10-20 分钟。