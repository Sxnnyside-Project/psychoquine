import { useSignal } from "@preact/signals";
import { useCallback } from "preact/hooks";

interface QuineOutput {
  original: string;
  one_line: string;
  multi_line: string;
  escape_strategy: string;
  stats: {
    input_bytes: number;
    one_line_bytes: number;
    multi_line_bytes: number;
    expansion_ratio: number;
  };
}

interface GenerateResponse {
  success: boolean;
  data?: QuineOutput;
  error?: string;
}

// Check if running in Tauri
const isTauri = typeof window !== "undefined" && "__TAURI__" in window;

// Fallback quine generator for web-only mode
function generateQuineFallback(input: string): QuineOutput {
  const escaped = input
    .replace(/\\/g, "\\\\")
    .replace(/"/g, '\\"')
    .replace(/\n/g, "\\n")
    .replace(/\r/g, "\\r")
    .replace(/\t/g, "\\t");

  const oneLine =
    `(function(){var d="${escaped}";var q=String.fromCharCode(34);console.log("(function(){var d="+q+d+q+";"+d.slice(d.indexOf("var q")))})()`;

  const multiLine = `(function() {
    var d = "${escaped}";
    var q = String.fromCharCode(34);
    console.log("(function(){var d=" + q + d + q + ";" + d.slice(d.indexOf("var q")));
})()`;

  return {
    original: input,
    one_line: oneLine,
    multi_line: multiLine,
    escape_strategy: "Standard",
    stats: {
      input_bytes: input.length,
      one_line_bytes: oneLine.length,
      multi_line_bytes: multiLine.length,
      expansion_ratio: oneLine.length / input.length,
    },
  };
}

export default function QuineGenerator() {
  const input = useSignal("");
  const output = useSignal<QuineOutput | null>(null);
  const error = useSignal<string | null>(null);
  const isLoading = useSignal(false);
  const activeTab = useSignal<"one_line" | "multi_line">("one_line");
  const escapeStrategy = useSignal("standard");
  const copySuccess = useSignal<string | null>(null);

  const handleGenerate = useCallback(async () => {
    if (!input.value.trim()) {
      error.value = "Input cannot be empty";
      return;
    }

    isLoading.value = true;
    error.value = null;

    try {
      if (isTauri) {
        // Dynamic import for Tauri
        const { invoke } = await import("@tauri-apps/api");
        const response = await invoke<GenerateResponse>("generate_quine", {
          input: input.value,
          options: {
            escape_strategy: escapeStrategy.value,
          },
        });

        if (response.success && response.data) {
          output.value = response.data;
        } else {
          error.value = response.error || "Unknown error occurred";
        }
      } else {
        // Web fallback
        output.value = generateQuineFallback(input.value);
      }
    } catch (err) {
      error.value = err instanceof Error
        ? err.message
        : "Failed to generate quine";
    } finally {
      isLoading.value = false;
    }
  }, []);

  const handleCopy = useCallback(async (text: string, label: string) => {
    try {
      if (isTauri) {
        const { writeText } = await import("@tauri-apps/api/clipboard");
        await writeText(text);
      } else {
        await navigator.clipboard.writeText(text);
      }
      copySuccess.value = label;
      setTimeout(() => {
        copySuccess.value = null;
      }, 2000);
    } catch (err) {
      console.error("Failed to copy:", err);
    }
  }, []);

  const handleClear = useCallback(() => {
    input.value = "";
    output.value = null;
    error.value = null;
  }, []);

  return (
    <div class="generator-container">
      <div class="panel input-panel">
        <div class="panel-header">
          <h2 class="panel-title">
            <span class="terminal-prompt">&gt;</span> INPUT
          </h2>
          <div class="panel-actions">
            <select
              class="escape-select"
              value={escapeStrategy.value}
              onChange={(e) =>
                escapeStrategy.value = (e.target as HTMLSelectElement).value}
            >
              <option value="standard">Standard Escape</option>
              <option value="unicode">Unicode Escape</option>
              <option value="hex">Hexadecimal Escape</option>
              <option value="raw">Raw</option>
            </select>
          </div>
        </div>
        <div class="panel-content">
          <textarea
            class="input-textarea"
            placeholder="Enter your code, text, or any resource here..."
            value={input.value}
            onInput={(e) =>
              input.value = (e.target as HTMLTextAreaElement).value}
            spellcheck={false}
          />
        </div>
        <div class="panel-footer">
          <button
            class="btn btn-secondary"
            onClick={handleClear}
            disabled={isLoading.value}
          >
            CLEAR
          </button>
          <button
            class="btn btn-primary"
            onClick={handleGenerate}
            disabled={isLoading.value || !input.value.trim()}
          >
            {isLoading.value ? "GENERATING..." : "GENERATE QUINE"}
          </button>
        </div>
      </div>

      <div class="panel output-panel">
        <div class="panel-header">
          <h2 class="panel-title">
            <span class="terminal-prompt">&gt;</span> OUTPUT
          </h2>
          <div class="tab-buttons">
            <button
              class={`tab-btn ${
                activeTab.value === "one_line" ? "active" : ""
              }`}
              onClick={() => activeTab.value = "one_line"}
            >
              ONE-LINE
            </button>
            <button
              class={`tab-btn ${
                activeTab.value === "multi_line" ? "active" : ""
              }`}
              onClick={() => activeTab.value = "multi_line"}
            >
              MULTI-LINE
            </button>
          </div>
        </div>

        {error.value && (
          <div class="error-message">
            <span class="error-icon">⚠</span>
            {error.value}
          </div>
        )}

        <div class="panel-content">
          {output.value
            ? (
              <div class="output-display">
                <pre class="output-code">
                {activeTab.value === "one_line"
                  ? output.value.one_line
                  : output.value.multi_line}
                </pre>
              </div>
            )
            : (
              <div class="output-placeholder">
                <p>Generated quine will appear here</p>
              </div>
            )}
        </div>

        {output.value && (
          <div class="panel-footer">
            <div class="stats">
              <span class="stat">
                IN: {output.value.stats.input_bytes}B
              </span>
              <span class="stat">
                OUT: {activeTab.value === "one_line"
                  ? output.value.stats.one_line_bytes
                  : output.value.stats.multi_line_bytes}B
              </span>
              <span class="stat">
                RATIO: {output.value.stats.expansion_ratio.toFixed(2)}x
              </span>
            </div>
            <button
              class="btn btn-copy"
              onClick={() =>
                handleCopy(
                  activeTab.value === "one_line"
                    ? output.value!.one_line
                    : output.value!.multi_line,
                  activeTab.value,
                )}
            >
              {copySuccess.value === activeTab.value ? "COPIED!" : "COPY"}
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
