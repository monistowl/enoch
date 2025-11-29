/**
 * Enochian Chess - Interactive Board Widget
 * Alpine.js component for rendering and interacting with the board
 */

// Piece glyphs by army and type
const PIECE_GLYPHS = {
  K: { Blue: '♚', Black: '♚', Red: '♚', Yellow: '♚' },
  Q: { Blue: '♛', Black: '♛', Red: '♛', Yellow: '♛' },
  R: { Blue: '♜', Black: '♜', Red: '♜', Yellow: '♜' },
  B: { Blue: '♝', Black: '♝', Red: '♝', Yellow: '♝' },
  N: { Blue: '♞', Black: '♞', Red: '♞', Yellow: '♞' },
  P: { Blue: '♟', Black: '♟', Red: '♟', Yellow: '♟' },
};

// Army colors for CSS classes
const ARMY_COLORS = {
  B: 'blue',    // Blue army
  K: 'black',   // Black army
  R: 'red',     // Red army
  Y: 'yellow',  // Yellow army
};

// Square colors for the checkered pattern
const SQUARE_COLORS = [
  'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
  'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
  'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
  'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
  'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
  'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
  'light', 'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark',
  'dark', 'light', 'dark', 'light', 'dark', 'light', 'dark', 'light',
];

/**
 * Convert algebraic notation (e.g., "e4") to square index (0-63)
 */
function algebraicToIndex(notation) {
  if (!notation || notation.length !== 2) return -1;
  const file = notation.charCodeAt(0) - 97; // 'a' = 0
  const rank = parseInt(notation[1], 10) - 1;
  if (file < 0 || file > 7 || rank < 0 || rank > 7) return -1;
  return rank * 8 + file;
}

/**
 * Convert square index to algebraic notation
 */
function indexToAlgebraic(index) {
  const file = String.fromCharCode(97 + (index % 8));
  const rank = Math.floor(index / 8) + 1;
  return `${file}${rank}`;
}

/**
 * Parse piece code (e.g., "BK" = Blue King)
 */
function parsePieceCode(code) {
  if (!code || code.length !== 2) return null;
  const armyChar = code[0]; // B, K, R, Y
  const pieceChar = code[1]; // K, Q, R, B, N, P

  const armyNames = { B: 'Blue', K: 'Black', R: 'Red', Y: 'Yellow' };
  const army = armyNames[armyChar];
  if (!army) return null;

  const glyph = PIECE_GLYPHS[pieceChar]?.[army];
  if (!glyph) return null;

  return {
    army: armyChar,
    type: pieceChar,
    glyph,
    colorClass: ARMY_COLORS[armyChar],
  };
}

/**
 * Convert position object to 64-element array
 */
function positionToArray(positionObj) {
  const arr = new Array(64).fill(null);
  for (const [square, pieceCode] of Object.entries(positionObj)) {
    const idx = algebraicToIndex(square);
    if (idx >= 0) {
      arr[idx] = parsePieceCode(pieceCode);
    }
  }
  return arr;
}

/**
 * Apply a move to a position array (mutates in place)
 */
function applyMove(position, from, to) {
  const fromIdx = typeof from === 'string' ? algebraicToIndex(from) : from;
  const toIdx = typeof to === 'string' ? algebraicToIndex(to) : to;

  if (fromIdx >= 0 && toIdx >= 0 && position[fromIdx]) {
    position[toIdx] = position[fromIdx];
    position[fromIdx] = null;
  }
}

/**
 * Main Alpine.js component for the board
 */
