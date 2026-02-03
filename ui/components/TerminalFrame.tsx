import { ComponentChildren } from "preact";

interface TerminalFrameProps {
  title: string;
  children: ComponentChildren;
}

export default function TerminalFrame({ title, children }: TerminalFrameProps) {
  return (
    <div class="terminal-frame">
      <div class="terminal-titlebar">
        <div class="terminal-buttons">
          <span class="terminal-button close"></span>
          <span class="terminal-button minimize"></span>
          <span class="terminal-button maximize"></span>
        </div>
        <span class="terminal-title">{title}</span>
        <div class="terminal-spacer"></div>
      </div>
      <div class="terminal-content">
        {children}
      </div>
    </div>
  );
}
