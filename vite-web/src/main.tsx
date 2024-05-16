import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import { solve } from "solver";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

console.log(
  solve(
    `
  xxx
  x..
  x..
`,
    ["MOI"]
  )
);
