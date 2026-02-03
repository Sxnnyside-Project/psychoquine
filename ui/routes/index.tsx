import QuineGenerator from "../islands/QuineGenerator.tsx";

export default function Home() {
  return (
    <div class="app-container">
      <header class="app-header">
        <div class="logo-container">
          <h1 class="logo">
            <span class="logo-psycho">PSYCHO</span>
            <span class="logo-quine">QUINE</span>
          </h1>
          <p class="tagline">Universal Quine Generator</p>
        </div>
        <div class="header-info">
          <span class="version">v0.1.0</span>
          <span class="project-tag">CoreRed</span>
        </div>
      </header>

      <main class="main-content">
        <QuineGenerator />
      </main>

      <footer class="app-footer">
        <div class="footer-content">
          <span class="copyright">© 2026 Sxnnyside Project</span>
          <span class="separator">|</span>
          <a
            href="https://github.com/Sxnnyside-Project/psychoquine"
            class="footer-link"
            target="_blank"
            rel="noopener noreferrer"
          >
            GitHub
          </a>
          <span class="separator">|</span>
          <span class="license">MIT License</span>
        </div>
      </footer>
    </div>
  );
}
