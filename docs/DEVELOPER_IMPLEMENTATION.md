# Butler Developer Implementation Guide

## Technology Stack

Frontend:

* Tauri v2
* React
* TypeScript
* Tailwind

Backend:

* Rust

AI Runtime:

* Ollama
* OpenAI Compatible APIs

Default Model:

* Gemma 4 E2B Instruct Q4_K_M

---

# Architecture

Frontend
↓
Agent Runtime
↓
Memory Manager
↓
Tool Router
↓
OS Tools

---

# Project Structure

/src
/app
/features
/components
/services

/src/features
chat
memory
research
steward
companion
tasks

/src/services
llm
shell
filesystem
browser
memory

---

# Core Modules

## Chat Module

Responsibilities:

* User conversation
* Session handling
* Context injection

Features:

* Markdown rendering
* Streaming responses

---

## Memory Module

Storage Format:

Markdown

Directory:

memory/

memory/projects/
memory/preferences/
memory/habits/
memory/computer/

Responsibilities:

* Read memory
* Write memory
* Summarize memory

---

## Companion Module

State Machine

States:

Idle
Thinking
Working
Warning
Happy

Responsibilities:

* Expression control
* Daily briefing
* Presence

---

## Steward Module

Capabilities:

* Disk analysis
* Cache cleanup
* Temp cleanup
* Process inspection

Safety Levels:

SAFE
CAREFUL
DANGEROUS

---

## Research Module

Capabilities:

* Search web
* Summarize pages
* Compare products
* Save findings

---

# LLM Adapter

Interface:

ILlmProvider

Methods:

generate()
stream()
embedding()
health()

Providers:

OllamaProvider
OpenAIProvider

---

# Tool System

ITool

Properties:

name
description
permissionLevel

Methods:

execute()

---

# Built-in Tools

DiskTool

Capabilities:

* Analyze storage
* Detect large folders

---

ProcessTool

Capabilities:

* Process list
* Memory usage

---

FileTool

Capabilities:

* Search
* Move
* Rename

---

ShellTool

Capabilities:

* Execute commands

Restrictions:

No elevated privileges by default.

---

# Mode System

Planning

No execution.

---

Ask

Confirm all actions.

---

Careful

Auto-execute SAFE actions.

---

YOLO

Execute all actions.

---

# Confidence System

Every response includes:

confidenceScore

Range:

0.0 - 1.0

Rules:

confidence < 0.65

→ search web

confidence < 0.40

→ ask user

Never hallucinate if confidence is low.

---

# Context Strategy

Do not inject entire history.

Use:

Current Session
+
Relevant Memory
+
Task Context

Only.

---

# Token Optimization

Never send:

* Full logs
* Full files
* Full folders

Use preprocessing.

Examples:

* Extract errors
* Extract warnings
* Generate summaries

before sending to LLM.

---

# MVP Milestones

## v0.1

Chat
Memory
Shell

---

## v0.2

Steward Tools

---

## v0.3

Research

---

## v0.4

Companion

---

## v0.5

Planning / Ask / Careful / YOLO

---

# Non Goals

Version 1.0 must NOT include:

* Plugin marketplace
* Agent swarm
* Autonomous self-modification
* Remote execution

Keep architecture simple.