function enochBoard(scenario) {
  return {
    // Scenario data
    scenario: scenario || {},
    mode: scenario?.mode || 'demo',

    // Board state
    position: [],
    selected: null,
    legalMoves: [],
    highlightedSquares: [],
    lastMove: null,

    // Tutorial/demo state
    step: 0,
    isPlaying: false,
    playInterval: null,

    // Computed
    get maxStep() {
      return (this.scenario.steps?.length || 1) - 1;
    },

    get currentStepData() {
      return this.scenario.steps?.[this.step] || {};
    },

    get currentNarrative() {
      return this.currentStepData.narrative || '';
    },

    get stepLabel() {
      if (!this.scenario.steps?.length) return '';
      return `${this.step + 1} / ${this.scenario.steps.length}`;
    },

    get turnIndicator() {
      return this.currentStepData.turn || '';
    },

    // Initialization
    init() {
      // Load initial position
      if (this.scenario.initialPosition) {
        this.position = positionToArray(this.scenario.initialPosition);
      } else {
        this.position = new Array(64).fill(null);
      }

      // Apply moves up to current step
      this.applyMovesToStep(this.step);

      // Update highlights from step data
      this.updateHighlights();

      // Auto-play in demo mode
      if (this.mode === 'demo' && this.scenario.autoPlay !== false) {
        this.startAutoPlay();
      }
    },

    // Navigation
    nextStep() {
      if (this.step < this.maxStep) {
        this.step++;
        this.applyStepMove(this.step);
        this.updateHighlights();
      }
    },

    prevStep() {
      if (this.step > 0) {
        this.step--;
        // Rebuild position from start
        this.position = positionToArray(this.scenario.initialPosition || {});
        this.applyMovesToStep(this.step);
        this.updateHighlights();
      }
    },

    goToStep(stepNum) {
      this.step = Math.max(0, Math.min(stepNum, this.maxStep));
      this.position = positionToArray(this.scenario.initialPosition || {});
      this.applyMovesToStep(this.step);
      this.updateHighlights();
    },

    // Apply moves
    applyMovesToStep(targetStep) {
      for (let i = 0; i <= targetStep; i++) {
        this.applyStepMove(i);
      }
    },

    applyStepMove(stepNum) {
      const stepData = this.scenario.steps?.[stepNum];
      if (stepData?.move) {
        const [from, to] = stepData.move;
        this.lastMove = { from: algebraicToIndex(from), to: algebraicToIndex(to) };
        applyMove(this.position, from, to);
      }
    },

    updateHighlights() {
      const stepData = this.currentStepData;
      this.highlightedSquares = (stepData.highlight || []).map(algebraicToIndex);

      // Update legal moves if specified
      if (stepData.legalMoves) {
        this.legalMoves = [];
        for (const [square, moves] of Object.entries(stepData.legalMoves)) {
          this.legalMoves.push(...moves.map(algebraicToIndex));
        }
      } else {
        this.legalMoves = [];
      }
    },

    // Auto-play for demo mode
    startAutoPlay() {
      if (this.isPlaying) return;
      this.isPlaying = true;

      const playSpeed = this.scenario.playSpeed || 1500;

      this.playInterval = setInterval(() => {
        if (this.step < this.maxStep) {
          this.nextStep();
        } else {
          // Loop back to start
          this.goToStep(0);
        }
      }, playSpeed);
    },

    stopAutoPlay() {
      this.isPlaying = false;
      if (this.playInterval) {
        clearInterval(this.playInterval);
        this.playInterval = null;
      }
    },

    toggleAutoPlay() {
      if (this.isPlaying) {
        this.stopAutoPlay();
      } else {
        this.startAutoPlay();
      }
    },

    // Square interaction
    selectSquare(index) {
      if (this.mode === 'demo') {
        // In demo mode, clicking pauses/resumes
        this.toggleAutoPlay();
        return;
      }

      // Tutorial mode: check if this is an expected move
      if (this.mode === 'tutorial') {
        const stepData = this.currentStepData;

        if (this.selected !== null) {
          // Check if this completes an expected move
          if (stepData.expectedMove) {
            const [expFrom, expTo] = stepData.expectedMove;
            const fromIdx = algebraicToIndex(expFrom);
            const toIdx = algebraicToIndex(expTo);

            if (this.selected === fromIdx && index === toIdx) {
              // Correct move!
              this.nextStep();
              this.selected = null;
              return;
            }
          }

          // Check if this is a legal move destination
          if (this.legalMoves.includes(index)) {
            // Apply the move locally (for free exploration)
            applyMove(this.position, this.selected, index);
            this.lastMove = { from: this.selected, to: index };
          }

          this.selected = null;
          this.legalMoves = [];
        } else {
          // Select a piece
          if (this.position[index]) {
            this.selected = index;
            // Show legal moves if defined for this square
            const sqName = indexToAlgebraic(index);
            const moves = stepData.legalMoves?.[sqName] || [];
            this.legalMoves = moves.map(algebraicToIndex);
          }
        }
      }
    },

    // CSS class helpers
    squareClass(index) {
      const classes = ['board-widget__square', `board-widget__square--${SQUARE_COLORS[index]}`];

      if (this.selected === index) {
        classes.push('board-widget__square--selected');
      }
      if (this.highlightedSquares.includes(index)) {
        classes.push('board-widget__square--highlighted');
      }
      if (this.legalMoves.includes(index)) {
        classes.push('board-widget__square--legal');
      }
      if (this.lastMove?.from === index || this.lastMove?.to === index) {
        classes.push('board-widget__square--last-move');
      }

      return classes.join(' ');
    },

    pieceClass(piece) {
      return `board-widget__piece board-widget__piece--${piece.colorClass}`;
    },

    // Position helpers for SVG
    squareX(index) {
      return (index % 8) * 50;
    },

    squareY(index) {
      return (7 - Math.floor(index / 8)) * 50; // Flip so rank 1 is at bottom
    },

    pieceX(index) {
      return this.squareX(index) + 25; // Center of square
    },

    pieceY(index) {
      return this.squareY(index) + 32; // Slightly below center for text baseline
    },

    // Render methods for SVG (used with x-html)
    renderSquares() {
      let html = '';
      for (let i = 0; i < 64; i++) {
        const x = this.squareX(i);
        const y = this.squareY(i);
        const classes = this.squareClass(i);
        html += `<rect x="${x}" y="${y}" width="50" height="50" class="${classes}" data-sq="${i}" style="cursor:pointer"/>`;
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
      return html;
    },

    renderPieces() {
      let html = '';
      for (let i = 0; i < this.position.length; i++) {
        const piece = this.position[i];
        if (piece) {
          const x = this.pieceX(i);
          const y = this.pieceY(i);
          const cls = this.pieceClass(piece);
          html += `<text x="${x}" y="${y}" class="${cls}" text-anchor="middle">${piece.glyph}</text>`;
        }
      }
      return html;
    },

    // Handle click on board via event delegation
    handleBoardClick(event) {
      const sq = event.target.dataset?.sq;
      if (sq !== undefined) {
        this.selectSquare(parseInt(sq, 10));
      }
    },
  };
}

// Register globally for Alpine
window.enochBoard = enochBoard;
