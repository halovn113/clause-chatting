# Claude Code — Full Tool Inventory & Khai Thác Cho Project Butler

> Tổng cộng: **57 tools** trong source + **7 tools/features chỉ có trên docs chính thức**. Phân tích cách khai thác từng tool cho Butler.

---

## 🔧 Base Tool System

| File | Nội dung | Khai thác |
|------|----------|-----------|
| `src/Tool.ts` | Interface `Tool` (~35 methods): `name`, `description()`, `prompt()`, `inputSchema`, `outputSchema`, `call()`, `isEnabled`, `isConcurrencySafe`, `isReadOnly`, `checkPermissions`... | **THAM KHẢO CHÍNH** — thiết kế `ITool` cho Butler. Pattern `buildTool()` với safe defaults rất hay. |
| `src/tools.ts` | `getAllBaseTools()` — single source of truth, `assembleToolPool()` merge built-in + MCP, `getTools()` filter by permission/mode | Học cách registry + dynamic tool pool. |
| `src/services/tools/toolOrchestration.ts` | `runTools()` — phân batch: read-only chạy song song (max 10), write chạy tuần tự | Học concurrency model cho tool execution. |

---

## 📁 File Operations (5)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 1 | **FileRead** | `FileReadTool/FileReadTool.ts` | ✅ Bắt buộc — đọc file với offset/limit, render ảnh/PDF. Học limit strategy. |
| ☐ 2 | **FileEdit** | `FileEditTool/FileEditTool.ts` | ✅ Quan trọng — exact string replace. Học cách detect file bị sửa ngoài ý muốn. |
| ☐ 3 | **FileWrite** | `FileWriteTool/FileWriteTool.ts` | ✅ Cơ bản. |
| ☐ 4 | **NotebookEdit** | `NotebookEditTool/NotebookEditTool.ts` | ⚠️ Optional — nếu sau này hỗ trợ Jupyter. |
| ☐ 5 | **Glob** | `GlobTool/GlobTool.ts` | ✅ Cần — fast file pattern matching. Học truncation strategy (100 files). |

---

## 🔍 Search Tools (2)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 6 | **Grep** | `GrepTool/GrepTool.ts` | ✅ Cần — search nội dung bằng regex. Học output modes (content/files_with_matches/count). |
| ☐ 7 | **WebSearch** | `WebSearchTool/WebSearchTool.ts` | ✅ Cho Research module. Dùng internal model nhỏ summarize kết quả. |

---

## 🌐 Web & Network (2)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 8 | **WebFetch** | `WebFetchTool/WebFetchTool.ts` | ✅ Cần — fetch URL → markdown, có preapproved hosts. |
| ☐ 9 | **WebBrowser** | `WebBrowserTool/WebBrowserTool.ts` | ⚠️ Feature-gated. Có thể skip v0.1. |

---

## 💻 Shell Execution (2)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 10 | **Bash** | `BashTool/BashTool.tsx` | ✅ **Bắt buộc** cho Butler. Background tasks, sandbox, git tracking, auto-classify commands. |
| ☐ 11 | **PowerShell** | `PowerShellTool/PowerShellTool.tsx` | ⚠️ Nếu hỗ trợ Windows. |

---

## 🤖 Agent/Worker Management (5)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 12 | **Agent (Task)** | `AgentTool/AgentTool.tsx` (1398 dòng — tool phức tạp nhất) | ✅ **CỰC KỲ QUAN TRỌNG**. Sub-agent spawning, async/sync, forked agents. Đây là pattern core cho multi-agent. |
| ☐ 13 | **TaskStop** | `TaskStopTool/TaskStopTool.ts` | ✅ Cần để kill background task. |
| ☐ 14 | **TaskOutput** | `TaskOutputTool/TaskOutputTool.tsx` | ✅ Lấy output từ background task, blocking mode. |
| ☐ 15 | **SendMessage** | `SendMessageTool/SendMessageTool.ts` | ✅ Gửi follow-up message cho agent đang chạy — enable agent continuation. |
| ☐ 16 | **TeamCreate** | `TeamCreateTool/TeamCreateTool.ts` | ⚠️ V1 skip (agent swarm). V2 cân nhắc. |
| ☐ 17 | **TeamDelete** | `TeamDeleteTool/TeamDeleteTool.ts` | ⚠️ V1 skip. |

---

