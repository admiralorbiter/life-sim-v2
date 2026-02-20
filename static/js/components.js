// components.js — UI component renderers

const Components = {
    // ─── Stat Icons ─────────────────────────────────────
    statIcon(stat) {
        const icons = {
            money: '💰', stress: '😰', support: '🤝',
            timeSlots: '⏰', credentials: '📚',
        };
        return icons[stat] || '•';
    },

    // ─── Render Effects Tags ────────────────────────────
    effectTags(effects) {
        return effects.map(e => {
            const sign = e.delta >= 0 ? '+' : '';
            const icon = this.statIcon(e.stat);
            const cls = e.delta >= 0 ? 'effect-positive' : 'effect-negative';
            return `<span class="effect-tag ${cls}">${icon} ${sign}${e.delta}</span>`;
        }).join('');
    },

    // ─── Stats Bar ──────────────────────────────────────
    updateStats(state, prevState) {
        const stats = [
            { id: 'stat-money', val: `$${state.money}`, prev: prevState?.money },
            { id: 'stat-stress', val: `${state.stress}/100`, prev: prevState?.stress },
            { id: 'stat-support', val: `${state.support}/10`, prev: prevState?.support },
            { id: 'stat-time', val: `${state.timeSlots} slots`, prev: prevState?.timeSlots },
        ];

        for (const s of stats) {
            const el = document.getElementById(s.id);
            if (!el) continue;
            const oldText = el.textContent;
            el.textContent = s.val;

            // Animate on change
            if (oldText !== s.val && prevState) {
                const parent = el.parentElement;
                parent.classList.remove('stat-flash-up', 'stat-flash-down');
                void parent.offsetWidth; // reflow
                const curr = parseFloat(s.val.replace(/[^0-9.-]/g, ''));
                const prev = s.prev ?? curr;
                parent.classList.add(curr > prev ? 'stat-flash-up' : 'stat-flash-down');
            }
        }

        // Credentials
        const creds = state.credentials.length > 0
            ? state.credentials.map(c => `<span class="cred-tag">${c}</span>`).join(' ')
            : 'No credentials yet';
        document.getElementById('stat-credentials').innerHTML = creds;

        // Danger states
        const stressEl = document.getElementById('stat-stress');
        if (stressEl) stressEl.parentElement.classList.toggle('danger', state.stress > 75);

        const moneyEl = document.getElementById('stat-money');
        if (moneyEl) moneyEl.parentElement.classList.toggle('danger', state.money <= 0);
    },

    // ─── Stage Info ─────────────────────────────────────
    updateStageInfo(stage, turn) {
        const names = {
            'middle-school': 'Middle School',
            'high-school': 'High School',
            'post-high': 'Post-High Decision',
            'early-adult': 'Early Adult',
        };
        const display = names[stage] || stage;
        document.getElementById('stage-label').textContent = `Stage: ${display}`;
        document.getElementById('turn-label').textContent = `Turn: ${turn}`;
    },

    // ─── Action Card ────────────────────────────────────
    actionCard(action, isSelected) {
        const effectsHtml = this.effectTags(action.effects);
        const timeCost = action.timeCost || 1;
        return `
            <div class="action-card ${isSelected ? 'selected' : ''}" 
                 data-id="${action.id}" 
                 onclick="Game.toggleAction(this, '${action.id}')">
                <div class="card-header">
                    <span class="action-name">${action.label}</span>
                    <span class="time-cost">⏰ ${timeCost}</span>
                </div>
                <div class="action-desc">${action.description}</div>
                <div class="action-effects">${effectsHtml}</div>
            </div>
        `;
    },

    // ─── Decision Option Card ───────────────────────────
    decisionCard(option, index, isSelected) {
        const effectsHtml = this.effectTags(option.effects);
        const tagHtml = option.grantsTag
            ? `<div class="grants-tag">📚 Grants: ${option.grantsTag}</div>`
            : '';
        return `
            <div class="decision-card ${isSelected ? 'selected' : ''}"
                 data-index="${index}"
                 onclick="Game.selectDecision(this, ${index})">
                <div class="decision-name">${option.label}</div>
                <div class="decision-desc">${option.description}</div>
                <div class="decision-effects">${effectsHtml}</div>
                ${tagHtml}
            </div>
        `;
    },

    // ─── Event Option Card ──────────────────────────────
    eventOptionCard(option, index, isSelected) {
        const effectsHtml = this.effectTags(option.effects);
        const requiresSupport = option.requiresSupport
            ? `<div class="requires-support">🤝 Requires Support ≥ ${option.requiresSupport}</div>`
            : '';
        return `
            <div class="event-option-card ${isSelected ? 'selected' : ''}"
                 data-index="${index}"
                 onclick="Game.selectEventOption(this, ${index})">
                <div class="event-option-name">${option.label}</div>
                <div class="event-option-desc">${option.description}</div>
                <div class="event-option-effects">${effectsHtml}</div>
                ${requiresSupport}
            </div>
        `;
    },

    // ─── Feedback Item ──────────────────────────────────
    feedbackItem(msg) {
        let cls = 'neutral';
        if (msg.includes('⚠️')) cls = 'warning';
        else if (msg.includes('-$') || msg.includes('Stress +') || msg.includes('Support -')) cls = 'negative';
        else if (msg.includes('+') || msg.includes('Earned')) cls = 'positive';
        return `<div class="feedback-item ${cls}">${msg}</div>`;
    },

    // ─── Turn Log Entry ─────────────────────────────────
    turnLogEntry(turnNum, stage, feedback) {
        const stageNames = {
            'middle-school': 'MS',
            'high-school': 'HS',
            'post-high': 'PH',
            'early-adult': 'EA',
        };
        const stageName = stageNames[stage] || stage;
        const summary = feedback.slice(0, 3).join(' • ');
        return `
            <div class="log-entry">
                <span class="log-turn">${stageName} T${turnNum}</span>
                <span class="log-summary">${summary}</span>
            </div>
        `;
    },

    // ─── Toast Notification ─────────────────────────────
    showToast(message, type = 'info') {
        const container = document.getElementById('toast-container');
        if (!container) return;
        const toast = document.createElement('div');
        toast.className = `toast toast-${type}`;
        toast.textContent = message;
        container.appendChild(toast);
        requestAnimationFrame(() => toast.classList.add('show'));
        setTimeout(() => {
            toast.classList.remove('show');
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    },
};
