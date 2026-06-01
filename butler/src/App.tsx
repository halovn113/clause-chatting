import { useState, useRef, useEffect, type KeyboardEvent } from "react";
import "./App.css";

type Message = {
  role: "user" | "assistant" | "system";
  content: string;
};

function App() {
  const [messages, setMessages] = useState<Message[]>([
    {
      role: "assistant",
      content: "Good morning. I'm your Butler. How can I help you today?",
    },
  ]);
  const [input, setInput] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const sendMessage = async () => {
    const text = input.trim();
    if (!text || isLoading) return;

    const userMsg: Message = { role: "user", content: text };
    setMessages((prev) => [...prev, userMsg]);
    setInput("");
    setIsLoading(true);

    try {
      const { invoke } = await import("@tauri-apps/api/core");

      const response = await invoke<string>("llm_generate", {
        messages: [...messages, userMsg].map((m) => ({
          role: m.role,
          content: m.content,
        })),
        config: {
          model_path: "",
          context_size: 8192,
          threads: 4,
          batch_size: 512,
          gpu_layers: 0,
          temperature: 0.7,
          max_tokens: 4096,
        },
      });

      setMessages((prev) => [
        ...prev,
        { role: "assistant", content: response },
      ]);
    } catch (e) {
      setMessages((prev) => [
        ...prev,
        {
          role: "system",
          content: `Error: ${e}. Make sure a model is loaded.`,
        },
      ]);
    } finally {
      setIsLoading(false);
    }
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      sendMessage();
    }
  };

  return (
    <div className="flex flex-col h-screen max-w-4xl mx-auto">
      {/* Header */}
      <header className="flex items-center justify-between px-4 py-3 border-b border-zinc-800 bg-zinc-950 shrink-0">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-emerald-500" />
          <span className="font-semibold text-zinc-100">Project Butler</span>
        </div>
        <div className="flex gap-3 text-xs text-zinc-500">
          <span>v0.1.0</span>
          <span>llama.cpp</span>
        </div>
      </header>

      {/* Messages */}
      <main className="flex-1 overflow-y-auto px-4 py-4 space-y-4">
        {messages.map((msg, i) => (
          <div
            key={i}
            className={`flex ${
              msg.role === "user" ? "justify-end" : "justify-start"
            }`}
          >
            <div
              className={`max-w-[80%] rounded-xl px-4 py-3 text-sm leading-relaxed ${
                msg.role === "user"
                  ? "bg-zinc-700 text-zinc-100"
                  : msg.role === "system"
                    ? "bg-red-900/30 text-red-300 border border-red-800"
                    : "bg-zinc-800 text-zinc-200"
              }`}
            >
              {msg.content}
            </div>
          </div>
        ))}
        {isLoading && (
          <div className="flex justify-start">
            <div className="bg-zinc-800 rounded-xl px-4 py-3 text-sm text-zinc-400">
              <span className="animate-pulse">Thinking</span>
              <span className="animate-pulse" style={{ animationDelay: "0.2s" }}>.</span>
              <span className="animate-pulse" style={{ animationDelay: "0.4s" }}>.</span>
              <span className="animate-pulse" style={{ animationDelay: "0.6s" }}>.</span>
            </div>
          </div>
        )}
        <div ref={messagesEndRef} />
      </main>

      {/* Input */}
      <footer className="px-4 py-3 border-t border-zinc-800 bg-zinc-950 shrink-0">
        <div className="flex gap-2 items-end">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Message Butler..."
            rows={1}
            className="flex-1 bg-zinc-800 border border-zinc-700 rounded-lg px-4 py-2.5 text-sm text-zinc-100 placeholder-zinc-500 resize-none focus:outline-none focus:border-emerald-600 focus:ring-1 focus:ring-emerald-600/50"
            disabled={isLoading}
          />
          <button
            onClick={sendMessage}
            disabled={isLoading || !input.trim()}
            className="bg-emerald-600 hover:bg-emerald-500 disabled:bg-zinc-700 disabled:text-zinc-500 text-white rounded-lg px-4 py-2.5 text-sm font-medium transition-colors shrink-0"
          >
            Send
          </button>
        </div>
        <div className="flex gap-4 mt-2 text-xs text-zinc-600">
          <span>Model: none loaded</span>
          <span>Memory: 0 entries</span>
          <span>Mode: Ask</span>
        </div>
      </footer>
    </div>
  );
}

export default App;
