// app.js — Main game controller (state machine)

const Game = {
    phase: 'start',
    phaseData: null,
    selectedActions: [],
    selectedDecisionIndex: null,
    selectedEventOption: null,
    turnLog: [],
    prevState: null,
    currentState: null,

    // ─── Boot ───────────────────────────────────────────
    init() {
        document.getElementById('btn-new-game').addEventListener('click', () => Game.startNewGame());
    },

    async startNewGame() {
        const seedInput = document.getElementById('seed-input');
        const seed = seedInput.value.trim() || null;

        const result = await API.newGame(seed);
        if (result.state) {
            this.currentState = result.state;
            this.prevState = null;
            this.turnLog = [];
            Components.updateStats(result.state, null);
            this.renderTurnLog();
            this.phase = 'plan';
            this.resetSelections();
            await this.loadPhase();
            Components.showToast(`Game started! Seed: ${result.state.seed}`, 'info');
        }
    },

    resetSelections() {
        this.selectedActions = [];
        this.selectedDecisionIndex = null;
        this.selectedEventOption = null;
    },

    // ─── Phase Flow ─────────────────────────────────────
    async loadPhase() {
        this.phaseData = await API.getPhaseData();

        if (this.phaseData.isGameOver) {
            this.phase = 'gameover';
            this.renderGameOver();
            return;
        }

        Components.updateStageInfo(this.phaseData.currentStage, this.phaseData.currentTurn);

        switch (this.phase) {
            case 'plan': this.renderPlanPhase(); break;
            case 'commit': this.renderCommitPhase(); break;
            case 'event': this.renderEventPhase(); break;
        }
    },

    // ─── Phase 1: Plan ──────────────────────────────────
    renderPlanPhase() {
        const content = document.getElementById('phase-content');
        const actions = this.phaseData.actions || [];
        this.selectedActions = [];

        const timeSlots = this.currentState?.timeSlots || 3;

        let html = `
            <div class="phase-card">
                <div class="phase-label">Phase 1 of 4</div>
                <h2>📋 Allocate Your Time</h2>
                <p class="phase-hint">
                    Choose actions to spend your <strong>${timeSlots} time slots</strong> on.
                </p>
                <div class="time-budget" id="time-budget">
                    ${this.renderTimeSlots(0, timeSlots)}
                </div>
                <div class="action-grid" id="action-grid">
                    ${actions.map(a => Components.actionCard(a, false)).join('')}
                </div>
                <div class="phase-nav">
                    <div class="selected-count" id="selected-count">0 / ${timeSlots} time used</div>
                    <button class="btn btn-primary" id="btn-plan-next" onclick="Game.finishPlan()" disabled>
                        Continue to Decision →
                    </button>
                </div>
            </div>
        `;
        content.innerHTML = html;
    },

    renderTimeSlots(used, total) {
        let html = '';
        for (let i = 0; i < total; i++) {
            html += `<span class="time-pip ${i < used ? 'filled' : ''}"></span>`;
        }
        return html;
    },

    toggleAction(el, id) {
        const timeSlots = this.currentState?.timeSlots || 3;
        const action = this.phaseData.actions.find(a => a.id === id);
        const cost = action?.timeCost || 1;

        if (el.classList.contains('selected')) {
            el.classList.remove('selected');
            this.selectedActions = this.selectedActions.filter(a => a !== id);
        } else {
            // Check if we have enough time slots
            const currentCost = this.selectedActions.reduce((sum, aid) => {
                const a = this.phaseData.actions.find(x => x.id === aid);
                return sum + (a?.timeCost || 1);
            }, 0);
            if (currentCost + cost > timeSlots) {
                Components.showToast('Not enough time slots!', 'warning');
                return;
            }
            el.classList.add('selected');
            this.selectedActions.push(id);
        }

        const totalCost = this.selectedActions.reduce((sum, aid) => {
            const a = this.phaseData.actions.find(x => x.id === aid);
            return sum + (a?.timeCost || 1);
        }, 0);

        document.getElementById('selected-count').textContent = `${totalCost} / ${timeSlots} time used`;
        document.getElementById('time-budget').innerHTML = this.renderTimeSlots(totalCost, timeSlots);
        document.getElementById('btn-plan-next').disabled = this.selectedActions.length === 0;
    },

    finishPlan() {
        this.phase = 'commit';
        this.renderCommitPhase();
    },

    // ─── Phase 2: Commit ────────────────────────────────
    renderCommitPhase() {
        const content = document.getElementById('phase-content');
        const decision = this.phaseData.decision;
        this.selectedDecisionIndex = null;

        if (!decision) {
            this.phase = 'event';
            this.renderEventPhase();
            return;
        }

        let html = `
            <div class="phase-card">
                <div class="phase-label">Phase 2 of 4</div>
                <h2>🤔 ${decision.prompt}</h2>
                <p class="phase-hint">This decision will shape your future. Choose wisely.</p>
                <div class="decision-grid" id="decision-grid">
                    ${decision.options.map((opt, i) => Components.decisionCard(opt, i, false)).join('')}
                </div>
                <div class="phase-nav">
                    <button class="btn btn-secondary" onclick="Game.backToPlan()">← Back to Plan</button>
                    <button class="btn btn-primary" id="btn-commit-next" onclick="Game.finishCommit()" disabled>
                        Draw Life Event →
                    </button>
                </div>
            </div>
        `;
        content.innerHTML = html;
    },

    selectDecision(el, index) {
        document.querySelectorAll('.decision-card').forEach(c => c.classList.remove('selected'));
        el.classList.add('selected');
        this.selectedDecisionIndex = index;
        document.getElementById('btn-commit-next').disabled = false;
    },

    backToPlan() {
        this.phase = 'plan';
        this.renderPlanPhase();
    },

    finishCommit() {
        this.phase = 'event';
        this.renderEventPhase();
    },

    // ─── Phase 3: Event ─────────────────────────────────
    renderEventPhase() {
        const content = document.getElementById('phase-content');
        this.selectedEventOption = null;

        // Show draw animation first
        let html = `
            <div class="phase-card event-draw-card">
                <div class="phase-label">Phase 3 of 4</div>
                <h2>🃏 Life Event</h2>
                <p class="phase-hint">A random life event is about to happen...</p>
                <div class="card-draw-area">
                    <div class="draw-card-back" id="draw-card-back" onclick="Game.revealEvent()">
                        <div class="card-back-design">
                            <span>🎴</span>
                            <span class="draw-text">Click to Draw</span>
                        </div>
                    </div>
                </div>
            </div>
        `;
        content.innerHTML = html;
    },

    async revealEvent() {
        // Draw event from the server (preview, no state change yet)
        const drawResult = await API.drawEvent();
        const event = drawResult.event;

        if (event && event.options && event.options.length > 0) {
            this.drawnEvent = event;
            this.renderEventOptions(event);
        } else {
            // No event available — submit turn immediately
            await this.submitFinalTurn(0);
        }
    },

    renderEventOptions(event) {
        const content = document.getElementById('phase-content');
        this.selectedEventOption = null;

        let html = `
            <div class="phase-card event-reveal-card">
                <div class="phase-label">Phase 3 of 4 — Life Event</div>
                <div class="event-card-display">
                    <div class="event-rarity rarity-${event.rarity || 'common'}">${(event.rarity || 'common').toUpperCase()}</div>
                    <h2>🃏 ${event.title}</h2>
                    <p class="event-flavor">${event.flavorText}</p>
                </div>
                <h3>How do you respond?</h3>
                <div class="event-options-grid">
                    ${event.options.map((opt, i) => Components.eventOptionCard(opt, i, false)).join('')}
                </div>
                <div class="phase-nav">
                    <button class="btn btn-primary" id="btn-event-submit" onclick="Game.submitEventChoice()" disabled>
                        See Results →
                    </button>
                </div>
            </div>
        `;
        content.innerHTML = html;
    },

    selectEventOption(el, index) {
        document.querySelectorAll('.event-option-card').forEach(c => c.classList.remove('selected'));
        el.classList.add('selected');
        this.selectedEventOption = index;
        document.getElementById('btn-event-submit').disabled = false;
    },

    async submitEventChoice() {
        await this.submitFinalTurn(this.selectedEventOption ?? 0);
    },

    async submitFinalTurn(eventOptionIdx) {
        const choices = {
            actionIds: this.selectedActions,
            decisionId: this.phaseData.decision ? this.phaseData.decision.id : '',
            decisionOptionIndex: this.selectedDecisionIndex ?? 0,
            eventOptionIndex: eventOptionIdx,
        };

        const result = await API.submitTurn(choices);

        if (result.state) {
            this.prevState = this.currentState;
            this.currentState = result.state;
            Components.updateStats(result.state, this.prevState);
        }

        if (result.isGameOver) {
            this.phase = 'gameover';
            this.renderGameOver();
            return;
        }

        this.renderFeedback(result);
    },

    // ─── Phase 4: Feedback ──────────────────────────────
    renderFeedback(result) {
        const content = document.getElementById('phase-content');
        const turnResult = result.turnResult || {};
        const feedback = turnResult.feedback || [];
        const event = turnResult.eventDrawn;
        const state = result.state;

        // Log this turn
        const turnNum = (state?.currentTurn || 1) - 1; // turn already advanced
        this.turnLog.push({
            turn: turnNum,
            stage: state?.currentStage || '',
            feedback: feedback,
        });
        this.renderTurnLog();

        let html = `<div class="phase-card feedback-card">`;
        html += `<div class="phase-label">Phase 4 of 4 — Results</div>`;
        html += `<h2>📊 Turn ${turnNum} Complete</h2>`;

        // Show event card summary if any
        if (event) {
            const chosenIdx = this.selectedEventOption ?? 0;
            const chosenLabel = event.options[chosenIdx]?.label || 'N/A';
            html += `
                <div class="event-summary">
                    <span class="event-summary-title">🃏 ${event.title}</span>
                    <span class="event-summary-choice">→ ${chosenLabel}</span>
                </div>
            `;
        }

        // Feedback list with staggered animation
        if (feedback.length > 0) {
            html += `<div class="feedback-list">`;
            feedback.forEach((msg, i) => {
                html += `<div class="feedback-item-wrapper" style="animation-delay: ${i * 0.1}s">
                    ${Components.feedbackItem(msg)}
                </div>`;
            });
            html += `</div>`;
        }

        // Stage transition banner
        if (turnResult.stageTransitioned) {
            const stageNames = {
                'middle-school': 'Middle School', 'high-school': 'High School',
                'post-high': 'Post-High Decision', 'early-adult': 'Early Adult',
            };
            const newStage = stageNames[state?.currentStage] || state?.currentStage;
            html += `
                <div class="stage-transition">
                    🎓 Stage Complete! Advancing to <strong>${newStage}</strong>
                </div>
            `;
        }

        // Warnings
        if (turnResult.stressWarning) {
            html += `<div class="turn-warning">⚠️ ${turnResult.stressWarning}</div>`;
        }

        html += `
            <div class="phase-nav">
                <button class="btn btn-primary" onclick="Game.nextTurn()">Next Turn →</button>
            </div>
        </div>`;

        content.innerHTML = html;
    },

    nextTurn() {
        this.phase = 'plan';
        this.resetSelections();
        this.loadPhase();
    },

    // ─── Game Over ──────────────────────────────────────
    async renderGameOver() {
        const content = document.getElementById('phase-content');
        const endingData = await API.getEnding();
        const ending = endingData.ending;
        const state = endingData.state;

        let html = `
            <div class="phase-card gameover-card">
                <h2>🎬 Game Over</h2>
        `;

        if (ending) {
            html += `
                <div class="ending-badge">${ending.title}</div>
                <p class="ending-narrative">${ending.narrative}</p>
                <div class="ending-reflection">
                    <div class="reflection-header">💭 Reflection</div>
                    ${ending.reflection}
                </div>
            `;
        } else {
            html += `<p class="ending-narrative">Your journey has ended. Every choice led you here.</p>`;
        }

        html += `
                <div class="final-stats">
                    <h3>Final Stats</h3>
                    <div class="final-stats-grid">
                        <div class="final-stat">
                            <div class="final-stat-val">$${state.money}</div>
                            <div class="final-stat-label">Money</div>
                        </div>
                        <div class="final-stat">
                            <div class="final-stat-val">${state.stress}</div>
                            <div class="final-stat-label">Stress</div>
                        </div>
                        <div class="final-stat">
                            <div class="final-stat-val">${state.support}</div>
                            <div class="final-stat-label">Support</div>
                        </div>
                        <div class="final-stat">
                            <div class="final-stat-val">${state.credentials.length}</div>
                            <div class="final-stat-label">Credentials</div>
                        </div>
                    </div>
                    ${state.credentials.length > 0 ?
                `<div class="final-creds">${state.credentials.map(c => `<span class="cred-tag">${c}</span>`).join(' ')}</div>`
                : ''}
                </div>

                <div class="turn-timeline">
                    <h3>📜 Your Journey</h3>
                    ${this.turnLog.map(entry => Components.turnLogEntry(entry.turn, entry.stage, entry.feedback)).join('')}
                </div>

                <button class="btn btn-primary" onclick="location.reload()">🔄 Play Again</button>
            </div>
        `;

        content.innerHTML = html;
    },

    // ─── Turn Log Sidebar ───────────────────────────────
    renderTurnLog() {
        const logEl = document.getElementById('turn-log');
        if (!logEl) return;

        if (this.turnLog.length === 0) {
            logEl.innerHTML = '<div class="log-empty">No turns yet</div>';
            return;
        }

        logEl.innerHTML = this.turnLog.map(entry =>
            Components.turnLogEntry(entry.turn, entry.stage, entry.feedback)
        ).join('');

        logEl.scrollTop = logEl.scrollHeight;
    },
};

// Boot
document.addEventListener('DOMContentLoaded', () => Game.init());