## 📋 Task/Todo Management (5)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 18 | **TodoWrite** | `TodoWriteTool/TodoWriteTool.ts` | ✅ Học pattern quản lý todo in-session. |
| ☐ 19 | **TaskCreate** | `TaskCreateTool/TaskCreateTool.ts` | ✅ V2 task system — subject, description, hooks. |
| ☐ 20 | **TaskGet** | `TaskGetTool/TaskGetTool.ts` | ✅ |
| ☐ 21 | **TaskUpdate** | `TaskUpdateTool/TaskUpdateTool.ts` | ✅ — blocking relationships, task_completed hooks. |
| ☐ 22 | **TaskList** | `TaskListTool/TaskListTool.ts` | ✅ |

---

## 🔌 MCP Tools (4)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 23 | **MCPTool** | `MCPTool/MCPTool.ts` | ⚠️ V1 skip — MCP là protocol phức tạp. Học pattern thôi. |
| ☐ 24 | **ListMcpResources** | `ListMcpResourcesTool/ListMcpResourcesTool.ts` | ⚠️ Skip. |
| ☐ 25 | **ReadMcpResource** | `ReadMcpResourceTool/ReadMcpResourceTool.ts` | ⚠️ Skip. |
| ☐ 26 | **McpAuth** | `McpAuthTool/McpAuthTool.ts` | ⚠️ Skip. |

---

## 🎯 Skills & Commands (1)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 27 | **Skill** | `SkillTool/SkillTool.ts` (1108 dòng) | ✅ **Quan trọng** — pattern load skill/command từ local `.claude/commands/`, chạy như sub-agent. Dùng cho Butler để mở rộng capability. |

---

## 📐 Plan/Worktree Mode (4)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 28 | **EnterPlanMode** | `EnterPlanModeTool/EnterPlanModeTool.ts` | ✅ Map trực tiếp sang Planning mode của Butler (read-only). |
| ☐ 29 | **ExitPlanMode** | `ExitPlanModeTool/ExitPlanModeV2Tool.ts` | ✅ Học prompting chiến lược khi chuyển mode. |
| ☐ 30 | **EnterWorktree** | `EnterWorktreeTool/EnterWorktreeTool.ts` | ⚠️ Git-specific, có thể học session isolation. |
| ☐ 31 | **ExitWorktree** | `ExitWorktreeTool/ExitWorktreeTool.ts` | ⚠️ Git-specific. |

---

## ⚙️ Configuration & Discovery (2)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 32 | **Config** | `ConfigTool/ConfigTool.ts` | ✅ Cần tool get/set config (settings.defaultMode). |
| ☐ 33 | **ToolSearch** | `ToolSearchTool/ToolSearchTool.ts` | ✅ **RẤT HAY** — fuzzy search tool, "deferred loading" để giảm context. |

---

## 🧠 LSP / IDE (1)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 34 | **LSP** | `LSPTool/LSPTool.ts` | ⚠️ V1 skip. Có thể dùng cho Developer Assistant sau này. |

---

## ⏰ Scheduling / Automation (4)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 35 | **CronCreate** | `ScheduleCronTool/CronCreateTool.ts` | ✅ Cho daily briefing, steward auto-cleanup. Học durable scheduling. |
| ☐ 36 | **CronDelete** | `ScheduleCronTool/CronDeleteTool.ts` | ✅ |
| ☐ 37 | **CronList** | `ScheduleCronTool/CronListTool.ts` | ✅ |
| ☐ 38 | **RemoteTrigger** | `RemoteTriggerTool/RemoteTriggerTool.ts` | ❌ V1 skip (cloud-dependent). |

---

## 💬 User Interaction (2)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 39 | **AskUserQuestion** | `AskUserQuestionTool/AskUserQuestionTool.tsx` | ✅ Cần cho Ask/Careful mode — hỏi xác nhận với 2-4 options. |
| ☐ 40 | **Brief** | `BriefTool/BriefTool.ts` | ✅ Cho daily briefing, notification đến user. |

---

## 📊 Context & Output (3)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 41 | **StructuredOutput** | `SyntheticOutputTool/SyntheticOutputTool.ts` | ✅ JSON Schema validation — dùng khi cần structured response. |
| ☐ 42 | **Snip** | `SnipTool/SnipTool.ts` | ⚠️ Context trimming — học strategy. |
| ☐ 43 | **CtxInspect** | `CtxInspectTool/CtxInspectTool.ts` | ⚠️ Debug context — hữu ích khi phát triển. |

