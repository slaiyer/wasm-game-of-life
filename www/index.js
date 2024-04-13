/**
 * This script file controls the behavior of the game of life simulation.
 * It imports the `Universe` module from the `wasm-game-of-life` package and sets up the necessary variables and event listeners.
 * The simulation is rendered on a canvas element with the specified cell size and colors.
 * The script provides functions for playing, pausing, and resetting the simulation, as well as handling user interactions such as clicking on cells and deploying predefined patterns.
 * It also includes utility functions for drawing the grid and cells on the canvas.
 */
import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

const CELL_SIZE = 3; // px
const GRID_COLOR = "#000000";
const DEAD_COLOR = "#000000";
const ALIVE_COLOR = "#FFFFFF";

let ticksPerFrame = document.getElementById("ticksPerFrame");
ticksPerFrame.addEventListener("change", function () {
    localStorage.setItem("ticksPerFrame", ticksPerFrame.value);
})

let chanceOfLife = document.getElementById("chance");
chanceOfLife.addEventListener("change", function () {
    localStorage.setItem("chanceOfLife", chanceOfLife.value);
    chanceOfLife.value = (+chanceOfLife.value).toFixed(1);
    window.location.reload();
})

window.onload = function () {
    pause();
    chanceOfLife.value = localStorage.getItem("chanceOfLife");
    chanceOfLife.value = (+chanceOfLife.value).toFixed(1);
    ticksPerFrame.value = localStorage.getItem("ticksPerFrame");
}

console.log("Chance of life: ", +chanceOfLife.value);

// Construct the universe, and get its width and height.
const universe = Universe.new(+chanceOfLife.value / 100);
const width = universe.width();
const height = universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;

function renderLoop() {
    // debugger;

    for (let i = 0; i < +ticksPerFrame.value; ++i) {
        universe.tick();
    }

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
}

function isPaused() {
    return animationId === null;
}

const playPauseButton = document.getElementById("play-pause");

function play() {
    playPauseButton.textContent = "⏸";
    renderLoop();
}

function pause() {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
}

playPauseButton.addEventListener("click", function (event) {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

window.addEventListener(
    "keypress",
    function (event) {
        if (event.defaultPrevented) {
            return; // Do nothing if the event was already processed
        }

        switch (event.key) {
            case " ":
                playPauseButton.click();
                break;
            case "R":
                window.location.reload();
                break;
            case "C":
                pause();
                universe.clear();
                drawGrid();
                drawCells();
                break;
            default:
                return; // Quit when this doesn't handle the key event.
        }

        // Cancel the default action to avoid it being handled twice
        event.preventDefault();
    },
    true,
);

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

function getIndex(row, column) {
    return row * width + column;
}

function bitIsSet(n, arr) {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
}

function drawCells() {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = bitIsSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

canvas.addEventListener('contextmenu', function (event) {
    event.preventDefault();
});

canvas.addEventListener(
    "mouseup",
    function (event) {
        if (event.defaultPrevented) {
            return; // Do nothing if the event was already processed
        }

        const boundingRect = canvas.getBoundingClientRect();

        const scaleX = canvas.width / boundingRect.width;
        const scaleY = canvas.height / boundingRect.height;

        const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
        const canvasTop = (event.clientY - boundingRect.top) * scaleY;

        const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
        const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

        if (event.ctrlKey) {
            universe.deploy('glider', row, col);
        } else if (event.shiftKey) {
            universe.deploy('pulsar', row, col);
        } else {
            universe.toggle_cell(row, col);
        }

        drawGrid();
        drawCells();

        event.preventDefault();
    },
    true,
);

pause();
drawGrid();
drawCells();
