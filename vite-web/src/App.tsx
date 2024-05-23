import { useState } from "react";
import { solve } from "solver";
import wordlist from "../../rust/words.txt?raw";
import "./App.css";

type CellType = { type: "black" } | { type: "normal"; value: string };

function App() {
  const [grid, setGrid] = useState(() =>
    toGrid(
      `
    ...TEE.xxx
    xxxxxxx.xx
    xxxxxxxx.x
    xxxx.xxxxx
    xxxx.SANAT
  `
    )
  );

  return (
    <>
      <div className="crossword">
        {grid.map((row, y) => (
          <div key={y} className="row">
            {row.map((cell, x) => (
              <div
                key={x}
                className={`cell ${cell.type} ${
                  cell.type === "black" || cell.value === "" ? "clickable" : ""
                }`}
              >
                <input
                  type="text"
                  value={cell.type === "black" ? "." : cell.value}
                  onChange={(e) => {
                    const value = e.target.value.toUpperCase();
                    if (!/[^A-ZÄÅÖ.]?/.test(value)) return;
                    if (value.length === 1) e.target.blur();
                    if (value.length > 1) return;
                    setGrid(
                      changeGridAt(x, y, {
                        type: value === "." ? "black" : "normal",
                        value,
                      })
                    );
                  }}
                />
              </div>
            ))}
          </div>
        ))}
      </div>
      <div>
        <div>
          <button
            onClick={() => {
              setGrid(
                grid.concat([
                  grid[0].map(() => ({ type: "normal", value: "" })),
                ])
              );
            }}
          >
            Lisää rivi
          </button>
          <button onClick={() => setGrid(grid.slice(0, -1))}>
            Poista rivi
          </button>
        </div>
        <div>
          <button
            onClick={() => {
              setGrid(
                grid.map((row) => row.concat([{ type: "normal", value: "" }]))
              );
            }}
          >
            Lisää sarake
          </button>

          <button onClick={() => setGrid(grid.map((row) => row.slice(0, -1)))}>
            Poista sarake
          </button>
        </div>
        <div>
          <button
            className="generate-button"
            onClick={() => {
              const gridText = grid
                .map((row) =>
                  row
                    .map((cell) =>
                      cell.type === "black" ? "." : cell.value || "x"
                    )
                    .join("")
                )
                .join("\n");
              const result = solve(
                gridText,
                wordlist.split("\n").map((w) => w.toUpperCase())
              );
              if (result === "Unsolved") {
                alert("Ristikkoa ei voi ratkaista");
                return;
              }
              setGrid(toGrid(result));
            }}
          >
            Generoi ristikko
          </button>
          <button
            onClick={() => {
              setGrid(
                grid.map((row) =>
                  row.map((cell) =>
                    cell.type === "black"
                      ? { type: "black" }
                      : { type: "normal", value: "" }
                  )
                )
              );
            }}
          >
            Tyhjennä
          </button>
        </div>
      </div>
    </>
  );
}

export default App;

const toGrid = (str: string): CellType[][] =>
  str
    .trim()
    .split("\n")
    .map((row) =>
      row
        .trim()
        .split("")
        .map((cell) =>
          cell === "."
            ? { type: "black" }
            : { type: "normal", value: cell === "x" ? "" : cell }
        )
    );

const changeGridAt =
  (x: number, y: number, value: CellType) => (grid: CellType[][]) => {
    const newGrid = grid.map((row) => row.map((cell) => ({ ...cell })));
    newGrid[y][x] = value;
    return newGrid;
  };