---

## 🧪 Tungsten / REPL (3)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 44 | **Tungsten** | `TungstenTool/TungstenTool.ts` | ❌ Ant-only internal. |
| ☐ 45 | **REPL** | `REPLTool/REPLTool.ts` | ⚠️ Học pattern wrapper — ẩn tool primitive sau interface REPL. |

---

## 🚀 Proactive / Kairos (7)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 46 | **Sleep** | `SleepTool/SleepTool.ts` | ⚠️ Có thể cần cho scheduled tasks. |
| ☐ 47 | **SuggestBackgroundPR** | `SuggestBackgroundPRTool/SuggestBackgroundPRTool.ts` | ❌ Ant-only. |
| ☐ 48 | **MonitorTool** | `MonitorTool/MonitorTool.ts` | ✅ Cho Steward module — monitor system. |
| ☐ 49 | **SendUserFile** | `SendUserFileTool/SendUserFileTool.ts` | ⚠️ Optional. |
| ☐ 50 | **PushNotification** | `PushNotificationTool/PushNotificationTool.ts` | ✅ Cho daily briefing notification. |
| ☐ 51 | **SubscribePR** | `SubscribePRTool/SubscribePRTool.ts` | ❌ GitHub-specific. |
| ☐ 52 | **Workflow** | `WorkflowTool/WorkflowTool.ts` | ✅ Load workflow từ `.butler/workflows/`. |

---

## 👥 Peer/Infrastructure (2)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 53 | **ListPeers** | `ListPeersTool/ListPeersTool.ts` | ❌ Skip. |
| ☐ 54 | **TerminalCapture** | `TerminalCaptureTool/TerminalCaptureTool.ts` | ⚠️ Optional. |

---

## 🧪 Test/Dev (3)

| # | Tool | File | Dùng cho Butler? |
|---|------|------|------------------|
| ☐ 55 | **TestingPermission** | `testing/TestingPermissionTool.tsx` | ❌ Test-only. |
| ☐ 56 | **OverflowTest** | `OverflowTestTool/OverflowTestTool.ts` | ❌ Skip. |
| ☐ 57 | **VerifyPlanExecution** | `VerifyPlanExecutionTool/VerifyPlanExecutionTool.ts` | ⚠️ Có thể dùng để verify plan output. |

---

## 📊 Tổng kết — Độ ưu tiên khai thác

### 🔴 BẮT BUỘC ĐỌC (Priority 1 — 17 files)

| File | Lý do |
|------|-------|
| `src/Tool.ts` | Base interface cho mọi tool — thiết kế ITool cho Butler |
| `src/tools.ts` | Tool registry + pool assembly + permission filter |
| `src/services/tools/toolOrchestration.ts` | Concurrency model |
| `src/tools/BashTool/BashTool.tsx` | Shell execution — core capability |
| `src/tools/AgentTool/AgentTool.tsx` | Multi-agent spawning — phức tạp nhất, đọc kỹ |
| `src/tools/FileReadTool/FileReadTool.ts` | File operations |
| `src/tools/FileEditTool/FileEditTool.ts` | File editing với conflict detection |
| `src/tools/FileWriteTool/FileWriteTool.ts` | File writing |
| `src/tools/GlobTool/GlobTool.ts` | File search |
| `src/tools/GrepTool/GrepTool.ts` | Content search |
| `src/tools/WebFetchTool/WebFetchTool.ts` | Web fetching |
| `src/tools/WebSearchTool/WebSearchTool.ts` | Web search |
| `src/tools/SkillTool/SkillTool.ts` | Skill/command loading |
| `src/tools/AskUserQuestionTool/AskUserQuestionTool.tsx` | User interaction pattern |
| `src/tools/ToolSearchTool/ToolSearchTool.ts` | Deferred tool discovery |
| `src/tools/EnterPlanModeTool/EnterPlanModeTool.ts` | Plan mode = Planning mode của Butler |
| `src/tools/SyntheticOutputTool/SyntheticOutputTool.ts` | Structured JSON output |

### 🟡 NÊN ĐỌC (Priority 2 — 10 files)

