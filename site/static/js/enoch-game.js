/**
 * Enochian Chess - Game State Manager
 * Parent Alpine.js component that owns all game state
 */

// ============================================
// Constants
// ============================================

const PIECE_GLYPHS = {
  K: { Blue: '♚', Black: '♚', Red: '♚', Yellow: '♚' },
  Q: { Blue: '♛', Black: '♛', Red: '♛', Yellow: '♛' },
  R: { Blue: '♜', Black: '♜', Red: '♜', Yellow: '♜' },
  B: { Blue: '♝', Black: '♝', Red: '♝', Yellow: '♝' },
  N: { Blue: '♞', Black: '♞', Red: '♞', Yellow: '♞' },
  P: { Blue: '♟', Black: '♟', Red: '♟', Yellow: '♟' },
};

const ARMY_COLORS = {
  B: 'blue',
  K: 'black',
  R: 'red',
  Y: 'yellow',
};

const ARMY_NAMES = {
  B: 'Blue',
  K: 'Black',
  R: 'Red',
  Y: 'Yellow',
};

const ARMY_TEAMS = {
  B: 'Air',
  K: 'Air',
  R: 'Earth',
  Y: 'Earth',
};

// Turn order: Blue -> Black -> Red -> Yellow
const TURN_ORDER = ['B', 'K', 'R', 'Y'];

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

// ============================================
// Utility Functions
// ============================================

function algebraicToIndex(notation) {
  if (!notation || notation.length !== 2) return -1;
  const file = notation.charCodeAt(0) - 97;
  const rank = parseInt(notation[1], 10) - 1;
  if (file < 0 || file > 7 || rank < 0 || rank > 7) return -1;
  return rank * 8 + file;
}

function indexToAlgebraic(index) {
  const file = String.fromCharCode(97 + (index % 8));
  const rank = Math.floor(index / 8) + 1;
  return `${file}${rank}`;
}

function parsePieceCode(code) {
  if (!code || code.length !== 2) return null;
  const armyChar = code[0];
  const pieceChar = code[1];

  const army = ARMY_NAMES[armyChar];
  if (!army) return null;

  const glyph = PIECE_GLYPHS[pieceChar]?.[army];
  if (!glyph) return null;

  return {
    code,
    army: armyChar,
    armyName: army,
    type: pieceChar,
    glyph,
    colorClass: ARMY_COLORS[armyChar],
    team: ARMY_TEAMS[armyChar],
  };
}

function positionToArray(positionObj) {
  const arr = new Array(64).fill(null);
  for (const [square, pieceCode] of Object.entries(positionObj || {})) {
    const idx = algebraicToIndex(square);
    if (idx >= 0) {
      arr[idx] = parsePieceCode(pieceCode);
    }
  }
  return arr;
}

// ============================================
// Game State Component
// ============================================

