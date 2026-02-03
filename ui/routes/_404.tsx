export default function NotFound() {
  return (
    <div class="error-container">
      <div class="error-content">
        <h1 class="error-code">404</h1>
        <p class="error-message">RESOURCE NOT FOUND</p>
        <a href="/" class="error-link">
          {">"} RETURN TO TERMINAL
        </a>
      </div>
    </div>
  );
}