| File | Lý do |
|------|-------|
| `src/tools/TodoWriteTool/TodoWriteTool.ts` | Task management |
| `src/tools/TaskCreateTool/TaskCreateTool.ts` | V2 task system |
| `src/tools/TaskUpdateTool/TaskUpdateTool.ts` | Task state machine |
| `src/tools/BriefTool/BriefTool.ts` | Notification pattern |
| `src/tools/ScheduleCronTool/CronCreateTool.ts` | Scheduling |
| `src/tools/ConfigTool/ConfigTool.ts` | Settings management |
| `src/tools/SendMessageTool/SendMessageTool.ts` | Agent continuation |
| `src/tools/TaskOutputTool/TaskOutputTool.tsx` | Background task output |
| `src/tools/TaskStopTool/TaskStopTool.ts` | Task lifecycle |
| `src/tools/MonitorTool/MonitorTool.ts` | System monitoring |

### 🟢 THAM KHẢO SAU (Priority 3)

Các tool feature-gated, MCP, LSP, testing, peer infrastructure — đọc khi cần mở rộng.

---

## 🏗️ Bài học kiến trúc quan trọng cho Butler

### 1. Tool Interface Pattern
```
Tool {
  name, description(), prompt(),
  inputSchema, outputSchema,
  call(), checkPermissions(),
  isEnabled, isConcurrencySafe, isReadOnly
}
```
→ Butler nên có `ITool` trait tương tự trong Rust.

### 2. Tool Registry + Filter
- `getAllBaseTools()` — single source of truth
- `getTools()` — filter by permission context (Planning/Ask/Careful/YOLO)
- `assembleToolPool()` — merge built-in + external (MCP/plugin)

### 3. Concurrency Model
- Read-only tools → song song (max 10)
- Write tools → tuần tự
→ Tách tool thành 2 loại trong Butler.

### 4. Deferred Tool Loading (ToolSearch)
- Tool có `shouldDefer=true` thì ẩn khỏi context
- Model dùng ToolSearch để unlock
→ Giảm context size, rất quan trọng cho local model nhỏ.

### 5. Mode System (Plan → Execute)
- EnterPlanMode / ExitPlanMode
- Có plan approval flow
→ Map sang Planning → Ask → Careful → YOLO của Butler.

### 6. Permission Check
- Mỗi tool có `checkPermissions()` 
- Có `filterToolsByDenyRules()` 
→ Butler cần Safety Level (SAFE/CAREFUL/DANGEROUS).

### 7. Agent Spawning
- `AgentTool` spawn sub-agent với context riêng
- Background/async mode
- SendMessage để tiếp tục agent đang chạy
→ Core pattern cho Butler multi-agent (nếu cần sau này).

---

## 🆕 Tools & Features Chỉ Có Trên Docs Chính Thức (Không Có Trong Source Leak)

> Source code leak có thể cũ hơn bản production hiện tại. Các tool/features sau xuất hiện trên docs.anthropic.com nhưng KHÔNG có trong source leaked.

| # | Tool/Feature | Mô tả | Dùng cho Butler? |
|---|-------------|-------|-------------------|
| ☐ 58 | **ShareOnboardingGuide** | Upload ONBOARDING.md và trả share link cho teammate. Pro/Max/Team/Enterprise only | ⚠️ Optional — cho team onboarding. |
| ☐ 59 | **ScheduleWakeup** | Reschedule iteration tiếp theo cho `/loop` tự chọn interval. Claude tự gọi, user không gọi trực tiếp | ✅ Hay — dùng cho self-paced scheduling trong Butler. |
| ☐ 60 | **WaitForMcpServers** | Chờ MCP server kết nối xong trước khi gửi request | ❌ V1 skip (MCP-dependent). |
| ☐ 61 | **Monitor** (v2) | Chạy lệnh background, feed mỗi output line về Claude để react real-time | ✅ **Quan trọng** — Butler Steward module cần pattern này. |
| ☐ 62 | **Workflow** (v2 — dynamic) | Orchestrate nhiều subagent thành workflow dynamic | ✅ Cho Butler workflow automation. |

### 🔑 Systems & Concepts Chỉ Có Trên Docs