function enochGame(scenario) {
  return {
    // Scenario config
    scenario: scenario || {},
    mode: scenario?.mode || 'demo',

    // Core game state
    position: [],
    turn: 'B',  // Current army to move
    frozen: { B: false, K: false, R: false, Y: false },
    captures: { Air: [], Earth: [] },

    // UI state
    selected: null,
    legalMoves: [],
    highlightedSquares: [],
    lastMove: null,

    // Step/tutorial state
    step: 0,
    isPlaying: false,
    playInterval: null,

    // Feedback
    feedback: null,
    feedbackMessage: '',
    feedbackTimeout: null,

    // Game status
    status: 'playing', // 'playing', 'check', 'checkmate', 'stalemate', 'draw', 'win'
    winner: null,

    // ========== Computed Properties ==========

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

    get turnName() {
      return ARMY_NAMES[this.turn] || '';
    },

    get turnColor() {
      return ARMY_COLORS[this.turn] || '';
    },

    get turnTeam() {
      return ARMY_TEAMS[this.turn] || '';
    },

    // Backward compatibility: returns turn from step data or computed turn name
    get turnIndicator() {
      return this.currentStepData.turn || this.turnName;
    },

    // Render key - increment to force x-html re-render
    renderKey: 0,

    // ========== Initialization ==========

    init() {
      this.loadInitialState();
      this.applyMovesToStep(this.step);
      this.updateHighlights();

      if (this.mode === 'demo' && this.scenario.autoPlay !== false) {
        this.startAutoPlay();
      }
    },

    // Force re-render by incrementing key
    triggerRender() {
      this.renderKey++;
    },

    loadInitialState() {
      // Load position
      this.position = positionToArray(this.scenario.initialPosition);

      // Load initial state overrides
      const initial = this.scenario.initialState || {};

      // Frozen armies
      if (initial.frozen) {
        for (const army of initial.frozen) {
          const char = army[0].toUpperCase();
          if (this.frozen.hasOwnProperty(char)) {
            this.frozen[char] = true;
          }
        }
      }

      // Captured pieces
      if (initial.captured) {
        this.captures.Air = (initial.captured.Air || []).map(parsePieceCode).filter(Boolean);
        this.captures.Earth = (initial.captured.Earth || []).map(parsePieceCode).filter(Boolean);
      }

      // Starting turn
      if (initial.turn) {
        this.turn = initial.turn[0].toUpperCase();
      }
    },

    // ========== Move Application ==========

    applyMovesToStep(targetStep) {
      for (let i = 0; i <= targetStep; i++) {
        this.applyStepMove(i);
      }
    },

    applyStepMove(stepNum) {
      const stepData = this.scenario.steps?.[stepNum];
      if (stepData?.move) {
        const [from, to] = stepData.move;
        this.executeMove(algebraicToIndex(from), algebraicToIndex(to));
      }
      // Update turn if specified in step
      if (stepData?.turn) {
        this.turn = stepData.turn[0].toUpperCase();
      }
    },

    executeMove(fromIdx, toIdx) {
      if (fromIdx < 0 || toIdx < 0 || !this.position[fromIdx]) return;

      const movingPiece = this.position[fromIdx];
      const capturedPiece = this.position[toIdx];

      // Handle capture
      if (capturedPiece) {
        const capturingTeam = movingPiece.team;
        this.captures[capturingTeam].push(capturedPiece);

        // Check for king capture -> freeze army
        if (capturedPiece.type === 'K') {
          this.frozen[capturedPiece.army] = true;
        }
      }

      // Move piece
      this.position[toIdx] = movingPiece;
      this.position[fromIdx] = null;
      this.lastMove = { from: fromIdx, to: toIdx };

      // Advance turn (skip frozen armies)
      this.advanceTurn();
    },

    advanceTurn() {
      let attempts = 0;
      do {
        const currentIdx = TURN_ORDER.indexOf(this.turn);
        this.turn = TURN_ORDER[(currentIdx + 1) % 4];
        attempts++;
      } while (this.frozen[this.turn] && attempts < 4);
    },

    // ========== Navigation ==========

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
        this.rebuildState();
      }
    },

    goToStep(stepNum) {
      this.step = Math.max(0, Math.min(stepNum, this.maxStep));
      this.rebuildState();
    },

    rebuildState() {
      // Reset to initial state
      this.loadInitialState();
      this.applyMovesToStep(this.step);
      this.updateHighlights();
    },

    updateHighlights() {
      const stepData = this.currentStepData;
      this.highlightedSquares = (stepData.highlight || []).map(algebraicToIndex);

      if (stepData.legalMoves) {
        this.legalMoves = [];
        for (const moves of Object.values(stepData.legalMoves)) {
          this.legalMoves.push(...moves.map(algebraicToIndex));
        }
      } else {
        this.legalMoves = [];
      }
    },

    // ========== Auto-play ==========

    startAutoPlay() {
      if (this.isPlaying) return;
      this.isPlaying = true;
      const speed = this.scenario.playSpeed || 1500;

      this.playInterval = setInterval(() => {
        if (this.step < this.maxStep) {
          this.nextStep();
        } else {
          this.goToStep(0);
        }
      }, speed);
    },

    stopAutoPlay() {
      this.isPlaying = false;
      if (this.playInterval) {
        clearInterval(this.playInterval);
        this.playInterval = null;
      }
    },

    toggleAutoPlay() {
      this.isPlaying ? this.stopAutoPlay() : this.startAutoPlay();
    },

    // ========== Interaction ==========

    selectSquare(index) {
      if (this.mode === 'demo') {
        this.toggleAutoPlay();
        return;
      }

      if (this.mode === 'tutorial') {
        this.handleTutorialClick(index);
      }

      this.triggerRender();
    },

    handleTutorialClick(index) {
      const stepData = this.currentStepData;

      if (this.selected !== null) {
        // Attempt to complete a move
        if (stepData.expectedMove) {
          const [expFrom, expTo] = stepData.expectedMove;
          const fromIdx = algebraicToIndex(expFrom);
          const toIdx = algebraicToIndex(expTo);

          if (this.selected === fromIdx && index === toIdx) {
            this.showFeedback('success', 'Correct!');
            this.executeMove(this.selected, index);
            this.selected = null;
            this.legalMoves = [];
            setTimeout(() => this.nextStep(), 800);
            return;
          } else if (this.legalMoves.includes(index)) {
            this.showFeedback('error', 'Not quite! Try a different move.');
            this.selected = null;
            this.legalMoves = [];
            return;
          }
        }

        // Free exploration move
        if (this.legalMoves.includes(index)) {
          this.executeMove(this.selected, index);
        }

        this.selected = null;
        this.legalMoves = [];
      } else {
        // Select a piece
        if (this.position[index]) {
          this.selected = index;
          const sqName = indexToAlgebraic(index);
          const moves = stepData.legalMoves?.[sqName] || [];
          this.legalMoves = moves.map(algebraicToIndex);
        }
      }
    },

    // ========== Feedback ==========

    showFeedback(type, message) {
      if (this.feedbackTimeout) clearTimeout(this.feedbackTimeout);
      this.feedback = type;
      this.feedbackMessage = message;
      this.feedbackTimeout = setTimeout(() => this.clearFeedback(), 2000);
    },

    clearFeedback() {
      this.feedback = null;
      this.feedbackMessage = '';
      if (this.feedbackTimeout) {
        clearTimeout(this.feedbackTimeout);
        this.feedbackTimeout = null;
      }
    },

    // ========== Render Helpers ==========

    squareClass(index) {
      const classes = ['board-widget__square', `board-widget__square--${SQUARE_COLORS[index]}`];

      if (this.selected === index) classes.push('board-widget__square--selected');
      if (this.highlightedSquares.includes(index)) classes.push('board-widget__square--highlighted');
      if (this.legalMoves.includes(index)) classes.push('board-widget__square--legal');
      if (this.lastMove?.from === index || this.lastMove?.to === index) {
        classes.push('board-widget__square--last-move');
      }

      return classes.join(' ');
    },

    pieceClass(piece) {
      const classes = ['board-widget__piece', `board-widget__piece--${piece.colorClass}`];
      if (this.frozen[piece.army]) {
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

    handleBoardClick(event) {
      const sq = event.target.dataset?.sq;
      if (sq !== undefined) {
        this.selectSquare(parseInt(sq, 10));
      }
    },
  };
}

