import { FC, MouseEventHandler, useEffect, useRef, useState, MouseEvent } from 'react';
import { Cell, memory, Universe } from '@nx-rs/lib-rs';

const CELL_SIZE_PX = 5;

const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#000000";
const ALIVE_COLOR = "#FFFFFF";
const HEIGHT = 100;
const WIDTH = 200;


const GameOfLife: FC = () => {
  const universe = useRef(Universe.new(WIDTH, HEIGHT)) ;
  const width = universe.current.width();
  const height = universe.current.height();
  const canvasRef = useRef(null);
  const numFramesToWait = useRef(2);
  const [animationId, setAnimationId] = useState<number | null>(null);

  const drawGrid = (ctx: CanvasRenderingContext2D) => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE_PX + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE_PX + 1) + 1, (CELL_SIZE_PX + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE_PX + 1) + 1);
      ctx.lineTo((CELL_SIZE_PX + 1) * width + 1, j * (CELL_SIZE_PX + 1) + 1);
    }

    ctx.stroke();
  };

  const getIndex = (row: number, column: number) =>  {
    return row * width + column;
  };

  const bitIsSet = (n: number, arr: Uint8Array) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
  };

  const drawCells = (ctx: CanvasRenderingContext2D) => {
    const cellsPtr = universe.current.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);
    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        const idx = getIndex(row, col);

        ctx.fillStyle = bitIsSet(idx, cells) ? DEAD_COLOR : ALIVE_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE_PX + 1) + 1,
          row * (CELL_SIZE_PX + 1) + 1,
          CELL_SIZE_PX,
          CELL_SIZE_PX
        );
      }
    }

    ctx.stroke();
  };

  const isPaused = () => {
    return animationId === null;
  };

  const renderLoop = () => {
    const canvas: any = canvasRef.current;
    const context: CanvasRenderingContext2D = canvas.getContext('2d');
    let frameCount = 0;
    let animationFrameId: any;

    const draw = (ctx: CanvasRenderingContext2D, frameCount: number) => {
      if(frameCount % numFramesToWait.current === 0) {
        universe.current.tick();
      }
      drawGrid(ctx);
      drawCells(ctx);
    };

    frameCount++;
    draw(context, frameCount);

    const drawFrame = () => {
      frameCount++;
      draw(context, frameCount);
      animationFrameId = window.requestAnimationFrame(drawFrame);
      setAnimationId(animationFrameId);
    };

    drawFrame();
  }

  useEffect(() => {
    const canvas: any = canvasRef.current;
    const ctx: CanvasRenderingContext2D = canvas.getContext('2d');
    drawGrid(ctx);
    drawCells(ctx);
  });

  const play = () => {
    renderLoop();
  }

  const reset = () => {
    const canvas: any = canvasRef.current;
    const ctx: CanvasRenderingContext2D = canvas.getContext('2d');
    universe.current.reset();
    drawGrid(ctx);
    drawCells(ctx);
  }


  const random = () => {
    const canvas: any = canvasRef.current;
    const ctx: CanvasRenderingContext2D = canvas.getContext('2d');
    universe.current.random(0.1);
    drawGrid(ctx);
    drawCells(ctx);
  }


  const pause = () => {
    if(animationId) {
      cancelAnimationFrame(animationId);
      setAnimationId(null);
      const canvas: any = canvasRef.current;
      const ctx: CanvasRenderingContext2D = canvas.getContext('2d');
      drawGrid(ctx);
      drawCells(ctx);
    }
  }

  const getRowColumn = (event: MouseEvent<HTMLCanvasElement>) => {
    const canvas: any = canvasRef.current;
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE_PX + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE_PX + 1)), width - 1);
    return [row, col];
  }

  const handleToggleClick: MouseEventHandler<HTMLCanvasElement> = (event) => {
    const canvas: any = canvasRef.current;
    const ctx: CanvasRenderingContext2D = canvas.getContext('2d');
    const [row, col] = getRowColumn(event);
    universe.current.toggle_cell(row, col);
    drawGrid(ctx);
    drawCells(ctx);
  }

  const handleGliderClick: MouseEventHandler<HTMLCanvasElement> = (event) => {
    const [row, col] = getRowColumn(event);
    const canvas: any = canvasRef.current;
    const ctx: CanvasRenderingContext2D = canvas.getContext('2d');
    universe.current.glider(row, col);
    drawGrid(ctx);
    drawCells(ctx);
  }

  const handlePulsarClick: MouseEventHandler<HTMLCanvasElement> = (event) => {
    const [row, col] = getRowColumn(event);
    const canvas: any = canvasRef.current;
    const ctx: CanvasRenderingContext2D = canvas.getContext('2d');
    universe.current.pulsar(row, col);
    drawGrid(ctx);
    drawCells(ctx);
  }


  const handleCanvasClick: MouseEventHandler<HTMLCanvasElement> = (event) => {
    if(event.ctrlKey) {
      handleGliderClick(event);
    } else if(event.shiftKey) {
      handlePulsarClick(event);
    } else {
      handleToggleClick(event);
    }
  }

  return (
    <div style={{ padding: '10px', width: '100%' }}>
      <h4 data-test="game-of-life-title">Game of Life</h4>
      <div style={{display: 'flex', flexDirection: 'row'}}>
        <div>
          Click on cell to toggle. Ctrl + Click to insert <a href={"https://conwaylife.com/wiki/Glider"}>Glider</a>,
          Shift + click to insert <a href={"https://conwaylife.com/wiki/Pulsar"}>Pulsar</a>
        </div>
        <div style={{flexGrow: 1}}/>
        <a href={"https://github.com/JakubSokolowski/nx-rs"}>
          source code
        </a>
      </div>
      <div style={{height: '10px'}}/>
      <div style={{display: 'flex', flexDirection: 'row'}}>
        <button
          onClick={
            () => {
              if(isPaused()) {
                play();
              } else {
                pause();
              }
            }
          }
        >
          {isPaused() ? "Start" : "Stop"}
        </button>
        <div style={{ width: '10px' }}/>
        <button onClick={() => reset()}>
          {"Reset"}
        </button>
        <div style={{ width: '10px' }}/>
        <button onClick={() => random()}>
          {"Random"}
        </button>
        <div style={{ width: '10px' }}/>
        <input
          type={'range'}
          min={2}
          max={10}
          value={numFramesToWait.current}
          onChange={(event) => numFramesToWait.current = Number.parseInt(event.target.value)}
        />
      </div>
      <div style={{ height: '10px' }} />
      <canvas
        onClick={handleCanvasClick}
        style={{ width: '100%', height: '100%' }}
        width={(CELL_SIZE_PX + 1) * width + 1}
        height={(CELL_SIZE_PX + 1) * height + 1}
        ref={canvasRef}
      />
    </div>
  );
};

export default GameOfLife;
