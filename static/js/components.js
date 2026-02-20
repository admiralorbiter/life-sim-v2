// components.js — UI component renderers

const Components = {
    // ─── Stat Icons ─────────────────────────────────────
    statIcon(stat) {
        const icons = {
            money: '💰', stress: '😰', support: '🤝',
            timeSlots: '⏰', credentials: '📚',
            bills: '🏠', emergencyFund: '🏦',
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

        // Bills & Emergency Fund (visible in EarlyAdult, or when set)
        const billsRow = document.getElementById('stat-bills-row');
        const efundRow = document.getElementById('stat-efund-row');

        if (billsRow) {
            if (state.monthlyBills > 0 || state.currentStage === 'early-adult') {
                billsRow.style.display = '';
                document.getElementById('stat-bills').textContent = `$${state.monthlyBills}/turn`;
            } else {
                billsRow.style.display = 'none';
            }
        }
        if (efundRow) {
            if (state.emergencyFund > 0 || state.currentStage === 'early-adult') {
                efundRow.style.display = '';
                document.getElementById('stat-efund').textContent = `$${state.emergencyFund}`;
            } else {
                efundRow.style.display = 'none';
            }
        }

        // Job display
        const jobRow = document.getElementById('stat-job-row');
        if (jobRow) {
            if (state.currentJob) {
                jobRow.style.display = '';
                document.getElementById('stat-job').textContent = state.currentJob.title;
            } else {
                jobRow.style.display = 'none';
            }
        }

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
            'post-high': 'Post-High',
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
        const specialLabel = action.specialEffect
            ? `<div class="special-tag">✨ ${action.specialEffect === 'emergency_fund_deposit' ? 'Adds to Emergency Fund' : 'Reduces Bills'}</div>`
            : '';
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
                ${specialLabel}
            </div>
        `;
    },

    // ─── Decision Option Card ───────────────────────────
    decisionCard(option, index, isSelected, playerCredentials = []) {
        const effectsHtml = this.effectTags(option.effects);
        const tagHtml = option.grantsTag
            ? `<div class="grants-tag">📚 Grants: ${option.grantsTag}</div>`
            : '';

        // Check credential gate
        const locked = option.requiresTag && !playerCredentials.includes(option.requiresTag);
        const lockHtml = option.requiresTag
            ? `<div class="requires-tag ${locked ? 'locked' : 'unlocked'}">🔑 Requires: ${option.requiresTag}</div>`
            : '';

        const jobHtml = option.setsJob
            ? `<div class="sets-job">💼 Assigns job</div>`
            : '';
        const billsHtml = option.setBills !== undefined && option.setBills !== null
            ? `<div class="sets-bills">🏠 Bills: $${option.setBills}/turn</div>`
            : '';

        return `
            <div class="decision-card ${isSelected ? 'selected' : ''} ${locked ? 'locked' : ''}"
                 data-index="${index}"
                 onclick="${locked ? '' : `Game.selectDecision(this, ${index})`}">
                <div class="decision-name">${option.label}</div>
                <div class="decision-desc">${option.description}</div>
                <div class="decision-effects">${effectsHtml}</div>
                ${tagHtml}${lockHtml}${jobHtml}${billsHtml}
                ${locked ? '<div class="lock-overlay">🔒 Credential Required</div>' : ''}
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

    // ─── Stage Transition Screen ────────────────────────
    stageTransitionScreen(oldStage, newStage, state) {
        const names = {
            'middle-school': 'Middle School',
            'high-school': 'High School',
            'post-high': 'Post-High',
            'early-adult': 'Early Adult',
        };
        const themes = {
            'middle-school': { emoji: '📚', color: '#4fc3f7' },
            'high-school': { emoji: '🎓', color: '#ab47bc' },
            'post-high': { emoji: '🚀', color: '#66bb6a' },
            'early-adult': { emoji: '💼', color: '#ffa726' },
        };

        const oldName = names[oldStage] || oldStage;
        const newName = names[newStage] || newStage;
        const newTheme = themes[newStage] || { emoji: '🌟', color: '#fff' };

        const stageHints = {
            'high-school': 'New friends, harder choices. Part-time work is now available. Decisions matter more.',
            'post-high': 'The biggest fork in the road. Choose your path: college, trade school, or straight to work. Where will you live?',
            'early-adult': 'Bills are real now. Jobs, budgets, and emergency funds. Every dollar counts.',
        };

        return `
            <div class="transition-overlay" style="--stage-color: ${newTheme.color}">
                <div class="transition-content">
                    <div class="transition-completed">
                        <div class="transition-check">✅</div>
                        <h3>${oldName} Complete</h3>
                    </div>
                    <div class="transition-divider"></div>
                    <div class="transition-upcoming">
                        <div class="transition-emoji">${newTheme.emoji}</div>
                        <h2>Welcome to ${newName}</h2>
                        <p class="transition-hint">${stageHints[newStage] || 'A new chapter begins...'}</p>
                    </div>
                    <div class="transition-stats">
                        <div class="transition-stat"><span>💰</span> $${state.money}</div>
                        <div class="transition-stat"><span>😰</span> ${state.stress}</div>
                        <div class="transition-stat"><span>🤝</span> ${state.support}</div>
                        <div class="transition-stat"><span>📚</span> ${state.credentials.length} creds</div>
                    </div>
                    <button class="btn btn-primary transition-btn" onclick="Game.continueAfterTransition()">
                        Continue →
                    </button>
                </div>
            </div>
        `;
    },

    // ─── Feedback Item ──────────────────────────────────
    feedbackItem(msg) {
        let cls = 'neutral';
        if (msg.includes('⚠️')) cls = 'warning';
        else if (msg.includes('-$') || msg.includes('Stress +') || msg.includes('Support -')) cls = 'negative';
        else if (msg.includes('+') || msg.includes('Earned') || msg.includes('✅')) cls = 'positive';
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