// ============================================
// Widget Components
// ============================================

function capturedPieces(team) {
  return {
    team,

    get pieces() {
      // Access parent game state
      return this.$data.captures?.[this.team] || [];
    },

    get hasPieces() {
      return this.pieces.length > 0;
    },
  };
}

function turnIndicator() {
  return {
    get armyName() {
      return this.$data.turnName || '';
    },

    get colorClass() {
      return this.$data.turnColor || '';
    },

    get teamName() {
      return this.$data.turnTeam || '';
    },

    get statusText() {
      const status = this.$data.status;
      if (status === 'check') return 'Check!';
      if (status === 'checkmate') return 'Checkmate!';
      if (status === 'stalemate') return 'Stalemate';
      if (status === 'draw') return 'Draw';
      if (status === 'win') return `${this.$data.winner} wins!`;
      return '';
    },
  };
}

function gameControls() {
  return {
    get step() { return this.$data.step; },
    get maxStep() { return this.$data.maxStep; },
    get stepLabel() { return this.$data.stepLabel; },
    get isPlaying() { return this.$data.isPlaying; },
    get hasSteps() { return this.$data.scenario?.steps?.length > 1; },

    prev() { this.$data.prevStep(); },
    next() { this.$data.nextStep(); },
    toggle() { this.$data.toggleAutoPlay(); },
  };
}

// ============================================
// Backward Compatibility
// ============================================

// Keep enochBoard as alias for enochGame
function enochBoard(scenario) {
  return enochGame(scenario);
}

// ============================================
// Register Globally
// ============================================

window.enochGame = enochGame;
window.enochBoard = enochBoard;
window.capturedPieces = capturedPieces;
window.turnIndicator = turnIndicator;
window.gameControls = gameControls;

// Export utilities for other modules
window.EnochUtils = {
  algebraicToIndex,
  indexToAlgebraic,
  parsePieceCode,
  positionToArray,
  PIECE_GLYPHS,
  ARMY_COLORS,
  ARMY_NAMES,
  ARMY_TEAMS,
  SQUARE_COLORS,
};