| System | Mô tả | Áp dụng cho Butler |
|--------|-------|--------------------|
| **Hooks** | Lifecycle hooks: PreToolUse, PostToolUse, SessionStart, Stop, SubagentStart/Stop, etc. Có 5 loại: command, HTTP, MCP tool, prompt, agent | ✅ **CỰC KỲ QUAN TRỌNG** — Butler cần hook system cho Steward module (cleanup, lint trước commit, v.v.) |
| **Skills** (SKILL.md) | Loadable workflows với YAML frontmatter, dynamic context injection, supporting files, `context: fork` cho subagent | ✅ Rất hay — Butler nên có system tương tự: `.butler/skills/` |
| **Auto Memory** (CLAUDE.md + MEMORY.md) | Persistent instructions + auto-learned notes. 200 lines / 25KB đầu được load mỗi session | ✅ **Map trực tiếp** sang Butler Memory module |
| **Subagents** (Explore, Plan, General) | Sub-agent spawning với custom system prompt, restricted tools, model selection | ✅ Agent system của Butler cần pattern này |
| **Permission Modes** | default, acceptEdits, auto, dontAsk, bypassPermissions, plan | ✅ Map sang Planning → Ask → Careful → YOLO |
| **Agent Teams** | Multi-agent coordination (CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS) | ⚠️ V1 skip, V2 cân nhắc |
| **Background Agents** | Chạy agent ngầm, monitor từ agent view | ✅ Butler cần background task execution |
| **Worktrees** | Git worktree isolation cho subagent | ⚠️ Optional — developer feature |
| **Routines** | Scheduled tasks chạy trên Anthropic infrastructure | ⚠️ Cloud-dependent, V1 skip |
| **Channels** | Push events từ Telegram, Discord, iMessage vào session | ✅ Interesting — Butler có thể push notification tương tự |
| **Remote Control** | Tiếp tục session từ điện thoại/browser khác | ⚠️ V1 skip |

### 📋 Commands Chỉ Có Trên Docs

| Command | Mô tả | Butler tương đương |
|---------|-------|---------------------|
| `/init` | Tạo CLAUDE.md tự động | `/butler init` |
| `/memory` | Edit CLAUDE.md + auto-memory | `/butler memory` |
| `/plan` | Vào plan mode | `/butler plan` |
| `/compact` | Tóm tắt context | `/butler compact` |
| `/btw` | Câu hỏi phụ không thêm vào history | Side-question pattern |
| `/model` | Switch model | `/butler model` |
| `/effort` | Adjust reasoning level | `/butler effort` |
| `/loop` | Chạy prompt lặp lại | `/butler loop` |
| `/batch` | Phân tách task lớn thành parallel subagents | `/butler batch` |
| `/code-review` | Review diff với effort level | `/butler review` |
| `/deep-research` | Fan-out web search, cross-check, synthesize | `/butler research` |
| `/skills` | Quản lý skills | `/butler skills` |
| `/agents` | Quản lý subagents | `/butler agents` |
| `/diff` | Interactive diff viewer | `/butler diff` |
| `/doctor` | Diagnose installation | `/butler doctor` |
| `/context` | Visualize context usage | `/butler context` |
| `/goal` | Set condition để Claude tự tiếp tục | `/butler goal` |

---

## 🏗️ Bài học kiến trúc MỚI từ Docs (Không có trong Source)

### 8. Permission Mode System (5 levels)

```
default → acceptEdits → auto → dontAsk → bypassPermissions
```
- `default`: prompt cho mọi thứ
- `acceptEdits`: auto-accept file edits, prompt cho cmds ngoài working dir
- `auto`: background classifier review commands
- `dontAsk`: auto-deny prompts (allowed tools vẫn hoạt động)
- `bypassPermissions`: skip tất cả (an toàn nhất cho automation)

→ Butler nên implement 4 levels: Planning, Ask, Careful, YOLO (như đã planned)

### 9. Skill System (SKILL.md)

```yaml
---
name: my-skill
description: What this skill does
context: fork  # chạy trong subagent riêng
allowed-tools: Bash(git *)  # pre-approve tools
disable-model-invocation: true  # chỉ manual
---
Skill instructions here...
```

Key features:
- **Dynamic context injection**: `` !`git diff HEAD` `` chạy shell commands trước khi gửi prompt
- **Supporting files**: template.md, examples/, scripts/
- **Permission override**: `allowed-tools` auto-approve trong skill scope
- **Frontmatter control**: who invokes, agent type, model, effort level

→ Butler nên có `.butler/skills/` với pattern tương tự

### 10. Hook System (Lifecycle Hooks)

