/**
 * Enochian Chess - WASM Integration
 * Connects the Rust engine to Alpine.js game components
 */

// WASM module state
let wasmModule = null;
let WasmGame = null;
let wasmReady = false;
let wasmLoadPromise = null;

/**
 * Initialize the WASM module
 * Returns a promise that resolves when WASM is ready
 */
async function initEnochWasm() {
  if (wasmReady) return true;
  if (wasmLoadPromise) return wasmLoadPromise;

  wasmLoadPromise = (async () => {
    try {
      // Dynamic import of the WASM module
      const wasm = await import('/wasm/enoch.js');
      await wasm.default('/wasm/enoch_bg.wasm');
      WasmGame = wasm.WasmGame;
      wasmModule = wasm;
      wasmReady = true;
      console.log('Enochian Chess WASM engine loaded');
      return true;
    } catch (err) {
      console.error('Failed to load WASM module:', err);
      return false;
    }
  })();

  return wasmLoadPromise;
}

/**
 * WASM-powered game component for Alpine.js
 * Uses the Rust engine for move validation and game logic
 */
function enochWasmGame() {
  return {
    // WASM game instance
    wasmGame: null,
    wasmReady: false,

    // Board state (from WASM)
    position: [],

    // Game state
    turn: 'B',
    frozen: { B: false, K: false, R: false, Y: false },
    captures: { Air: [], Earth: [] },
    status: 'loading',
    inCheck: { B: false, K: false, R: false, Y: false },
    winner: null,

    // UI state
    selected: null,
    legalMoves: [],
    legalCaptures: [],
    lastMove: null,
    renderKey: 0,

    // Feedback
    feedback: null,
    feedbackMessage: '',
    feedbackTimeout: null,

    // Computed
    get turnName() {
      const names = { B: 'Blue', K: 'Black', R: 'Red', Y: 'Yellow' };
      return names[this.turn] || '';
    },

    get turnIndicator() {
      return this.turnName;
    },

    get turnColor() {
      const colors = { B: 'blue', K: 'black', R: 'red', Y: 'yellow' };
      return colors[this.turn] || '';
    },

    // Initialize
    async init() {
      this.status = 'loading';

      const loaded = await initEnochWasm();
      if (!loaded) {
        this.status = 'error';
        this.feedbackMessage = 'Failed to load game engine';
        return;
      }

      // Create new game
      this.wasmGame = new WasmGame();
      this.wasmReady = true;
      this.syncFromWasm();
      this.status = 'playing';
    },

    // Sync state from WASM engine
    syncFromWasm() {
      if (!this.wasmGame) return;

      // Get board state
      const boardState = this.wasmGame.getBoardState();
      this.position = new Array(64).fill(null);

      for (const sq of boardState) {
        if (sq.piece) {
          this.position[sq.index] = {
            code: sq.piece.code,
            army: sq.piece.army[0], // First char: B, K, R, Y
            armyName: sq.piece.army,
            type: sq.piece.kind[0], // First char: K, Q, R, B, N, P
            glyph: sq.piece.glyph,
            colorClass: sq.piece.army.toLowerCase(),
            frozen: sq.piece.frozen,
          };
        }
      }

      // Get game state
      const gameState = this.wasmGame.getGameState();
      this.turn = gameState.current_army[0]; // First char
      this.frozen = {
        B: gameState.frozen.blue,
        K: gameState.frozen.black,
        R: gameState.frozen.red,
        Y: gameState.frozen.yellow,
      };
      this.inCheck = {
        B: gameState.in_check.blue,
        K: gameState.in_check.black,
        R: gameState.in_check.red,
        Y: gameState.in_check.yellow,
      };
      this.status = gameState.status;
      this.winner = gameState.winner;

      this.triggerRender();
    },

    triggerRender() {
      this.renderKey++;
    },

    // Handle board click
    handleBoardClick(event) {
      const sq = event.target.dataset?.sq;
      if (sq !== undefined) {
        this.selectSquare(parseInt(sq, 10));
      }
    },

    selectSquare(index) {
      if (!this.wasmReady || this.winner) return;

      if (this.selected !== null) {
        // Attempt to make a move
        if (this.legalMoves.includes(index) || this.legalCaptures.includes(index)) {
          this.makeMove(this.selected, index);
        }
        this.selected = null;
        this.legalMoves = [];
        this.legalCaptures = [];
      } else {
        // Select a piece
        const piece = this.position[index];
        if (piece && piece.army === this.turn && !piece.frozen) {
          this.selected = index;
          const result = this.wasmGame.getLegalMoves(index);
          this.legalMoves = result.moves || [];
          this.legalCaptures = result.captures || [];
        }
      }
      this.triggerRender();
    },

    makeMove(from, to) {
      const result = this.wasmGame.applyMove(from, to);

      if (result.success) {
        this.lastMove = { from, to };

        // Track capture
        if (result.captured) {
          const team = this.position[from]?.armyName === 'Blue' ||
                       this.position[from]?.armyName === 'Black' ? 'Air' : 'Earth';
          this.captures[team].push({
            code: result.captured.code,
            glyph: result.captured.glyph,
            colorClass: result.captured.army.toLowerCase(),
          });
        }

        this.syncFromWasm();
        this.showFeedback('success', result.message);

        // Check for game end
        if (this.winner) {
          this.showFeedback('success', `${this.winner} wins!`);
        } else if (this.status === 'draw') {
          this.showFeedback('info', 'Game is a draw');
        }
      } else {
        this.showFeedback('error', result.message);
      }
    },

    // Feedback
    showFeedback(type, message) {
      if (this.feedbackTimeout) clearTimeout(this.feedbackTimeout);
      this.feedback = type;
      this.feedbackMessage = message;
      this.feedbackTimeout = setTimeout(() => this.clearFeedback(), 3000);
    },

    clearFeedback() {
      this.feedback = null;
      this.feedbackMessage = '';
    },

    // Reset game
    resetGame() {
      if (!this.wasmReady) return;
      this.wasmGame = new WasmGame();
      this.selected = null;
      this.legalMoves = [];
      this.legalCaptures = [];
      this.lastMove = null;
      this.captures = { Air: [], Earth: [] };
      this.syncFromWasm();
      this.showFeedback('info', 'New game started');
    },

    // Render helpers (same as enochGame)
    squareClass(index) {
      const colors = [
        'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
        'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
        'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
        'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
        'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
        'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
        'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
        'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
      ];
      const classes = ['board-widget__square', `board-widget__square--${colors[index]}`];

      if (this.selected === index) classes.push('board-widget__square--selected');
      if (this.legalMoves.includes(index)) classes.push('board-widget__square--legal');
      if (this.legalCaptures.includes(index)) classes.push('board-widget__square--highlighted');
      if (this.lastMove?.from === index || this.lastMove?.to === index) {
        classes.push('board-widget__square--last-move');
      }

      return classes.join(' ');
    },

    pieceClass(piece) {
      const classes = ['board-widget__piece', `board-widget__piece--${piece.colorClass}`];
      if (piece.frozen) {
        classes.push('board-widget__piece--frozen');
      }
      return classes.join(' ');
    },

    squareX(index) { return (index % 8) * 50; },
    squareY(index) { return (7 - Math.floor(index / 8)) * 50; },
    pieceX(index) { return this.squareX(index) + 25; },
    pieceY(index) { return this.squareY(index) + 32; },

    renderSquares() {
      let html = '';
      for (let i = 0; i < 64; i++) {
        const x = this.squareX(i);
        const y = this.squareY(i);
        html += `<rect x="${x}" y="${y}" width="50" height="50" class="${this.squareClass(i)}" data-sq="${i}" style="cursor:pointer"/>`;
      }
      return html;
    },

    renderLegalMoves() {
      let html = '';
      for (const sq of this.legalMoves) {
        const cx = this.pieceX(sq);
        const cy = this.squareY(sq) + 25;
        html += `<circle cx="${cx}" cy="${cy}" r="8" class="board-widget__legal-indicator"/>`;
      }
      for (const sq of this.legalCaptures) {
        const cx = this.pieceX(sq);
        const cy = this.squareY(sq) + 25;
        html += `<circle cx="${cx}" cy="${cy}" r="12" class="attack-indicator"/>`;
      }
      return html;
    },

    renderPieces() {
      let html = '';
      for (let i = 0; i < this.position.length; i++) {
        const piece = this.position[i];
        if (piece) {
          const x = this.pieceX(i);
          const y = this.pieceY(i);
          html += `<text x="${x}" y="${y}" class="${this.pieceClass(piece)}" text-anchor="middle">${piece.glyph}</text>`;
        }
      }
      return html;
    },
  };
}

// Register globally
window.initEnochWasm = initEnochWasm;
window.enochWasmGame = enochWasmGame;
