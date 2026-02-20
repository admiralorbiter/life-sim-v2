// app.js — Main game controller

let currentPhase = 'start'; // start, plan, commit, event, feedback, gameover
let phaseData = null;
let selectedActions = [];
let selectedDecisionIndex = null;
let drawnEvent = null;
let selectedEventOption = null;

// ─── Boot ───────────────────────────────────────────────
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('btn-new-game').addEventListener('click', startNewGame);
});

async function startNewGame() {
    const seedInput = document.getElementById('seed-input');
    const seed = seedInput.value.trim() || null;
    const result = await API.newGame(seed);
    if (result.state) {
        updateStats(result.state);
        currentPhase = 'plan';
        selectedActions = [];
        selectedDecisionIndex = null;
        drawnEvent = null;
        selectedEventOption = null;
        await loadPhase();
    }
}

// ─── Phase Flow ─────────────────────────────────────────
async function loadPhase() {
    phaseData = await API.getPhaseData();

    if (phaseData.isGameOver) {
        currentPhase = 'gameover';
        await renderGameOver();
        return;
    }

    updateStageInfo(phaseData.currentStage, phaseData.currentTurn);

    switch (currentPhase) {
        case 'plan':
            renderPlanPhase();
            break;
        case 'commit':
            renderCommitPhase();
            break;
        case 'event':
            renderEventPhase();
            break;
        case 'feedback':
            // Feedback happens after submit — we won't land here from loadPhase
            break;
    }
}

// ─── Phase 1: Plan ──────────────────────────────────────
function renderPlanPhase() {
    const content = document.getElementById('phase-content');
    const actions = phaseData.actions || [];
    const state = null; // We'll get time slots from the stats bar

    selectedActions = [];

    let html = `
        <div class="phase-card">
            <div class="phase-label">Phase 1: Plan</div>
            <h2>Allocate Your Time</h2>
            <p class="phase-hint">Choose up to 3 actions. Each costs 1 time slot.</p>
            <div class="action-grid" id="action-grid">
    `;

    for (const action of actions) {
        const effectsHtml = action.effects.map(e => {
            const sign = e.delta >= 0 ? '+' : '';
            const icon = statIcon(e.stat);
            return `<span class="effect-tag">${icon} ${sign}${e.delta}</span>`;
        }).join('');

        html += `
            <div class="action-card" data-id="${action.id}" onclick="toggleAction(this, '${action.id}')">
                <div class="action-name">${action.label}</div>
                <div class="action-desc">${action.description}</div>
                <div class="action-effects">${effectsHtml}</div>
            </div>
        `;
    }

    html += `
            </div>
            <div class="selected-count" id="selected-count">0 / 3 selected</div>
            <button class="btn btn-primary" id="btn-plan-next" onclick="finishPlan()" disabled>Continue to Decision →</button>
        </div>
    `;

    content.innerHTML = html;
}

function toggleAction(el, id) {
    if (el.classList.contains('selected')) {
        el.classList.remove('selected');
        selectedActions = selectedActions.filter(a => a !== id);
    } else {
        if (selectedActions.length >= 3) return;
        el.classList.add('selected');
        selectedActions.push(id);
    }
    document.getElementById('selected-count').textContent = `${selectedActions.length} / 3 selected`;
    document.getElementById('btn-plan-next').disabled = selectedActions.length === 0;
}

function finishPlan() {
    currentPhase = 'commit';
    renderCommitPhase();
}

// ─── Phase 2: Commit ────────────────────────────────────
function renderCommitPhase() {
    const content = document.getElementById('phase-content');
    const decision = phaseData.decision;
    selectedDecisionIndex = null;

    if (!decision) {
        // No decision for this turn — skip to event
        currentPhase = 'event';
        renderEventPhase();
        return;
    }

    let html = `
        <div class="phase-card">
            <div class="phase-label">Phase 2: Commit</div>
            <h2>${decision.prompt}</h2>
            <div class="decision-grid" id="decision-grid">
    `;

    decision.options.forEach((opt, i) => {
        const effectsHtml = opt.effects.map(e => {
            const sign = e.delta >= 0 ? '+' : '';
            const icon = statIcon(e.stat);
            return `<span class="effect-tag">${icon} ${sign}${e.delta}</span>`;
        }).join('');

        const tagHtml = opt.grantsTag
            ? `<div class="grants-tag">📚 Grants: ${opt.grantsTag}</div>`
            : '';

        html += `
            <div class="decision-card" data-index="${i}" onclick="selectDecision(this, ${i})">
                <div class="decision-name">${opt.label}</div>
                <div class="decision-desc">${opt.description}</div>
                <div class="decision-effects">${effectsHtml}</div>
                ${tagHtml}
            </div>
        `;
    });

    html += `
            </div>
            <button class="btn btn-primary" id="btn-commit-next" onclick="finishCommit()" disabled>Draw Life Event →</button>
        </div>
    `;

    content.innerHTML = html;
}

function selectDecision(el, index) {
    document.querySelectorAll('.decision-card').forEach(c => c.classList.remove('selected'));
    el.classList.add('selected');
    selectedDecisionIndex = index;
    document.getElementById('btn-commit-next').disabled = false;
}

