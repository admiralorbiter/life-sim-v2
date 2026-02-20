// debug.js — Dev tools panel controller

const DebugPanel = {
    toggle() {
        const body = document.getElementById('debug-body');
        body.style.display = body.style.display === 'none' ? 'block' : 'none';
    },

    async skipStage() {
        const result = await API.debugSkipStage();
        if (result.state) {
            Game.currentState = result.state;
            Components.updateStats(result.state, null);
            // Reload the current phase to reflect the new stage
            Game.phase = 'plan';
            Game.resetSelections();
            await Game.loadPhase();
            Components.showToast(result.message || 'Skipped to next stage', 'info');
        } else {
            Components.showToast(result.error || 'Failed', 'warning');
        }
    },

    async autoPlayTurn() {
        // Auto-select first available action, first decision, first event option
        const phaseData = await API.getPhaseData();
        if (phaseData.isGameOver) {
            Components.showToast('Game is already over!', 'warning');
            return;
        }

        const actions = phaseData.actions || [];
        const actionIds = actions.length > 0 ? [actions[0].id] : [];

        const choices = {
            actionIds: actionIds,
            decisionId: phaseData.decision ? phaseData.decision.id : '',
            decisionOptionIndex: 0,
            eventOptionIndex: 0,
        };

        const result = await API.submitTurn(choices);
        if (result.state) {
            Game.prevState = Game.currentState;
            Game.currentState = result.state;
            Components.updateStats(result.state, Game.prevState);

            const turnResult = result.turnResult || {};
            const feedback = turnResult.feedback || [];
            const turnNum = (result.state.currentTurn || 1) - 1;
            Game.turnLog.push({
                turn: turnNum,
                stage: result.state.currentStage,
                feedback: feedback,
            });
            Game.renderTurnLog();

            if (result.isGameOver) {
                Game.phase = 'gameover';
                Game.renderGameOver();
                Components.showToast('Game over!', 'info');
                return;
            }

            // If stage transitioned, show it
            if (turnResult.stageTransitioned && turnResult.oldStage && turnResult.newStage) {
                Game.renderStageTransition(turnResult.oldStage, turnResult.newStage);
                Components.showToast(`Auto-played T${turnNum} → Stage transition!`, 'info');
            } else {
                Game.phase = 'plan';
                Game.resetSelections();
                await Game.loadPhase();
                Components.showToast(`Auto-played T${turnNum}: ${feedback[0] || 'done'}`, 'info');
            }
        }
    },

    async setStats() {
        const stats = {};
        const fields = [
            { id: 'dbg-money', key: 'money' },
            { id: 'dbg-stress', key: 'stress' },
            { id: 'dbg-support', key: 'support' },
            { id: 'dbg-bills', key: 'monthlyBills' },
            { id: 'dbg-efund', key: 'emergencyFund' },
            { id: 'dbg-turn', key: 'turn' },
        ];

        for (const f of fields) {
            const el = document.getElementById(f.id);
            if (el && el.value !== '') {
                stats[f.key] = parseInt(el.value, 10);
            }
        }

        if (Object.keys(stats).length === 0) {
            Components.showToast('No values entered', 'warning');
            return;
        }

        const result = await API.debugSetStats(stats);
        if (result.state) {
            Game.currentState = result.state;
            Components.updateStats(result.state, null);
            Components.showToast('Stats updated!', 'info');
            // Clear inputs
            fields.forEach(f => document.getElementById(f.id).value = '');
        }
    },

    async grantTag() {
        const tag = document.getElementById('dbg-tag').value;
        const result = await API.debugGrantTag(tag);
        if (result.state) {
            Game.currentState = result.state;
            Components.updateStats(result.state, null);
            Components.showToast(result.message || `Granted: ${tag}`, 'info');
        }
    },
};
