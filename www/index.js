import { Universe, Cell } from "anthill";
import { memory } from "anthill/anthill_bg";
const CELL_SIZE = 5; // px  
const GRID_COLOR = "#CCCCCC";
const CELL_COLOR = {
    0: '#EEEEE4', // Empty
    1: 'green',   // Trail
    2: 'red',      // Searched
    3: '#FFD700', // Food
    4: '#8B4513' // Home
};
let universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("anthill-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');

let food_count = 0;

const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#FFFFFF";

const resetButton = document.getElementById("reset")
resetButton.textContent = "Reset Universe"
resetButton.addEventListener("click", event => {
    universe = Universe.new();
});

// Give the canvas room for all of our cells and a 1px border
// around each of them.
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

canvas.addEventListener("click", event => {
    const bounding_rect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / bounding_rect.width;
    const scaleY = canvas.height / bounding_rect.height;

    const canvasLeft = (event.clientX - bounding_rect.left) * scaleX;
    const canvasTop = (event.clientY - bounding_rect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.new_ant(row, col);
});

const getIndex = (row, column) => {
    return row * width + column;
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
    ctx.stroke();
};
// Give the canvas room for all of our cells and a 1px border
// around each of them.
const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = row * width + col;
            ctx.fillStyle = CELL_COLOR[cells[idx]];
            ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
        }
    }
    // Rest of your rendering code...

    const antsCount = universe.ants_count();
    const antsPtr = universe.ants_positions_flat();
    const ants = new Uint32Array(memory.buffer, antsPtr, antsCount * 2);

    for (let i = 0; i < antsCount; i++) {
        const row = ants[i * 2];
        const col = ants[i * 2 + 1];
        ctx.fillStyle = 'black'; // Ant color
        ctx.fillRect(col * (CELL_SIZE + 1) + 1, row * (CELL_SIZE + 1) + 1, CELL_SIZE, CELL_SIZE);
    }
    ctx.stroke();
};



const renderLoop = () => {
    universe.tick();
    drawGrid();
    drawCells();
    requestAnimationFrame(renderLoop);
};

renderLoop();