function finishCommit() {
    currentPhase = 'event';
    renderEventPhase();
}

// ─── Phase 3: Event ─────────────────────────────────────
function renderEventPhase() {
    const content = document.getElementById('phase-content');
    selectedEventOption = null;

    // We don't know the event yet — show a "draw" animation
    let html = `
        <div class="phase-card event-draw">
            <div class="phase-label">Phase 3: Life Event</div>
            <h2>🃏 A life event occurs...</h2>
            <p class="phase-hint">Something happens. How will you respond?</p>
            <button class="btn btn-primary" onclick="submitTurn()">Submit Turn & See Result →</button>
        </div>
    `;

    content.innerHTML = html;
}

// ─── Submit Turn ────────────────────────────────────────
async function submitTurn() {
    const choices = {
        actionIds: selectedActions,
        decisionId: phaseData.decision ? phaseData.decision.id : '',
        decisionOptionIndex: selectedDecisionIndex || 0,
        eventOptionIndex: 0, // Auto-pick first option for now (will enhance later)
    };

    const result = await API.submitTurn(choices);

    if (result.state) {
        updateStats(result.state);
    }

    if (result.isGameOver) {
        currentPhase = 'gameover';
        await renderGameOver();
        return;
    }

    // Show feedback
    renderFeedback(result);
}

// ─── Phase 4: Feedback ─────────────────────────────────
function renderFeedback(result) {
    const content = document.getElementById('phase-content');
    const turnResult = result.turnResult || {};
    const feedback = turnResult.feedback || [];
    const event = turnResult.eventDrawn;

    let html = `<div class="phase-card feedback-card">`;
    html += `<div class="phase-label">Phase 4: Feedback</div>`;

    // Show drawn event card if any
    if (event) {
        html += `
            <div class="event-reveal">
                <h3>🃏 ${event.title}</h3>
                <p class="event-flavor">${event.flavorText}</p>
                <div class="event-choice">Chose: ${event.options[0]?.label || 'N/A'}</div>
            </div>
        `;
    }

    // Show all feedback
    if (feedback.length > 0) {
        html += `<div class="feedback-list">`;
        for (const msg of feedback) {
            const cls = msg.includes('⚠️') ? 'warning' : msg.includes('-') ? 'negative' : 'positive';
            html += `<div class="feedback-item ${cls}">${msg}</div>`;
        }
        html += `</div>`;
    }

    // Stage transition
    if (turnResult.stageTransitioned) {
        html += `<div class="stage-transition">🎓 Stage transition!</div>`;
    }

    html += `
        <button class="btn btn-primary" onclick="nextTurn()">Next Turn →</button>
    </div>`;

    content.innerHTML = html;
}

function nextTurn() {
    currentPhase = 'plan';
    selectedActions = [];
    selectedDecisionIndex = null;
    drawnEvent = null;
    selectedEventOption = null;
    loadPhase();
}

// ─── Game Over ──────────────────────────────────────────
async function renderGameOver() {
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
            <div class="ending-title">${ending.title}</div>
            <p class="ending-narrative">${ending.narrative}</p>
            <div class="ending-reflection">
                <strong>Reflect:</strong> ${ending.reflection}
            </div>
        `;
    }

    html += `
            <div class="final-stats">
                <h3>Final Stats</h3>
                <div class="stat-row">💰 Money: $${state.money}</div>
                <div class="stat-row">😰 Stress: ${state.stress}/100</div>
                <div class="stat-row">🤝 Support: ${state.support}/10</div>
                <div class="stat-row">📚 Credentials: ${state.credentials.length > 0 ? state.credentials.join(', ') : 'None'}</div>
            </div>
            <button class="btn btn-primary" onclick="location.reload()">Play Again</button>
        </div>
    `;

    content.innerHTML = html;
}

// ─── UI Helpers ─────────────────────────────────────────
function updateStats(state) {
    document.getElementById('stat-money').textContent = `$${state.money}`;
    document.getElementById('stat-stress').textContent = `${state.stress}/100`;
    document.getElementById('stat-support').textContent = `${state.support}/10`;
    document.getElementById('stat-time').textContent = `${state.timeSlots} slots`;

    const creds = state.credentials.length > 0
        ? state.credentials.map(c => `<span class="cred-tag">${c}</span>`).join(' ')
        : 'No credentials yet';
    document.getElementById('stat-credentials').innerHTML = creds;

    // Color stress based on danger
    const stressEl = document.getElementById('stat-stress');
    stressEl.parentElement.classList.toggle('danger', state.stress > 75);
}

function updateStageInfo(stage, turn) {
    const stageDisplay = typeof stage === 'string'
        ? stage.replace(/([A-Z])/g, ' $1').trim()
        : stage;
    document.getElementById('stage-label').textContent = `Stage: ${stageDisplay}`;
    document.getElementById('turn-label').textContent = `Turn: ${turn}`;
}

function statIcon(stat) {
    const icons = {
        money: '💰',
        stress: '😰',
        support: '🤝',
        timeSlots: '⏰',
        credentials: '📚',
    };
    return icons[stat] || '•';
}