```json
{
  "hooks": {
    "PreToolUse": [{ "matcher": "Bash", "hooks": [{ "type": "command", "command": "..." }] }],
    "PostToolUse": [{ "matcher": "Edit|Write", "hooks": [{ "type": "command", "command": "lint-check.sh" }] }],
    "SessionStart": [...],
    "Stop": [...]
  }
}
```

5 loại hooks:
- **command**: chạy shell script, stdin JSON, stdout JSON decision
- **http**: POST đến URL
- **mcp_tool**: gọi MCP tool
- **prompt**: gửi đến LLM, trả về yes/no
- **agent**: spawn subagent để verify

→ Butler **BẮT BUỘC** cần hook system cho Steward module

### 11. Auto Memory System

- `MEMORY.md` — 200 lines / 25KB loaded mỗi session
- Topic files (`debugging.md`, `api-conventions.md`) — load on demand
- Auto-write khi Claude học được gì
- Per-project, per-user scope

→ Map trực tiếp sang Butler Memory module

### 12. Subagent System

```yaml
---
name: explore-codebase
description: Fast read-only codebase exploration
tools: Read, Glob, Grep
model: haiku
---
```

Key patterns:
- **Explore**: haiku model, read-only, fast
- **Plan**: inherit model, read-only, planning
- **General-purpose**: inherit model, all tools
- **Custom**: user-defined system prompt, tool restriction, model selection
- **Memory scope**: user/project/local
- **Isolation**: worktree isolation cho subagent

→ Butler multi-agent nên implement pattern này

### 13. Context Window Management

- `/compact` — tóm tắt conversation, keep skills + MEMORY.md
- Skills được re-attach sau compact (5,000 tokens/skill, max 25,000 tokens total)
- `Snip` tool — trim history
- Context visualization (`/context`)

→ Butler cần compact/auto-compact cho local model nhỏ

### 14. Monitor Tool Pattern

Monitor chạy command background, feed mỗi output line về Claude real-time để react:
- Tail log files → flag errors
- Poll PR/CI status → report changes
- Watch directory → react to file changes

→ Hay cho Butler Steward module

---

## 🔗 Liên kết với Butler Architecture

| Butler Module | Claude Code Files cần đọc |
|---------------|---------------------------|
| **Chat Module** | `src/QueryEngine.ts`, `src/services/SessionMemory/` |
| **Memory Module** | `src/memdir/`, `src/services/autoDream/` + Auto Memory docs |
| **Steward Module** | `src/tools/BashTool/`, `src/tools/MonitorTool/` + Hooks docs |
| **Research Module** | `src/tools/WebSearchTool/`, `src/tools/WebFetchTool/` + `/deep-research` skill |
| **Companion Module** | `src/buddy/`, `src/services/tips/` + Daily Briefing pattern |
| **Tasks Module** | `src/tools/TodoWriteTool/`, `src/tasks/` + TaskCreate/Update docs |
| **Tool System** | `src/Tool.ts`, `src/tools.ts`, `src/services/tools/` + Skill docs |
| **Mode System** | `src/tools/EnterPlanModeTool/`, permission filters + 5-level permission docs |
| **LLM Adapter** | `src/QueryEngine.ts` — streaming, context injection |
| **Hook System** | **Docs only** — lifecycle hooks, 5 types, JSON input/output schema |
| **Skill System** | `src/tools/SkillTool/` + **Docs** — SKILL.md frontmatter, dynamic injection |

---

## 📝 Tóm tắt — Cần Update Butler Architecture

Dựa trên docs chính thức, Butler cần bổ sung:

1. **Hook System** — PreToolUse, PostToolUse, SessionStart, Stop (5 loại: command, http, mcp, prompt, agent)
2. **Skill System** — `.butler/skills/` với SKILL.md, dynamic context (`` !`cmd` ``), supporting files
3. **Auto Memory** — MEMORY.md auto-learn, topic files, per-project scope
4. **Subagent patterns** — Explore (read-only, fast), Plan (read-only, inherit model), Custom (user-defined)
5. **5-level Permission** — default, acceptEdits, auto, dontAsk, bypassPermissions
6. **Monitor pattern** — background process monitoring, real-time reaction
7. **Context Management** — auto-compact, skill re-attachment, context visualization
8. **CLAUDE.md / BUTLER.md** — persistent instruction file, path-scoped rules (.butler/rules/)
